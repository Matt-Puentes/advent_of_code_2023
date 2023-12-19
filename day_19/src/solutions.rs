#[derive(Debug)]
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

#[derive(Debug)]
enum Rule<'a> {
    // Index, val, result
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

pub fn pt_1(str_input: &str) {
    let (rule_str, shape_strs) = str_input.split_once("\n\n").unwrap();
    let shapes: Vec<[usize; 4]> = shape_strs
        .lines()
        .map(|shape| {
            // let &[x, m, a, s]: &[usize; 4] =
            shape[1..shape.len() - 1]
                .split(',')
                .map(|val| val[2..].parse().unwrap())
                .collect::<Vec<_>>()[..]
                .try_into()
                .expect("4 values in a rule")

            // (x, m, a, s)
        })
        .collect();

    let rules: Vec<(&str, Vec<Rule>)> = rule_str
        .lines()
        .map(|rule| {
            //
            let (name, vals) = rule.split_once('\u{007b}').unwrap();
            (
                name,
                vals[..vals.len() - 1]
                    .split(',')
                    .map(|rule| {
                        if let Some((idx, val)) = rule.split_once('>') {
                            let (num, result) = val.split_once(':').unwrap();
                            Rule::GT(
                                match idx {
                                    "x" => 0,
                                    "m" => 1,
                                    "a" => 2,
                                    "s" => 3,
                                    _ => panic!("Bad rule code"),
                                },
                                num.parse().unwrap(),
                                result.into(),
                            )
                        } else if let Some((idx, val)) = rule.split_once('<') {
                            let (num, result) = val.split_once(':').unwrap();
                            Rule::LT(
                                match idx {
                                    "x" => 0,
                                    "m" => 1,
                                    "a" => 2,
                                    "s" => 3,
                                    _ => panic!("Bad rule code"),
                                },
                                num.parse().unwrap(),
                                result.into(),
                            )
                        } else {
                            Rule::End(rule.into())
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    let mut sum = 0;
    'shape: for shape in shapes {
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
            //
            let (name, vals) = rule.split_once('\u{007b}').unwrap();
            (
                name,
                vals[..vals.len() - 1]
                    .split(',')
                    .map(|rule| {
                        if let Some((idx, val)) = rule.split_once('>') {
                            let (num, result) = val.split_once(':').unwrap();
                            Rule::GT(
                                match idx {
                                    "x" => 0,
                                    "m" => 1,
                                    "a" => 2,
                                    "s" => 3,
                                    _ => panic!("Bad rule code"),
                                },
                                num.parse().unwrap(),
                                result.into(),
                            )
                        } else if let Some((idx, val)) = rule.split_once('<') {
                            let (num, result) = val.split_once(':').unwrap();
                            Rule::LT(
                                match idx {
                                    "x" => 0,
                                    "m" => 1,
                                    "a" => 2,
                                    "s" => 3,
                                    _ => panic!("Bad rule code"),
                                },
                                num.parse().unwrap(),
                                result.into(),
                            )
                        } else {
                            Rule::End(rule.into())
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    type Remain = [(usize, usize); 4];

    let mut process_queue: Vec<(&str, Remain)> =
        vec![("in", [(1, 4000), (1, 4000), (1, 4000), (1, 4000)])];

    let mut sum: u64 = 0;
    loop {
        let Some((name, mut remainders)) = process_queue.pop() else {
            // we're done
            break;
        };
        let (_, rules) = rules.iter().find(|r| r.0 == name).expect("rule");
        for rule in rules {
            let (filtered, rr) = match rule {
                rule @ (Rule::LT(i, v, r) | Rule::GT(i, v, r)) => {
                    // check for overlap
                    let mut filtered = remainders;
                    let (l, h) = remainders[*i];
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
                Rule::End(r) => (remainders, r),
            };

            match rr {
                RuleResult::Rule(rn) => process_queue.push((rn, filtered)),
                RuleResult::Accept => {
                    sum += filtered
                        .iter()
                        .map(|(l, h)| ((h - l) + 1) as u64)
                        .product::<u64>()
                }
                RuleResult::Reject => (),
            }

            if remainders.iter().any(|(l, h)| l == h) {
                break;
            }
        }
    }

    println!("Part 2 result: {}", sum)
}
