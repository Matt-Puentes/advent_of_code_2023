use memoize::memoize;
use std::vec;

pub fn pt_1(str_input: &str) {
    let lines: Vec<(Vec<_>, Vec<usize>)> = str_input
        .lines()
        .map(|l| {
            let Some((s, n)) = l.split_once(' ') else { panic!() };
            (
                s.chars().collect(),
                n.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect::<Vec<_>>();

    let sum: usize = lines
        .iter()
        .enumerate()
        .map(|(i, (s, n))| recurse_find(s, n, 0, 0, i))
        .sum();

    // 7191
    println!("Part 1 result: {}", sum)
}

#[memoize(Ignore: springs, Ignore: nums)]
fn recurse_find(
    springs: &[char],
    nums: &[usize],
    mut s_idx: usize,
    n_idx: usize,
    graph_num: usize,
) -> usize {
    if n_idx == nums.len() {
        // check if there's any unmatched springs left: 0 if true 1 if false
        return !springs.iter().skip(s_idx).any(|s| *s == '#') as usize;
    }
    let Some(num) = nums.get(n_idx) else {panic!("nums len {} n idx {}", nums.len(), n_idx)};

    let mut solutions = 0;

    loop {
        // println!("Finding Match for {} on {:?}", num, &springs[s_idx..]);
        let mut match_question = false;
        let matched = match springs.get(s_idx) {
            // There's no more characters left, no possible solution from here.
            None => break,
            // dots can never match springs, don't match
            Some('.') => 0,
            Some(c) if *c == '#' || *c == '?' => {
                // check that there are enough chars and they're all '#' or '?'
                let res =
                    if (1..*num).all(|i| matches!(springs.get(s_idx + i), Some('#') | Some('?'))) {
                        // 2) springs can't be adjacent, so also check that there is a '.' after.
                        match springs.get(s_idx + num) {
                            // The next char would be a dot or the end of the string, which is fine.
                            None | Some('.') => *num,
                            // There is a '?' char, meaning that '?' must be a '.', so consume it.
                            Some('?') => *num + 1,
                            // There is a '#' char, so the #s weren't fully consumed. Not valid.
                            Some('#') if *c == '?' => 0,
                            Some('#') if *c == '#' => break,
                            Some(c) => panic!("Unreachable char: {}", c),
                        }
                    } else if *c == '?' {
                        // You can "skip" unknowns, treat as a dot.
                        0
                    } else {
                        // You can't "skip" springs, so early exit.
                        break;
                    };
                if *c == '?' && res > 0 {
                    match_question = true
                }
                res
            }
            Some(c) => panic!("Unreachable char: {}", c),
        };

        if matched > 0 {
            s_idx += matched;

            solutions += recurse_find(springs, nums, s_idx, n_idx + 1, graph_num);

            if !match_question {
                break;
            }
            s_idx -= matched;
        }
        s_idx += 1
    }

    // We've checked all branches, return the # of matches we found
    solutions
}

pub fn pt_2(str_input: &str) {
    let lines: Vec<(Vec<_>, Vec<usize>)> = str_input
        .lines()
        .map(|l| {
            let Some((s, n)) = l.split_once(' ') else {panic!("a")};
            let repeat = s.to_string() + "?";
            (
                repeat
                    .chars()
                    .cycle()
                    .take(((s.len() + 1) * 5) - 1)
                    .collect(),
                n.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .cycle()
                    .take(n.split(',').count() * 5)
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, (springs, nums)) in lines.iter().enumerate() {
        let res = recurse_find(springs, nums, 0, 0, i);
        sum += res;
    }

    // 6512849198636
    println!("Part 2 result: {}", sum)
}
