use shared::Solution;

#[allow(clippy::needless_range_loop)]
pub fn pt_1(str_input: &str) -> Solution {
    let mut map: Vec<Vec<char>> = str_input.lines().map(|l| l.chars().collect()).collect();
    let h = map.len();
    let w = map[0].len();

    let mut sum = 0;
    let mut roundcount;
    for col in 0..w {
        roundcount = 0;
        for line in (0..h).rev() {
            match map[line][col] {
                'O' => {
                    roundcount += 1;
                    map[line][col] = '.'
                }
                '#' => {
                    for r in 1..=roundcount {
                        map[line + r][col] = 'O';
                        sum += h - (line + r)
                    }
                    roundcount = 0;
                }
                _ => (),
            }
        }
        for r in 0..roundcount {
            map[r][col] = 'O';
            sum += h - r
        }
    }
    sum.into()
}

#[allow(dead_code, unused_variables)]
fn print_map(dir: &str, map: &[Vec<char>]) {
    println!("===={dir}====",);
    for (l, line) in map.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            print!("{char}")
        }
        println!()
    }
}

#[allow(clippy::needless_range_loop)]
pub fn pt_2(str_input: &str) -> Solution {
    let mut previous_maps: Vec<Vec<Vec<char>>> = vec![];
    let mut map: Vec<Vec<char>> = str_input.lines().map(|l| l.chars().collect()).collect();
    let h = map.len();
    let w = map[0].len();

    let mut solve_idx = 0;

    'bigloop: for i in 0..1_000_000_000 {
        let mut roundcount;
        for col in 0..w {
            roundcount = 0;
            for line in (0..h).rev() {
                match map[line][col] {
                    'O' => {
                        roundcount += 1;
                        map[line][col] = '.'
                    }
                    '#' => {
                        for r in 1..=roundcount {
                            map[line + r][col] = 'O';
                        }
                        roundcount = 0;
                    }
                    _ => (),
                }
            }
            for r in 0..roundcount {
                map[r][col] = 'O';
            }
        }

        for line in 0..h {
            roundcount = 0;
            for col in (0..w).rev() {
                match map[line][col] {
                    'O' => {
                        roundcount += 1;
                        map[line][col] = '.'
                    }
                    '#' => {
                        for r in 1..=roundcount {
                            map[line][col + r] = 'O';
                        }
                        roundcount = 0;
                    }
                    _ => (),
                }
            }
            for r in 0..roundcount {
                map[line][r] = 'O';
            }
        }

        for col in 0..w {
            roundcount = 0;
            for line in 0..h {
                match map[line][col] {
                    'O' => {
                        roundcount += 1;
                        map[line][col] = '.'
                    }
                    '#' => {
                        for r in 1..=roundcount {
                            map[line - r][col] = 'O';
                        }
                        roundcount = 0;
                    }
                    _ => (),
                }
            }
            for r in 1..=roundcount {
                map[h - r][col] = 'O';
            }
        }

        for line in 0..h {
            roundcount = 0;
            for col in 0..w {
                match map[line][col] {
                    'O' => {
                        roundcount += 1;
                        map[line][col] = '.'
                    }
                    '#' => {
                        for r in 1..=roundcount {
                            map[line][col - r] = 'O';
                        }
                        roundcount = 0;
                    }
                    _ => (),
                }
            }
            for r in 1..=roundcount {
                map[line][w - r] = 'O';
            }
        }

        for (i2, prev_round) in previous_maps.iter().enumerate() {
            if prev_round == &map {
                let rounds_left_to_complete = (1_000_000_000 - 1) - i;
                let remainder = rounds_left_to_complete % (i - i2);
                solve_idx = i2 + remainder;
                break 'bigloop;
            }
        }
        previous_maps.push(map.clone());
    }

    let mut sum = 0;
    for line in 0..h {
        for col in 0..w {
            if previous_maps[solve_idx][line][col] == 'O' {
                sum += h - line;
            }
        }
    }
    // 101292
    sum.into()
}
