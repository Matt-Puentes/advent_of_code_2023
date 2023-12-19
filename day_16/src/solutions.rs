use core::fmt;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    N = 0,
    S = 1,
    E = 2,
    W = 3,
}

#[derive(Clone, Debug)]
struct Point(usize, usize);
// impl Point {
//     fn add(self, rhs: (isize, isize)) -> Option<Self> {
//         let l = if rhs.0.is_negative() {
//             if rhs.0.unsigned_abs() > self.0 {
//                 None
//             } else {
//                 Some(self.0 - rhs.0.unsigned_abs())
//             }
//         } else {
//             Some(self.0 + rhs.0.unsigned_abs())
//         };
//         let c = if rhs.1.is_negative() {
//             if rhs.1.unsigned_abs() > self.1 {
//                 None
//             } else {
//                 Some(self.1 - rhs.1.unsigned_abs())
//             }
//         } else {
//             Some(self.1 + rhs.1.unsigned_abs())
//         };

//         let (Some(l), Some(c)) = (l, c) else {
//             return None
//         };
//         Some(Point(l, c))
//     }
// }

impl Point {
    fn add(self, rhs: Dir) -> Option<Self> {
        let r = match rhs {
            Dir::N => (self.0.checked_sub(1), Some(self.1)),
            Dir::S => (self.0.checked_add(1), Some(self.1)),
            Dir::E => (Some(self.0), self.1.checked_add(1)),
            Dir::W => (Some(self.0), self.1.checked_sub(1)),
        };
        if let (Some(l), Some(c)) = r {
            Some(Point(l, c))
        } else {
            None
        }
    }
}

