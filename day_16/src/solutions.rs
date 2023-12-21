use core::fmt;
use shared::grid::{Dir, Grid, Pos};
use shared::Solution;

#[derive(PartialEq)]
enum Tile {
    Empty,   //= '.
    Hsplit,  // '-'
    Vsplit,  // '|'
    Fmirror, // '/'
    Bmirror, // '\'
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Vsplit => write!(f, "|"),
            Tile::Hsplit => write!(f, "-"),
            Tile::Bmirror => write!(f, "\\"),
            Tile::Fmirror => write!(f, "/"),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '|' => Tile::Vsplit,
            '-' => Tile::Hsplit,
            '\\' => Tile::Bmirror,
            '/' => Tile::Fmirror,
            _ => panic!(),
        }
    }
}

pub fn pt_1(str_input: &str) -> Solution {
    let map: Grid<Tile> = Grid::from(str_input);
    use Tile::*;

    // let map: Grid<Tile> = Grid::from(str_input)
    // let mut energy_map: Vec<Option<Dir>> = vec![None; map.width * map.height];
    let mut energy_map: Grid<Option<Dir>> = Grid::new_default(map.height, map.width);
    let mut lasers: Vec<(Pos, Dir)> = vec![(Pos(0, 0), Dir::R)];
    // While there's lasers
    loop {
        let Some((mut laser, mut laser_dir)) = lasers.pop() else {
            break;
        };
        // println!("Simulating laser at {:?} facing {:?}", laser, laser_dir);
        'simulation: loop {
            let Pos(l, c) = laser;
            energy_map[(l, c)] = Some(laser_dir);
            // println!("Energizing {},{} facing {:?}", l, c, laser_dir);

            // Non-empty tile (or OOB?) hit
            let (new_dir, new_laser): (Dir, Option<Dir>) = match (&map[(l, c)], &laser_dir) {
                (Empty, d) => (*d, None),
                (Hsplit, Dir::U) | (Hsplit, Dir::D) => (Dir::L, Some(Dir::R)),
                (Hsplit, d) => (*d, None),
                (Vsplit, Dir::R) | (Vsplit, Dir::L) => (Dir::U, Some(Dir::D)),
                (Vsplit, d) => (*d, None),
                (Fmirror, d) => (
                    match d {
                        Dir::U => Dir::R,
                        Dir::D => Dir::L,
                        Dir::R => Dir::U,
                        Dir::L => Dir::D,
                    },
                    None,
                ),
                (Bmirror, d) => (
                    match d {
                        Dir::U => Dir::L,
                        Dir::D => Dir::R,
                        Dir::R => Dir::D,
                        Dir::L => Dir::U,
                    },
                    None,
                ),
            };

            laser_dir = new_dir;
            if let Some(nl) = new_laser {
                // println!("Adding laser at {:?} facing {:?}", laser, nl);
                lasers.push((laser, nl))
            }

            // Move forward
            if !map.shift(&mut laser, &new_dir) {
                break 'simulation;
            }

            // Check if this has happened before
            if let Some(d) = energy_map[laser] {
                if d == laser_dir {
                    break 'simulation;
                }
            }
        }
    }

    let sum: usize = energy_map.into_iter().filter(|l| l.is_some()).count();
    sum.into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let map: Grid<Tile> = str_input.into();

    use Tile::*;

    // let w = str_input.find('\n').unwrap();
    // let h = map.len() / w;

    let mut largest: usize = 0;
    let mut starting_positions: Vec<(Pos, Dir)> =
        (0..map.width).map(|c| (Pos(0, c), Dir::D)).collect();
    starting_positions.extend((0..map.height).map(|l| (Pos(l, 0), Dir::R)));
    starting_positions.extend((0..map.width).map(|c| (Pos(map.height - 1, c), Dir::U)));
    starting_positions.extend((0..map.height).map(|l| (Pos(l, map.width - 1), Dir::L)));

    type Record = (Vec<(Pos, Dir)>, Vec<(Pos, Dir)>);
    // 3 dimensions (line, col, direction)
    let mut memoize_map: Grid<Vec<Option<Record>>> =
        Grid::new(vec![None; 4], map.height, map.width);

    loop {
        let Some(st) = starting_positions.pop() else {
            break;
        };

        let mut energy_map: Grid<[bool; 4]> = Grid::new([false; 4], map.height, map.width);
        let mut lasers: Vec<(Pos, Dir)> = vec![st];

        // While there's lasers
        'lasers: loop {
            let Some((mut laser, mut laser_dir)) = lasers.pop() else {
                break 'lasers;
            };

            if let Some((path, new_lasers)) = &memoize_map[(laser.0, laser.1)][laser_dir as usize] {
                // Memoized solution
                for (Pos(l, c), d) in path {
                    energy_map[(*l, *c)][*d as usize] = true
                }
                for (new_laser, new_laser_dir) in new_lasers {
                    if !energy_map[*new_laser][*new_laser_dir as usize] {
                        lasers.push((*new_laser, *new_laser_dir));
                    }
                }
                continue 'lasers;
            }

            let mut path: Vec<(Pos, Dir)> = vec![];
            let mut new_lasers: Vec<(Pos, Dir)> = vec![];
            let og_start = laser;
            let og_dir = laser_dir;
            'simulation: loop {
                let Pos(l, c) = laser;
                energy_map[(l, c)][laser_dir as usize] = true;
                path.push((Pos(l, c), laser_dir));

                let (new_dir, new_laser) = match (&map[(l, c)], &laser_dir) {
                    (Empty, d) => (*d, None),
                    (Hsplit, Dir::U) | (Hsplit, Dir::D) => (Dir::R, Some(Dir::L)),
                    (Hsplit, d) => (*d, None),
                    (Vsplit, Dir::L) | (Vsplit, Dir::R) => (Dir::U, Some(Dir::D)),
                    (Vsplit, d) => (*d, None),
                    (Fmirror, Dir::U) => (Dir::R, None),
                    (Fmirror, Dir::D) => (Dir::L, None),
                    (Fmirror, Dir::R) => (Dir::U, None),
                    (Fmirror, Dir::L) => (Dir::D, None),
                    (Bmirror, Dir::U) => (Dir::L, None),
                    (Bmirror, Dir::D) => (Dir::R, None),
                    (Bmirror, Dir::R) => (Dir::D, None),
                    (Bmirror, Dir::L) => (Dir::U, None),
                };

                // This laser dies, "splits" into two
                if let Some(nl) = new_laser {
                    // If the new laser has already been simulated, skip
                    if !energy_map[laser][nl as usize] {
                        lasers.push((laser, nl));
                    }
                    new_lasers.push((laser, nl));

                    if !energy_map[laser][new_dir as usize] {
                        lasers.push((laser, new_dir));
                    }
                    new_lasers.push((laser, new_dir));
                    break 'simulation;
                }

                laser_dir = new_dir;

                // Move forward
                if !map.shift(&mut laser, &new_dir) {
                    break 'simulation;
                }

                // Check if this has happened before
                if energy_map[laser][laser_dir as usize] {
                    break 'simulation;
                }
            }
            memoize_map[(og_start.0, og_start.1)][og_dir as usize] = Some((path, new_lasers));
        }

        let count = energy_map
            .iter()
            .filter(|l| l.iter().any(|s| *s)) //.filter(|s| s.iter().any(|s| *s)).count())
            .count();

        largest = largest.max(count);
    }

    largest.into()
}
