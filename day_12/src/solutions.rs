use memoize::memoize;
use std::vec;

pub fn pt_1(str_input: &str) {
    let lines: Vec<(Vec<_>, Vec<usize>)> = str_input
        .lines()
        .map(|l| {
            let Some((s, n)) = l.split_once(' ') else {panic!("a")};
            (
                s.chars().collect(),
                n.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (springs, nums) in lines {
        fn recurse_find(springs: Vec<char>, nums: &[usize], mut curr_sol: Vec<char>) -> usize {
            if nums.is_empty() {
                if springs.iter().any(|s| *s == '#') {
                    return 0;
                } else {
                    return 1;
                };
                // println!("Match found!");
                // curr_sol.resize(curr_sol.len() + springs.len(), '.');
                // println!("Solution: {}", curr_sol.into_iter().collect::<String>());
            }
            let Some((num, rest)) = nums.split_first() else { panic!() };

            let mut solutions = 0;

            let mut s_idx = 0;

            'stringchecker: loop {
                let mut assume_period = false;
                let mut match_question = false;
                let matched = match springs.get(s_idx) {
                    // There's no more characters left, no possible solution from here.
                    None => break 'stringchecker,
                    // dots can never match springs, don't match
                    Some('.') => 0,
                    Some('#') => {
                        // 1) there are enough chars, and they're all '#' or '?'
                        // 2) springs can't be adjacent, so there has to be a '.' after.
                        if (1..*num)
                            .all(|i| matches!(springs.get(s_idx + i), Some('#') | Some('?')))
                        {
                            // println!(
                            //     "Potential exact match for size {num} on spring {:?}, next is {:?}",
                            //     &springs.get(s_idx..(s_idx + num)),
                            //     springs.get(s_idx..)
                            // );

                            match springs.get(s_idx + num) {
                                // The next char would be a dot or the end of the string, which is fine.
                                None | Some('.') => *num,
                                // There is a '?' char, meaning that '?' must be a '.', so consume it.
                                Some('?') => {
                                    assume_period = true;
                                    *num + 1
                                }
                                // There is a '#' char, so the #s weren't fully consumed. Not valid.
                                Some('#') => break 'stringchecker,
                                Some(c) => panic!("Unreachable char: {}", c),
                            }
                        } else {
                            // println!(
                            //     "Couldn't fit size {num} in {:?}. Invalid!",
                            //     &springs.get(s_idx..(s_idx + num))
                            // );

                            // You can't "skip" springs, so early exit.
                            break 'stringchecker;
                        }
                    }
                    // Match must be exact
                    Some('?') => {
                        // 1) there are enough chars, and they're all '#' or '?'
                        // 2) springs can't be adjacent, so there has to be a '.' after.
                        if (1..*num)
                            .all(|i| matches!(springs.get(s_idx + i), Some('#') | Some('?')))
                        {
                            // println!(
                            //     "Potential match for size {num} on spring {:?}, next is {:?}",
                            //     &springs.get(s_idx..(s_idx + num)),
                            //     springs.get(s_idx..)
                            // );

                            match springs.get(s_idx + num) {
                                // The next char would be a dot or the end of the string, which is fine.
                                None | Some('.') => {
                                    match_question = true;
                                    *num
                                }
                                // There is a '?' char, meaning that '?' must be a '.', so consume it.
                                Some('?') => {
                                    match_question = true;
                                    assume_period = true;
                                    *num + 1
                                }
                                // There is a '#' char, so the #s weren't fully consumed. Not valid. Skip.
                                Some('#') => 0,
                                Some(c) => panic!("Unreachable char: {}", c),
                            }
                        } else {
                            // println!(
                            //     "Couldn't fit size {num} in {:?}. Moving on.",
                            //     &springs.get(s_idx..(s_idx + num))
                            // );
                            // You can "skip" unknowns, treat as a dot.
                            0
                        }
                    }
                    Some(c) => panic!("Unreachable char: {}", c),
                };

                if matched == 0 {
                    // No match, continue searching
                    // println!("No match for size {num} on spring {:?}", &springs[s_idx..]);
                    s_idx += 1;
                    curr_sol.push('.');
                    continue;
                } else {
                    let mut new_sol = curr_sol.clone();
                    if assume_period {
                        // println!(
                        //     "match for size {num} on spring {:?}",
                        //     &springs.get(s_idx..(s_idx + matched - 1))
                        // );
                        new_sol.resize(new_sol.len() + matched - 1, '#');
                        new_sol.push('.')
                    } else {
                        // println!(
                        //     "match for size {num} on spring {:?}",
                        //     &springs.get(s_idx..(s_idx + matched))
                        // );
                        new_sol.resize(new_sol.len() + matched, '#');
                    }

                    let old_s_idx = s_idx;
                    // there was some sort of match
                    s_idx += matched;

                    // change vec and pass "down"
                    let new_springs = springs[s_idx..].to_vec();
                    if new_springs.is_empty() && !rest.is_empty() {
                        // There cant be any solutions
                        continue;
                    }
                    solutions += recurse_find(new_springs, rest, new_sol.clone());
                    curr_sol.push('.');

                    if match_question {
                        s_idx = old_s_idx + 1
                    } else {
                        break;
                    }
                    // if assume_period {
                    //     s_idx -= 1;
                    // }
                }
            }

            // println!(" Backing up");
            // We've checked all branches, return the # of matches we found
            solutions
        }
        let res = recurse_find(springs, &nums[..], vec![]);
        println!("# of Results: {res:?}");
        sum += res;
    }

    println!("Part 1 result: {}", sum)
}

#[memoize]
fn recurse_find<'a>(springs: Vec<char>, nums: Vec<usize>) -> usize {
    if nums.is_empty() {
        // check if there's any unmatched springs left: 0 if true 1 if false
        return !springs.iter().any(|s| *s == '#') as usize;
    }
    let Some((num, rest)) = nums.split_first() else { panic!() };

    let mut solutions = 0;

    let mut s_idx = 0;

    'stringchecker: loop {
        let mut match_question = false;
        let matched = match springs.get(s_idx) {
            // There's no more characters left, no possible solution from here.
            None => break 'stringchecker,
            // dots can never match springs, don't match
            Some('.') => 0,
            Some('#') => {
                // 1) there are enough chars, and they're all '#' or '?'
                if (1..*num).all(|i| matches!(springs.get(s_idx + i), Some('#') | Some('?'))) {
                    // 2) springs can't be adjacent, so there has to be a '.' after.
                    match springs.get(s_idx + num) {
                        // The next char would be a dot or the end of the string, which is fine.
                        None | Some('.') => *num,
                        // There is a '?' char, meaning that '?' must be a '.', so consume it.
                        Some('?') => *num + 1,
                        // There is a '#' char, so the #s weren't fully consumed. Not valid.
                        Some('#') => break 'stringchecker,
                        Some(c) => panic!("Unreachable char: {}", c),
                    }
                } else {
                    // You can't "skip" springs, so early exit.
                    break 'stringchecker;
                }
            }
            // Match must be exact
            Some('?') => {
                // 1) there are enough chars, and they're all '#' or '?'
                let res =
                    if (1..*num).all(|i| matches!(springs.get(s_idx + i), Some('#') | Some('?'))) {
                        // 2) springs can't be adjacent, so there has to be a '.' after.
                        match springs.get(s_idx + num) {
                            // The next char would be a dot or the end of the string, which is fine.
                            None | Some('.') => *num,
                            // There is a '?' char, meaning that '?' must be a '.', so consume it.
                            Some('?') => *num + 1,
                            // There is a '#' char, so the #s weren't fully consumed. Not valid. Skip.
                            Some('#') => 0,
                            Some(c) => panic!("Unreachable char: {}", c),
                        }
                    } else {
                        // You can "skip" unknowns, treat as a dot.
                        0
                    };
                if res > 0 {
                    match_question = true
                }
                res
            }
            Some(c) => panic!("Unreachable char: {}", c),
        };

        if matched == 0 {
            // No match, continue searching
            s_idx += 1;
        } else {
            let old_s_idx = s_idx;
            s_idx += matched;

            // change vec and pass "down"
            let new_springs = springs[s_idx..].to_vec();
            if new_springs.is_empty() && !rest.is_empty() {
                // There cant be any solutions
            } else {
                solutions += recurse_find(new_springs, rest.iter().cloned().collect());

                if match_question {
                    s_idx = old_s_idx + 1
                } else {
                    break;
                }
            }
        }
    }

    // println!(" Backing up");
    // We've checked all branches, return the # of matches we found
    solutions
}

pub fn pt_2(str_input: &str) {
    let lines: Vec<(Vec<_>, Vec<usize>)> = str_input
        .lines()
        .map(|l| {
            let Some((s, n)) = l.split_once(' ') else {panic!("a")};
            let string = s.chars().collect::<Vec<_>>();
            let counts = n.split(',').map(|s| s.parse::<usize>().unwrap());
            let mut big_string = vec![];
            for i in 0..5 {
                big_string.extend(string.clone());
                println!("{i}");
                if i != 4 {
                    big_string.push('?');
                }
            }
            (
                big_string,
                counts.cycle().take(n.split(',').count() * 5).collect(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (springs, nums) in lines {
        println!("Springs: {springs:?}");
        println!("nums: {nums:?}");

        let res = recurse_find(springs, nums);
        println!("# of Results: {res:?}");
        sum += res;
    }

    println!("Part 2 result: {}", sum)
}
