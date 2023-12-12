use std::vec;

pub fn pt_1(str_input: &str) {
    let lines: Vec<(Vec<_>, Vec<usize>)> = str_input
        .lines()
        .map(|l| {
            let Some((s, n)) = l.split_once(' ') else {panic!("a")};
            let mut last_char = '0';
            let mut len = 0;
            let mut spr_vec: Vec<(char, usize)> = s.chars().fold(vec![], |mut acc, ch| {
                if last_char != '0' {
                    if ch == last_char {
                        len += 1
                    } else {
                        acc.push((last_char, len));
                        len = 1
                    }
                } else {
                    len += 1
                }
                last_char = ch;
                acc
            });
            spr_vec.push((last_char, len));
            (
                spr_vec,
                n.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect::<Vec<_>>();

    for (springs, nums) in lines {
        println!("Spring line: {springs:?}");
        println!("Spring info: {nums:?}");

        fn recurse_find(
            springs: Vec<char>,
            // springs: Vec<(char, usize)>,
            nums: &[usize],
            mut curr_sol: Vec<char>,
        ) -> usize {
            //Option<Vec<(char, usize)>> {
            if nums.is_empty() {
                if springs.iter().any(|c| *c == '#') {
                    println!("err: extra # spots not taken")
                }
                println!("Match found!");
                println!("Solution: {}", curr_sol.into_iter().collect::<String>());
                return 1;
            }
            let Some((num, rest)) = nums.split_first() else { panic!() };

            let mut solutions = 0;

            let mut len = *num;
            let mut solution_start: Option<usize> = None;
            let mut q_skip = 0;
            let mut s_idx = 0;
            loop {
                // does it fit?
                let Some((c, rest_springs)) = springs[s_idx..].split_first() else {panic!("a")};
                if *c != '.' {
                    println!("trying to fit {num}. into {c}*{s}-{q_skip}");
                }

                let matched = match c {
                    '#' => {
                        // Match must be exact
                        for i in 0..len {}
                        println!(" Not exact match, {s} vs {len}");
                        0
                    }
                    // dots can never match springs
                    '.' => 0,
                    '?' if s - q_skip == 0 => {
                        println!(" ? is 0 due to q_skip.");
                        0
                    }
                    '?' => match len.cmp(&(s - q_skip)) {
                        std::cmp::Ordering::Less => {
                            println!(" Fits inside ?, Taking {} + 1.", len);
                            len + 1
                        }
                        std::cmp::Ordering::Equal => {
                            println!(
                                " Fits exactly inside ?, Taking {}. TODO check for . after",
                                len
                            );
                            len
                        }
                        std::cmp::Ordering::Greater => {
                            println!(" ? Only fits {}.", s - q_skip);
                            *s - q_skip
                        }
                    },
                    _ => unreachable!(),
                };
                if matched != 0 {
                    curr_sol.resize(curr_sol.len() + matched, '#');
                    if matched < len {
                        // There is now a "partial match". Save your spot.
                        if solution_start.is_none() {
                            println!(" Setting solution start to {s_idx}");
                            solution_start = Some(s_idx);
                        }
                        len -= matched
                    } else {
                        // The entire len was matched
                        q_skip = 0;
                        len = 0
                    }

                    if len == 0 {
                        // change vec and pass down
                        let mut new_springs; // = springs.split_first().unwrap().1;
                        if *s == matched {
                            new_springs = rest_springs.to_vec();
                        } else {
                            new_springs = vec![(*c, s - matched)];
                            new_springs.extend_from_slice(rest_springs);
                        }
                        solutions += recurse_find(new_springs, rest, curr_sol.clone());

                        // Now that we've found a match, we want to go back to the first time we started "consuming" the spring
                        if let Some(ss) = solution_start {
                            q_skip += 1; // skip 1 char of ?s
                            len = *num;
                            s_idx = ss;
                            println!(
                                " Backing up to #{s_idx}: {:?} and len {len} with q_skip {q_skip}",
                                springs[ss]
                            );
                            solution_start = None;
                            continue; // skips increment of s_index
                        } else {
                            len = *num
                        }
                    } else {
                        // If it didn't fit, continue and try the next one
                        q_skip = 0
                    }
                } else {
                    curr_sol.push('.')
                }

                s_idx += 1;
                if s_idx == springs.len() {
                    //done iterating
                    break;
                }
            }

            println!(" Backing up");
            // We've checked all branches, return the # of matches we found
            solutions
        }
        let res = recurse_find(springs, &nums[..], vec![]);
        println!("# of Results: {res:?}");

        // let mut num_idx = 0;
        // let mut spring_lens: Vec<usize> = springs.iter().map(|s| s.1).collect();
        // let mut springs_in_spot: Vec<usize> = vec![0; springs.len()];
        // let success = loop {
        //     let num = nums[num_idx];
        //     println!("Looking for a spot for #{}: {num}", num_idx + 1);
        //     let mut idx = 0;

        //     // 'springcheck: for (idx, (c, max_len)) in springs.iter().enumerate() {
        //     let found = 'springcheck: loop {
        //         let Some((c, _max_len)) = springs.get(idx) else {
        //             break 'springcheck false;
        //         };
        //         let pprint_str = c.to_string().repeat(spring_lens[idx]);
        //         // Add the number of springs in a spot to account for the need for a . between every spring
        //         if *c == '#' {
        //             match num.cmp(&spring_lens[idx]) {
        //                 std::cmp::Ordering::Less => {
        //                     println!("{num} too small to fit in #{idx}: '{}'", pprint_str)
        //                 }
        //                 std::cmp::Ordering::Greater => {
        //                     println!("{num} too big to fit in #{idx}: '{}'", pprint_str)
        //                 }
        //                 std::cmp::Ordering::Equal => {
        //                     println!("{num} can exactly fit in #{idx}: '{}'", pprint_str);
        //                     println!("Reducing to {}", spring_lens[idx] - num);
        //                     spring_lens[idx] -= num;
        //                     springs_in_spot[idx] += 1;
        //                     break 'springcheck true;
        //                 }
        //             }
        //         }
        //         if *c == '?' {
        //             if (num + springs_in_spot[idx]) <= spring_lens[idx] {
        //                 println!(
        //                     "{num}+{} can fit in #{idx}: '{}'",
        //                     springs_in_spot[idx], pprint_str
        //                 );
        //                 println!(
        //                     "Reducing to {}",
        //                     spring_lens[idx] - (num + springs_in_spot[idx])
        //                 );
        //                 spring_lens[idx] -= num + springs_in_spot[idx];
        //                 springs_in_spot[idx] += 1;
        //                 break 'springcheck true;
        //             } else {
        //                 println!("{num} cannot fit in #{idx}: '{}'", pprint_str);
        //             }
        //         }
        //         idx += 1
        //     };
        //     if !found {
        //         println!("ERR: Could not find a spot.");
        //     }

        //     // Iterate
        //     num_idx += 1;
        //     if num_idx == nums.len() {
        //         // If we reach the end, we're done!
        //         break true;
        //     }
        // };

        // if success {
        //     let mut sum = 0;
        //     for (idx, (st, _max_len)) in springs.iter().enumerate() {
        //         if *st == '?' {
        //             sum += spring_lens[idx]
        //         }
        //     }
        //     println!("Done! There was {} spots of wiggle room", sum);
        // } else {
        //     println!("ERR: Failed.");
        // }
    }

    println!("Part 1 result: {}", str_input.len())
}

pub fn pt_2(str_input: &str) {
    println!("Part 2 result: {}", str_input.len())
}
