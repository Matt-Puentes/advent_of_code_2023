use shared::Solution;
static PD: [char; 4] = ['|', '7', 'F', 'S'];
static PU: [char; 4] = ['|', 'J', 'L', 'S'];
static PL: [char; 4] = ['-', 'J', '7', 'S'];
static PR: [char; 4] = ['-', 'L', 'F', 'S'];

pub fn pt_1(str_input: &str) -> Solution {
    let mut start_idx: (usize, usize) = (0, 0);
    let map: Vec<Vec<char>> = str_input.lines().map(|l| l.chars().collect()).collect();
    'out: for (l, line) in map.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_idx = (l, c);
                break 'out;
            }
        }
    }

    // Height
    let h = map.len();
    // Width
    let w = map[0].len();
    // Visited map
    let mut vm = vec![vec![false; w]; h];

    let mut queue: Vec<((usize, usize), usize)> = vec![(start_idx, 0)];
    let mut highest: usize = 0;
    while let Some(((l, c), d)) = queue.pop() {
        let char = &map[l][c];
        highest = highest.max(d);
        vm[l][c] = true;

        if PU.contains(char) && l > 0 && PD.contains(&map[l - 1][c]) && !vm[l - 1][c] {
            queue.push(((l - 1, c), d + 1))
        }
        if PD.contains(char) && l < h - 1 && PU.contains(&map[l + 1][c]) && !vm[l + 1][c] {
            queue.push(((l + 1, c), d + 1))
        }
        if PL.contains(char) && c > 0 && PR.contains(&map[l][c - 1]) && !vm[l][c - 1] {
            queue.push(((l, c - 1), d + 1))
        }
        if PR.contains(char) && c < w - 1 && PL.contains(&map[l][c + 1]) && !vm[l][c + 1] {
            queue.push(((l, c + 1), d + 1))
        }
    }

    ((highest + 1) / 2).into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let mut start: (usize, usize) = (0, 0);
    // Collect the input into a 2d matrix of chars, also search for ground
    let map: Vec<Vec<char>> = str_input.lines().map(|l| l.chars().collect()).collect();
    'out: for (l, line) in map.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == 'S' {
                start = (l, c);
                break 'out;
            }
        }
    }

    let h = map.len();
    let w = map[0].len();
    // Visited map
    let mut vm = vec![vec!['.'; w]; h];

    let mut queue: Vec<(usize, usize)> = vec![start];
    while let Some((l, c)) = queue.pop() {
        let ch = &map[l][c];
        vm[l][c] = *ch;

        // Check if the neighbor is in bounds, valid, and not already processed.
        if l > 0 && PU.contains(ch) && PD.contains(&map[l - 1][c]) && vm[l - 1][c] == '.' {
            queue.push((l - 1, c))
        }
        if l < h - 1 && PD.contains(ch) && PU.contains(&map[l + 1][c]) && vm[l + 1][c] == '.' {
            queue.push((l + 1, c))
        }
        if c > 0 && PL.contains(ch) && PR.contains(&map[l][c - 1]) && vm[l][c - 1] == '.' {
            queue.push((l, c - 1))
        }
        if c < w - 1 && PR.contains(ch) && PL.contains(&map[l][c + 1]) && vm[l][c + 1] == '.' {
            queue.push((l, c + 1))
        }
    }

    // Count points "inside" pipes in vm by edge counting
    let mut sum = 0;
    let (mut counting, mut above, mut below);
    for line in vm {
        (counting, above, below) = (false, false, false);
        for c in line {
            match c {
                '.' if counting => sum += 1,
                '|' => counting = !counting,
                'J' | 'L' if below => {
                    counting = !counting;
                    below = false
                }
                '7' | 'F' if above => {
                    counting = !counting;
                    above = false
                }
                'J' | 'L' => above = !above,
                '7' | 'F' => below = !below,
                _ => (),
            }
        }
    }
    sum.into()
}
