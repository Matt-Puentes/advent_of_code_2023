use shared::grid::Grid;
use shared::Solution;

enum Garden {
    Start,
    Plot,
    Rock,
}

impl std::convert::From<char> for Garden {
    fn from(value: char) -> Self {
        match value {
            'S' => Garden::Plot,
            '.' => Garden::Plot,
            '#' => Garden::Rock,
            _ => panic!("Invalid garden tile"),
        }
    }
}

pub fn pt_1(str_input: &str) -> Solution {
    let grid: Grid<Garden> = Grid::from(str_input);
    Solution::None
}

pub fn pt_2(str_input: &str) -> Solution {
    Solution::None
}
