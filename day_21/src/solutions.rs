use std::collections::VecDeque;

use shared::grid::{Grid, Pos};
use shared::Solution;

#[derive(Debug, PartialEq)]
enum Garden {
    Start,
    Plot,
    Rock,
}

impl std::convert::From<char> for Garden {
    fn from(value: char) -> Self {
        match value {
            'S' => Garden::Start,
            '.' => Garden::Plot,
            '#' => Garden::Rock,
            _ => panic!("Invalid garden tile"),
        }
    }
}

impl std::fmt::Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Garden::Start => write!(f, "S"),
            Garden::Plot => write!(f, "."),
            Garden::Rock => write!(f, "#"),
        }
    }
}

pub fn pt_1(str_input: &str) -> Solution {
    let grid: Grid<Garden> = Grid::from(str_input);
    let (startpos, _) = grid.find(&Garden::Start).expect("Start present");

    simulate(&grid, startpos, 64).into()
}

fn simulate(grid: &Grid<Garden>, start_pos: Pos, max: usize) -> usize {
    let mut queue: VecDeque<(Pos, usize)> = VecDeque::from([(start_pos, 0)]);

    // true if visited on an even step, false otherwise.
    let mut visited_grid: Grid<Option<bool>> = Grid::new_default(grid.height, grid.width);
    while let Some((spot, dist)) = queue.pop_front() {
        if visited_grid[spot].is_some() {
            continue;
        }
        visited_grid[spot] = Some(dist % 2 == 0);
        if dist == max {
            continue;
        }

        for p in grid
            .neighbors_iter(spot)
            .filter(|p| grid[*p] != Garden::Rock && visited_grid[*p].is_none())
        {
            queue.push_back((p, dist + 1))
        }
    }
    if max % 2 == 0 {
        visited_grid.iter().filter(|v| v.unwrap_or(false)).count()
    } else {
        visited_grid.iter().filter(|v| !(v.unwrap_or(true))).count()
    }
}

pub fn pt_2(str_input: &str) -> Solution {
    let grid: Grid<Garden> = Grid::from(str_input);
    let middle = (grid.width - 1) / 2;

    // I got a LOT of help from this video by HyperNeutrino
    // https://www.youtube.com/watch?v=9UOMZSL0JTg

    let w = grid.width;
    let h = grid.height;
    let size = grid.width;

    let steps = 26501365;
    let n = steps / size - 1;

    let odd_grids = (n / 2 * 2 + 1) * (n / 2 * 2 + 1);
    let even_grids = ((n + 1) / 2 * 2) * ((n + 1) / 2 * 2);

    // Calculate every possible way the grid could be covered
    let odd_fill = simulate(&grid, Pos(middle, middle), size * 2 + 1);
    let even_fill = simulate(&grid, Pos(middle, middle), size * 2);

    let corner_t = simulate(&grid, Pos(0, middle), size - 1);
    let corner_b = simulate(&grid, Pos(h - 1, middle), size - 1);
    let corner_l = simulate(&grid, Pos(middle, 0), size - 1);
    let corner_r = simulate(&grid, Pos(middle, w - 1), size - 1);

    let small_num = n + 1;
    let small_tr = simulate(&grid, Pos(0, w - 1), size / 2 - 1);
    let small_br = simulate(&grid, Pos(h - 1, w - 1), size / 2 - 1);
    let small_tl = simulate(&grid, Pos(0, 0), size / 2 - 1);
    let small_bl = simulate(&grid, Pos(h - 1, 0), size / 2 - 1);

    let big_num = n;
    let big_tr = simulate(&grid, Pos(0, w - 1), size * 3 / 2 - 1);
    let big_br = simulate(&grid, Pos(h - 1, w - 1), size * 3 / 2 - 1);
    let big_tl = simulate(&grid, Pos(0, 0), size * 3 / 2 - 1);
    let big_bl = simulate(&grid, Pos(h - 1, 0), size * 3 / 2 - 1);

    let final_answer = (odd_grids * odd_fill)
        + (even_grids * even_fill)
        + (corner_t + corner_b + corner_l + corner_r)
        + (small_num * (small_tr + small_br + small_tl + small_bl))
        + (big_num * (big_tr + big_br + big_tl + big_bl));

    final_answer.into()
}
