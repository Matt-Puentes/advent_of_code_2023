// use std::{collections::HashMap, iter::repeat};

pub fn pt_1(str_input: &str) {
    let w = str_input.find('\n').unwrap();
    let h = str_input.lines().count();
    let mut map: Vec<Vec<usize>> = vec![vec![0; w]; h];
    let mut round_rocks: Vec<(usize, usize)> = vec![];
    str_input.lines().enumerate().for_each(|(l, ls)| {
        ls.chars().enumerate().for_each(|(c, char)| match char {
            '#' => map[l][c] = 1,
            'O' => round_rocks.push((l, c)),
            _ => (),
        })
    });

    let mut top_row = vec![0; w];
    let mut sum = 0;
    'rockfall: for (l, c) in round_rocks.iter_mut() {
        for li in (0..*l).rev() {
            match map[li][*c] {
                n if n > 0 => {
                    // There is a rock here, with n rocks above it
                    // increase the load on the rock
                    map[li][*c] += 1;
                    // Move the rock to the appropriate place
                    *l = li + n;
                    sum += h - (li + n);
                    continue 'rockfall;
                }
                _ => (),
            }
        }
        // Rock hit the top
        *l = top_row[*c];
        sum += h - top_row[*c];
        top_row[*c] += 1;
    }

    println!("Part 1 result: {}", sum)
}

fn print_map(dir: &str, map: &[Vec<usize>], round_rocks: &Vec<(usize, usize)>) {
    println!("===={dir}====",);
    let mut rock_count = 0;
    for (l, line) in map.iter().enumerate() {
        for (c, col) in line.iter().enumerate() {
            if round_rocks.contains(&(l, c)) {
                rock_count += 1;
                print!("O")
            } else if *col > 0 {
                print!("{}", col)
            } else {
                print!(".")
            }
        }
        println!()
    }
    if rock_count != round_rocks.len() {
        panic!("Only {} rocks instead of {}", rock_count, round_rocks.len())
    }
}

pub fn pt_2(str_input: &str) {
    let w = str_input.find('\n').unwrap();
    let h = str_input.lines().count();
    let mut map: Vec<Vec<usize>> = vec![vec![0; w]; h];
    let mut round_rocks: Vec<(usize, usize)> = vec![];
    let mut square_rocks: Vec<(usize, usize)> = vec![];
    str_input.lines().enumerate().for_each(|(l, ls)| {
        ls.chars().enumerate().for_each(|(c, char)| match char {
            '#' => {
                square_rocks.push((l, c));
                map[l][c] = 1
            }
            'O' => round_rocks.push((l, c)),
            _ => (),
        })
    });

    let mut solve_idx = 0;
    let mut prev_rounds = vec![];
    'bigloop: for i in 0..1_000_000_000 {
        // Fall North
        round_rocks.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));
        let mut top_row = vec![0; w];
        'fallNorth: for (l, c) in round_rocks.iter_mut() {
            for li in (0..*l).rev() {
                match map[li][*c] {
                    n if n > 0 => {
                        map[li][*c] += 1;
                        *l = li + n;
                        continue 'fallNorth;
                    }
                    _ => (),
                }
            }
            // Rock hit the top
            *l = top_row[*c];
            top_row[*c] += 1;
        }
        //reset map
        for (l, c) in square_rocks.iter() {
            map[*l][*c] = 1;
        }

        // Fall West
        round_rocks.sort_by(|(_, c1), (_, c2)| c1.cmp(c2));
        let mut left_row = vec![0; h];
        'fallWest: for (l, c) in round_rocks.iter_mut() {
            for ci in (0..*c).rev() {
                match map[*l][ci] {
                    n if n > 0 => {
                        map[*l][ci] += 1;
                        *c = ci + n;
                        continue 'fallWest;
                    }
                    _ => (),
                }
            }
            // Rock hit the top
            *c = left_row[*l];
            left_row[*l] += 1;
        }
        //reset map
        for (l, c) in square_rocks.iter() {
            map[*l][*c] = 1;
        }

        // Fall South
        round_rocks.sort_by(|(l1, _), (l2, _)| l2.cmp(l1));
        let mut bottom_row = vec![1; w];
        'fallSouth: for (l, c) in round_rocks.iter_mut() {
            for li in *l + 1..h {
                match map[li][*c] {
                    n if n > 0 => {
                        map[li][*c] += 1;
                        *l = li - n;
                        continue 'fallSouth;
                    }
                    _ => (),
                }
            }
            // Rock hit the bottom
            *l = h - bottom_row[*c];
            bottom_row[*c] += 1;
        }
        //reset map
        for (l, c) in square_rocks.iter() {
            map[*l][*c] = 1;
        }

        // Fall East
        round_rocks.sort_by(|(_, c1), (_, c2)| c1.cmp(c2));
        let mut right_row = vec![1; h];
        'fallWest: for (l, c) in round_rocks.iter_mut() {
            for ci in *c + 1..w {
                match map[*l][ci] {
                    n if n > 0 => {
                        map[*l][ci] += 1;
                        *c = ci - n;
                        continue 'fallWest;
                    }
                    _ => (),
                }
            }
            // Rock hit the top
            *c = w - right_row[*l];
            right_row[*l] += 1;
        }
        //reset map
        for (l, c) in square_rocks.iter() {
            map[*l][*c] = 1;
        }

        for (i2, prev_round) in prev_rounds.iter().enumerate() {
            if prev_round == &round_rocks {
                println!("Cycle {i}=={i2}");
                let rounds_left_to_complete = (1_000_000_000 - 1) - i;
                let remainder = rounds_left_to_complete % (i - i2);
                solve_idx = i2 + remainder;
                break 'bigloop;
            }
        }
        prev_rounds.push(round_rocks.clone());
    }
    // print_map("Done", &map, &round_rocks);

    let sum: usize = prev_rounds[solve_idx].iter().map(|(l, _)| h - *l).sum();
    println!("Part 2 result: {}", sum)
}
