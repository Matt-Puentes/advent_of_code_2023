enum RuleResult<'a> {
    Rule(&'a str),
    Accept,
    Reject,
}

impl<'a> From<&'a str> for RuleResult<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            r => RuleResult::Rule(r),
        }
    }
}

enum Rule<'a> {
    GT(usize, usize, RuleResult<'a>),
    LT(usize, usize, RuleResult<'a>),
    End(RuleResult<'a>),
}

impl<'a> Rule<'a> {
    fn check(&'a self, val: usize) -> bool {
        match self {
            Rule::GT(_, v, _) => val > *v,
            Rule::LT(_, v, _) => val < *v,
            Rule::End(_) => true,
        }
    }
}

impl<'a> Rule<'a> {
    fn new(gt: bool, idx: &'a str, num: &'a str, result: &'a str) -> Self {
        let i = match idx {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => panic!("Bad rule code"),
        };
        let n = num.parse().unwrap();
        let r = result.into();
        if gt {
            Rule::GT(i, n, r)
        } else {
            Rule::LT(i, n, r)
        }
    }
}

pub fn pt_1(str_input: &str) {
    let (rule_str, shapes) = str_input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, Vec<Rule>)> = rule_str
        .lines()
        .map(|rule| {
            let (name, vals) = rule.split_once('\u{007b}').unwrap();
            let rules = vals[..vals.len() - 1]
                .split(',')
                .map(|rule| {
                    if let Some((idx, val)) = rule.split_once('>') {
                        let (num, result) = val.split_once(':').unwrap();
                        Rule::new(true, idx, num, result)
                    } else if let Some((idx, val)) = rule.split_once('<') {
                        let (num, result) = val.split_once(':').unwrap();
                        Rule::new(false, idx, num, result)
                    } else {
                        Rule::End(rule.into())
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();

    let mut sum = 0;
    'shape: for shape_str in shapes.lines() {
        // Parse shape
        let shape: [usize; 4] = shape_str[1..shape_str.len() - 1]
            .split(',')
            .map(|val| val[2..].parse().unwrap())
            .collect::<Vec<_>>()[..]
            .try_into()
            .expect("4 values in a rule");

        // Run shape through rules until accepted or rejected
        let mut curr_rule = rules.iter().find(|r| r.0 == "in").expect("rule named in");
        loop {
            let (_, tests) = curr_rule;

            let rr = tests
                .iter()
                .find_map(|test| match test {
                    Rule::GT(i, v, r) if shape[*i] > *v => Some(r),
                    Rule::GT(..) => None,
                    Rule::LT(i, v, r) if shape[*i] < *v => Some(r),
                    Rule::LT(..) => None,
                    Rule::End(r) => Some(r),
                })
                .expect("at least one matching rule");

            match rr {
                RuleResult::Rule(r) => {
                    curr_rule = rules.iter().find(|(name, _)| name == r).expect("rule");
                }
                RuleResult::Accept => {
                    sum += shape.iter().sum::<usize>();
                    continue 'shape;
                }
                RuleResult::Reject => continue 'shape,
            }
        }
    }

    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    let (rule_str, _) = str_input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, Vec<Rule>)> = rule_str
        .lines()
        .map(|rule| {
            let (name, vals) = rule.split_once('\u{007b}').unwrap();
            let rules = vals[..vals.len() - 1]
                .split(',')
                .map(|rule| {
                    if let Some((idx, val)) = rule.split_once('>') {
                        let (num, result) = val.split_once(':').unwrap();
                        Rule::new(true, idx, num, result)
                    } else if let Some((idx, val)) = rule.split_once('<') {
                        let (num, result) = val.split_once(':').unwrap();
                        Rule::new(false, idx, num, result)
                    } else {
                        Rule::End(rule.into())
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();

    type Remain = [(usize, usize); 4];

    let mut sum: usize = 0;
    let mut process_queue: Vec<(&str, Remain)> = vec![("in", [(1, 4000); 4])];
    loop {
        let Some((name, mut remainders)) = process_queue.pop() else { break };

        let (_, rules) = rules.iter().find(|r| r.0 == name).expect("rule");

        for rule in rules {
            let (filtered, rr) = match rule {
                rule @ (Rule::LT(i, v, r) | Rule::GT(i, v, r)) => {
                    let mut filtered = remainders;
                    let (l, h) = remainders[*i];
                    // check for overlap
                    match (rule.check(l), rule.check(h)) {
                        (true, true) => {
                            // Whole range is applicable
                            remainders[*i] = (0, 0);
                            filtered[*i] = (l, h);
                        }
                        (false, true) => {
                            // Partially applicable
                            remainders[*i] = (l, *v);
                            filtered[*i] = (*v + 1, h);
                        }
                        (true, false) => {
                            // Partially applicable
                            remainders[*i] = (*v, h);
                            filtered[*i] = (l, *v - 1);
                        }
                        (false, false) => {
                            // Nothing was matched
                            remainders[*i] = (l, *v);
                            filtered[*i] = (0, 0);
                        }
                    };
                    (filtered, r)
                }
                // always fully applicable, always the last rule so we don't care about editing the remainders
                Rule::End(r) => (remainders, r),
            };

            match rr {
                // provide new ranges to child rule
                RuleResult::Rule(rn) => process_queue.push((rn, filtered)),
                // Compute the number of shapes in the remaining ranges, accept them all
                RuleResult::Accept => {
                    sum += filtered
                        .iter()
                        .map(|(l, h)| ((h - l) + 1))
                        .product::<usize>()
                }
                // Ignore these shapes
                RuleResult::Reject => (),
            }

            // Check if any of the remainders are of size 0- if so, it can't possibly add to the # of accepted shapes,
            //  so we can break early.
            if remainders.iter().any(|(l, h)| l == h) {
                break;
            }
        }
    }

    println!("Part 2 result: {}", sum)
}
