use pathfinding::prelude::astar;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    None,
    N(u8),
    S(u8),
    E(u8),
    W(u8),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, Dir);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn successors_pt1(&self, map: &[Vec<u8>], h: usize, w: usize) -> Vec<(Pos, usize)> {
        let &Pos(l, c, dir) = self;
        let neighbors: Vec<Dir> = match &dir {
            Dir::None => vec![Dir::N(0), Dir::S(0), Dir::E(0), Dir::W(0)],
            Dir::N(2) => vec![Dir::E(0), Dir::W(0)],
            Dir::S(2) => vec![Dir::E(0), Dir::W(0)],
            Dir::E(2) => vec![Dir::N(0), Dir::S(0)],
            Dir::W(2) => vec![Dir::N(0), Dir::S(0)],
            Dir::N(c) => vec![Dir::N(c + 1), Dir::E(0), Dir::W(0)],
            Dir::S(c) => vec![Dir::S(c + 1), Dir::E(0), Dir::W(0)],
            Dir::E(c) => vec![Dir::E(c + 1), Dir::N(0), Dir::S(0)],
            Dir::W(c) => vec![Dir::W(c + 1), Dir::N(0), Dir::S(0)],
        };

        neighbors
            .iter()
            .filter_map(|dir| match dir {
                n @ Dir::N(_) if l != 0 => Some((Pos(l - 1, c, *n), map[l - 1][c] as usize)),
                n @ Dir::S(_) if l != h - 1 => Some((Pos(l + 1, c, *n), map[l + 1][c] as usize)),
                n @ Dir::E(_) if c != w - 1 => Some((Pos(l, c + 1, *n), map[l][c + 1] as usize)),
                n @ Dir::W(_) if c != 0 => Some((Pos(l, c - 1, *n), map[l][c - 1] as usize)),
                Dir::None => unreachable!(),
                _ => None,
            })
            .collect()
    }

    fn successors_pt2(&self, map: &[Vec<u8>], h: usize, w: usize) -> Vec<(Pos, usize)> {
        let &Pos(l, c, dir) = self;
        let neighbors: Vec<Dir> = match &dir {
            Dir::None => vec![Dir::N(3), Dir::S(3), Dir::E(3), Dir::W(3)],
            Dir::N(c) if *c < 3 => vec![Dir::N(3)],
            Dir::S(c) if *c < 3 => vec![Dir::S(3)],
            Dir::E(c) if *c < 3 => vec![Dir::E(3)],
            Dir::W(c) if *c < 3 => vec![Dir::W(3)],
            Dir::N(9) => vec![Dir::E(3), Dir::W(3)],
            Dir::S(9) => vec![Dir::E(3), Dir::W(3)],
            Dir::E(9) => vec![Dir::N(3), Dir::S(3)],
            Dir::W(9) => vec![Dir::N(3), Dir::S(3)],
            Dir::N(c) => vec![Dir::N(c + 1), Dir::E(3), Dir::W(3)],
            Dir::S(c) => vec![Dir::S(c + 1), Dir::E(3), Dir::W(3)],
            Dir::E(c) => vec![Dir::E(c + 1), Dir::N(3), Dir::S(3)],
            Dir::W(c) => vec![Dir::W(c + 1), Dir::N(3), Dir::S(3)],
        };

        neighbors
            .iter()
            .filter_map(|dir| match dir {
                n @ Dir::N(x) if *x == 3 && l > 4 => Some((
                    Pos(l - 4, c, *n),
                    (1..=4).map(|x| map[l - x][c] as usize).sum(),
                )),
                n @ Dir::S(x) if *x == 3 && l < h - 4 => Some((
                    Pos(l + 4, c, *n),
                    (1..=4).map(|x| map[l + x][c] as usize).sum(),
                )),
                n @ Dir::E(x) if *x == 3 && c < w - 4 => Some((
                    Pos(l, c + 4, *n),
                    (1..=4).map(|x| map[l][c + x] as usize).sum(),
                )),
                n @ Dir::W(x) if *x == 3 && c > 4 => Some((
                    Pos(l, c - 4, *n),
                    (1..=4).map(|x| map[l][c - x] as usize).sum(),
                )),
                n @ Dir::N(x) if *x > 3 && l != 0 => {
                    Some((Pos(l - 1, c, *n), map[l - 1][c] as usize))
                }
                n @ Dir::S(x) if *x > 3 && l != h - 1 => {
                    Some((Pos(l + 1, c, *n), map[l + 1][c] as usize))
                }
                n @ Dir::E(x) if *x > 3 && c != w - 1 => {
                    Some((Pos(l, c + 1, *n), map[l][c + 1] as usize))
                }
                n @ Dir::W(x) if *x > 3 && c != 0 => {
                    Some((Pos(l, c - 1, *n), map[l][c - 1] as usize))
                }
                Dir::None => unreachable!(),
                _ => None,
            })
            .collect()
    }
}

#[allow(dead_code)]
fn print_map(path: &[Pos], h: usize, w: usize) {
    for l in 0..h {
        'char: for c in 0..w {
            for Pos(pl, pc, d) in path {
                if *pl == l && *pc == c {
                    match d {
                        Dir::N(_) => print!("^"),
                        Dir::S(_) => print!("v"),
                        Dir::E(_) => print!(">"),
                        Dir::W(_) => print!("<"),
                        _ => print!("."),
                    };
                    continue 'char;
                }
            }
            print!(".")
        }
        println!()
    }
}

pub fn pt_1(str_input: &str) {
    let map: Vec<Vec<u8>> = str_input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let h = map.len();
    let w = map[0].len();

    let goal: Pos = Pos(h, w, Dir::None);
    let Some((_path, cost)) = astar(
        &Pos(0, 0, Dir::None),
        |p| p.successors_pt1(&map, h, w),
        |p| map[p.0][p.1] as usize + p.distance(&goal),
        |p| p.0 == (h - 1) && p.1 == (w - 1),
    ) else {
        panic!("No path found!")
    };

    println!("Part 1 result: {}", cost)
}

pub fn pt_2(str_input: &str) {
    let map: Vec<Vec<u8>> = str_input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let h = map.len();
    let w = map[0].len();

    let goal: Pos = Pos(h, w, Dir::None);
    let Some((_path, cost)) = astar(
        &Pos(0, 0, Dir::None),
        |p| p.successors_pt2(&map, h, w),
        |p| map[p.0][p.1] as usize + p.distance(&goal) + p.0 % 4 + p.1 % 4,
        |p| p.0 == (h - 1) && p.1 == (w - 1) && match p.2 {
            Dir::None => false,
            Dir::N(c) | Dir::S(c) | Dir::E(c) | Dir::W(c) =>  c >= 3
        },
    ) else {
        panic!("No path found!")
    };

    println!("Part 2 result: {}", cost)
}
