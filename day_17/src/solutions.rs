use pathfinding::prelude::astar;
use shared::{
    grid::Grid,
    grid::{Dir, Pos},
    Solution,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(Pos, Option<(Dir, u8)>);

impl Point {
    fn successors<'a>(
        &self,
        map: &'a Grid<u8>,
        min: u8,
        max: u8,
    ) -> impl Iterator<Item = (Point, u16)> + 'a {
        // min of 0 max of 2
        let &Point(p, d) = self;

        // Forward, right, left.

        // Direction moved, how much moved, cumulative how much moved
        let neighbors: Vec<(Dir, u8, u8)> = match d {
            // Go in any direction
            None => vec![
                (Dir::U, min, min),
                (Dir::D, min, min),
                (Dir::R, min, min),
                (Dir::L, min, min),
            ],
            // minimum not reached, go straight
            Some((d, c)) if c < min => vec![(d, min, c + min)],
            // maximum reached, turn left or right
            Some((d, c)) if c == max => vec![(d.rotr(), min, c + min), (d.rotl(), min, c + min)],
            // Go straight or turn left or right
            Some((d, c)) => vec![
                (d, min, c + min),
                (d.rotr(), min, c + min),
                (d.rotl(), min, c + min),
            ],
        };

        // Now we have the range function in map so i can sum up distances

        neighbors.into_iter().filter_map(move |(dir, moved, c)| {
            map.neighbor(&p, &dir)
                .map(|p| (Point(p, Some((dir, c))), map[p] as u16))
        })
    }
}

#[allow(dead_code)]
fn print_map(path: &[Point], h: usize, w: usize) {
    for l in 0..h {
        'char: for c in 0..w {
            for Point(Pos(pl, pc), d) in path {
                if *pl == l && *pc == c {
                    match d {
                        Some((Dir::U, ..)) => print!("^"),
                        Some((Dir::D, ..)) => print!("v"),
                        Some((Dir::R, ..)) => print!(">"),
                        Some((Dir::L, ..)) => print!("<"),
                        None => print!("."),
                    };
                    continue 'char;
                }
            }
            print!(".")
        }
        println!()
    }
}

pub fn pt_1(str_input: &str) -> Solution {
    let map: Grid<u8> = Grid::map_from(str_input, |c| c.to_digit(10).unwrap() as u8);
    let h = map.height;
    let w = map.width;

    let goal = Pos(h - 1, w - 1);
    // let goal = Pos(h, w);
    let Some((_path, cost)) = astar(
        &Point(Pos(0, 0), None),
        |p| p.successors(&map, 0, 2),
        |p| map[p.0] as u16 + p.0.distance(&goal) as u16,
        |p| p.0 == goal, //(h - 1) && p.0 .1 == (w - 1),
    ) else {
        panic!("No path found!")
    };
    // print_map(&_path, map.height, map.width);
    // 1001
    (cost as usize).into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let map: Grid<u8> = Grid::map_from(str_input, |c| c.to_digit(10).unwrap() as u8);
    let h = map.height;
    let w = map.width;

    let goal = Pos(h - 1, w - 1);
    // let goal = Pos(h, w);
    let Some((_path, cost)) = astar(
        &Point(Pos(0, 0), None),
        |p| p.successors(&map, 3, 9),
        |p| map[p.0] as u16 + p.0.distance(&goal) as u16,
        |p| {
            p.0 == goal
                && match p.1 {
                    Some((_, c)) => c >= 3,
                    _ => false,
                }
        },
    ) else {
        panic!("No path found!")
    };

    //1197
    (cost as usize).into()
}
