// .....
// .S-7.
// .|.|.
// .L-J.
// .....

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

// use std::fmt::{Debug, Display};

// enum Pipe {
//     Up,
//     Side,
//     NE,
//     NW,
//     SW,
//     SE,
//     Ground,
//     Start,
// }

// impl Display for Pipe {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Pipe::Up => write!(f, "|"),
//             Pipe::Side => write!(f, "-"),
//             Pipe::NE => write!(f, "└"),
//             Pipe::NW => write!(f, "┘"),
//             Pipe::SW => write!(f, "┐"),
//             Pipe::SE => write!(f, "┌"),
//             Pipe::Ground => write!(f, "."),
//             Pipe::Start => write!(f, "s"),
//         }
//     }
// }

// impl From<char> for Pipe {
//     fn from(value: char) -> Self {
//         match value {
//             '|' => Pipe::Up,
//             '-' => Pipe::Side,
//             'L' => Pipe::NE,
//             'J' => Pipe::NW,
//             '7' => Pipe::SW,
//             'F' => Pipe::SE,
//             '.' => Pipe::Ground,
//             'S' => Pipe::Start,
//             c => panic!("Unknown character '{}'", c),
//         }
//     }
// }

// let map_string = map
//     .iter()
//     .enumerate()
//     .map(|(l, s)| {
//         s.iter()
//             .enumerate()
//             .map(|(i, c)| if vm[l][i] { c } else { &'.' })
//             .collect::<String>()
//     })
//     .collect::<Vec<_>>()
//     .join("\n");
// println!("Map:\n{map_string}");

static UP: char = '|';
static SIDE: char = '-';
static NE: char = 'L';
static NW: char = 'J';
static SW: char = '7';
static SE: char = 'F';
static START: char = 'S';
// static GROUND: char = '.';
static PDOWN: [char; 4] = [UP, SW, SE, START];
static PUP: [char; 4] = [UP, NW, NE, START];
static PLEFT: [char; 4] = [SIDE, NW, SW, START];
static PRIGHT: [char; 4] = [SIDE, NE, SE, START];

pub fn pt_1(str_input: &str) {
    let mut start_idx: (usize, usize) = (0, 0);
    // Collect the input into a 2d matrix of chars, also search for 'S'
    let map: Vec<Vec<char>> = str_input.lines().map(|l| l.chars().collect()).collect();
    for (l, line) in map.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_idx = (l, c)
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
    loop {
        let Some(((l, c), d)) = queue.pop() else { break };
        let char = &map[l][c];
        vm[l][c] = true;
        highest = highest.max(d);

        if PUP.contains(char) && l > 0 && PDOWN.contains(&map[l - 1][c]) && !vm[l - 1][c] {
            queue.push(((l - 1, c), d + 1))
        }
        if PDOWN.contains(char) && l < h - 1 && PUP.contains(&map[l + 1][c]) && !vm[l + 1][c] {
            queue.push(((l + 1, c), d + 1))
        }
        if PLEFT.contains(char) && c > 0 && PRIGHT.contains(&map[l][c - 1]) && !vm[l][c - 1] {
            queue.push(((l, c - 1), d + 1))
        }
        if PRIGHT.contains(char) && c < w - 1 && PLEFT.contains(&map[l][c + 1]) && !vm[l][c + 1] {
            queue.push(((l, c + 1), d + 1))
        }
    }

    println!("Part 1 result: {}", (highest + 1) / 2)
}

pub fn pt_2(str_input: &str) {
    let mut start_idx: (usize, usize) = (0, 0);
    // Collect the input into a 2d matrix of chars, also search for 'S'
    let map: Vec<Vec<char>> = str_input
        .lines()
        .enumerate()
        .map(|(l, s)| {
            s.chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == 'S' {
                        start_idx = (l, i)
                    };
                    c
                })
                .collect()
        })
        .collect();

    // Height
    let h = map.len();
    // Width
    let w = map[0].len();
    // Visited map
    // let mut vm = vec![vec![false; w]; h];

    let mut queue: Vec<Vec<(usize, usize)>> = vec![vec![start_idx]];
    let loop_idxs: Vec<(usize, usize)> = loop {
        let vec = queue.pop().expect("Queue should not be empty"); // else { break };
        let (l, c) = vec.last().unwrap();
        let ch = &map[*l][*c];
        // if vec.len() > 1 && *ch == START {
        //     // println!("{vec:?}");
        //     // panic!("Found loop!");
        // };

        // println!("{vec:?}");
        if PUP.contains(ch) && *l > 0 && PDOWN.contains(&map[*l - 1][*c]) {
            if vec.len() > 2 && map[*l - 1][*c] == START {
                // loop found
                break vec;
            }
            if !vec.contains(&(l - 1, *c)) {
                let mut newvec = vec.clone();
                newvec.push((*l - 1, *c));
                queue.push(newvec)
            }
        }
        if PDOWN.contains(ch) && *l < h - 1 && PUP.contains(&map[*l + 1][*c]) {
            if vec.len() > 2 && map[*l + 1][*c] == START {
                // loop found
                break vec;
            }
            if !vec.contains(&(l + 1, *c)) {
                let mut newvec = vec.clone();
                newvec.push((*l + 1, *c));
                queue.push(newvec)
            }
        }
        if PLEFT.contains(ch) && *c > 0 && PRIGHT.contains(&map[*l][*c - 1]) {
            if vec.len() > 2 && map[*l][*c - 1] == START {
                // loop found
                break vec;
            }
            if !vec.contains(&(*l, c - 1)) {
                let mut newvec = vec.clone();
                newvec.push((*l, *c - 1));
                queue.push(newvec)
            }
        }
        if PRIGHT.contains(ch) && *c < w - 1 && PLEFT.contains(&map[*l][*c + 1]) {
            if vec.len() > 2 && map[*l][*c + 1] == START {
                // loop found
                break vec;
            }
            if !vec.contains(&(*l, c + 1)) {
                let mut newvec = vec.clone();
                newvec.push((*l, *c + 1));
                queue.push(newvec)
            }
        }
    };
    println!("Loop: {}", loop_idxs.len());
    // println!("Loop: {loop_idxs:?}");

    println!("Part 2 result: {}", str_input.len())
}