pub fn pt_1(str_input: &str) {
    let map: Vec<Vec<Tile>> = str_input
        .lines()
        .map(|s| s.chars().map(|s| s.into()).collect())
        .collect();
    use Tile::*;

    let h = map.len();
    let w = map[0].len();

    let mut energy_map: Vec<Vec<Option<Dir>>> = vec![vec![None; w]; h];
    let mut lasers: Vec<(Point, Dir)> = vec![(Point(0, 0), Dir::E)];
    // While there's lasers
    loop {
        let Some((mut laser, mut laser_dir)) = lasers.pop() else {
            break;
        };
        // println!("Simulating laser at {:?} facing {:?}", laser, laser_dir);
        'simulation: loop {
            let Point(l, c) = laser;
            energy_map[l][c] = Some(laser_dir);
            // println!("Energizing {},{} facing {:?}", l, c, laser_dir);

            // Non-empty tile (or OOB?) hit
            let (new_dir, new_laser): (Dir, Option<Dir>) = match (&map[l][c], &laser_dir) {
                (Empty, d) => (*d, None),
                (Hsplit, Dir::N) | (Hsplit, Dir::S) => (Dir::W, Some(Dir::E)),
                (Hsplit, d) => (*d, None),
                (Vsplit, Dir::E) | (Vsplit, Dir::W) => (Dir::N, Some(Dir::S)),
                (Vsplit, d) => (*d, None),
                (Fmirror, d) => (
                    match d {
                        Dir::N => Dir::E,
                        Dir::S => Dir::W,
                        Dir::E => Dir::N,
                        Dir::W => Dir::S,
                    },
                    None,
                ),
                (Bmirror, d) => (
                    match d {
                        Dir::N => Dir::W,
                        Dir::S => Dir::E,
                        Dir::E => Dir::S,
                        Dir::W => Dir::N,
                    },
                    None,
                ),
            };

            laser_dir = new_dir;
            if let Some(nl) = new_laser {
                // println!("Adding laser at {:?} facing {:?}", laser, nl);
                lasers.push((laser.clone(), nl))
            }

            // Move forward
            match laser.add(new_dir) {
                Some(nl) => laser = nl,
                None => break 'simulation,
            }
            if laser.0 >= h || laser.1 >= w {
                break 'simulation;
            }

            // Check if this has happened before
            if let Some(d) = energy_map[laser.0][laser.1] {
                if d == laser_dir {
                    break 'simulation;
                }
            }
        }
    }

    let sum: usize = energy_map
        .iter()
        .map(|l| l.iter().filter(|s| s.is_some()).count())
        .sum();

    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    let map: Vec<Vec<Tile>> = str_input
        .lines()
        .map(|s| s.chars().map(|s| s.into()).collect())
        .collect();
    use Tile::*;

    let h = map.len();
    let w = map[0].len();

    let mut largest: usize = 0;
    let mut starting_positions: Vec<(Point, Dir)> = (0..w).map(|c| (Point(0, c), Dir::S)).collect();
    starting_positions.extend((0..h).map(|l| (Point(l, 0), Dir::E)));
    starting_positions.extend((0..w).map(|c| (Point(h - 1, c), Dir::N)));
    starting_positions.extend((0..h).map(|l| (Point(l, w - 1), Dir::W)));

    type Record = (Vec<(Point, Dir)>, Vec<(Point, Dir)>);
    // 3 dimensions (line, col, direction)
    let mut memoize_map: Vec<Vec<Vec<Option<Record>>>> = vec![vec![vec![None; 4]; w]; h];

    loop {
        let Some(st) = starting_positions.pop() else {
            break
        };

        let mut energy_map: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; w]; h];
        let mut lasers: Vec<(Point, Dir)> = vec![st];

        // While there's lasers
        'lasers: loop {
            let Some((mut laser, mut laser_dir)) = lasers.pop() else {
                break 'lasers
            };

            if let Some((path, new_lasers)) = &memoize_map[laser.0][laser.1][laser_dir as usize] {
                // Memoized solution
                for (Point(l, c), d) in path {
                    energy_map[*l][*c][*d as usize] = true
                }
                for (new_laser, new_laser_dir) in new_lasers {
                    if !energy_map[new_laser.0][new_laser.1][*new_laser_dir as usize] {
                        lasers.push((new_laser.clone(), *new_laser_dir));
                    }
                }
                continue 'lasers;
            }

            let mut path: Vec<(Point, Dir)> = vec![];
            let mut new_lasers: Vec<(Point, Dir)> = vec![];
            let og_start = laser.clone();
            let og_dir = laser_dir;
            'simulation: loop {
                let Point(l, c) = laser;
                energy_map[l][c][laser_dir as usize] = true;
                path.push((Point(l, c), laser_dir));

                let (new_dir, new_laser) = match (&map[l][c], &laser_dir) {
                    (Empty, d) => (*d, None),
                    (Hsplit, Dir::N) | (Hsplit, Dir::S) => (Dir::W, Some(Dir::E)),
                    (Hsplit, d) => (*d, None),
                    (Vsplit, Dir::E) | (Vsplit, Dir::W) => (Dir::N, Some(Dir::S)),
                    (Vsplit, d) => (*d, None),
                    (Fmirror, Dir::N) => (Dir::E, None),
                    (Fmirror, Dir::S) => (Dir::W, None),
                    (Fmirror, Dir::E) => (Dir::N, None),
                    (Fmirror, Dir::W) => (Dir::S, None),
                    (Bmirror, Dir::N) => (Dir::W, None),
                    (Bmirror, Dir::S) => (Dir::E, None),
                    (Bmirror, Dir::E) => (Dir::S, None),
                    (Bmirror, Dir::W) => (Dir::N, None),
                };

                // This laser dies, "splits" into two
                if let Some(nl) = new_laser {
                    // If the new laser has already been simulated, skip
                    if !energy_map[laser.0][laser.1][nl as usize] {
                        lasers.push((laser.clone(), nl));
                    }
                    new_lasers.push((laser.clone(), nl));

                    if !energy_map[laser.0][laser.1][new_dir as usize] {
                        lasers.push((laser.clone(), new_dir));
                    }
                    new_lasers.push((laser.clone(), new_dir));
                    break 'simulation;
                }

                laser_dir = new_dir;

                // Move forward
                match laser.add(new_dir) {
                    Some(nl) => laser = nl,
                    None => break 'simulation,
                }
                if laser.0 >= h || laser.1 >= w {
                    break 'simulation;
                }

                // Check if this has happened before
                if energy_map[laser.0][laser.1][laser_dir as usize] {
                    break 'simulation;
                }
            }
            memoize_map[og_start.0][og_start.1][og_dir as usize] = Some((path, new_lasers));
        }
        largest = largest.max(
            energy_map
                .iter()
                .map(|l| l.iter().filter(|s| s.iter().any(|s| *s)).count())
                .sum(),
        );
    }

    println!("Part 2 result: {}", largest)
}
