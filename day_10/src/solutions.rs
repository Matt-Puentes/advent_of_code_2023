use shared::{
    grid::{Grid, Pos},
    Solution,
};
static PD: [char; 4] = ['|', '7', 'F', 'S'];
static PU: [char; 4] = ['|', 'J', 'L', 'S'];
static PL: [char; 4] = ['-', 'J', '7', 'S'];
static PR: [char; 4] = ['-', 'L', 'F', 'S'];
static ORDER: [([char; 4], [char; 4]); 4] = [(PU, PD), (PD, PU), (PL, PR), (PR, PL)];

pub fn pt_1(str_input: &str) -> Solution {
    let map: Grid<char> = Grid::from(str_input);
    let start_idx = map.find(&'S').unwrap().0;

    let mut vm: Grid<bool> = Grid::new_default(map.height, map.width);
    let mut queue: Vec<(Pos, usize)> = vec![(start_idx, 0)];
    let mut highest: usize = 0;

    while let Some((p, dist)) = queue.pop() {
        let char = &map[p];
        highest = highest.max(dist);
        vm[p] = true;

        // assuming up, down, left, right
        let neighbors = map.all_neighbors(p);
        for (i, neighbor) in neighbors.into_iter().enumerate() {
            if let Some(n) = neighbor {
                if ORDER[i].0.contains(char) && ORDER[i].1.contains(&map[n]) && !vm[n] {
                    queue.push((n, dist + 1))
                }
            }
        }
    }

    ((highest + 1) / 2).into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let map: Grid<char> = Grid::from(str_input);
    let start = map.find(&'S').unwrap().0;
    let mut vm = Grid::new('.', map.height, map.width);
    let mut queue: Vec<Pos> = vec![start];

    while let Some(p) = queue.pop() {
        let ch = &map[p];
        vm[p] = *ch;

        // assuming up, down, left, right
        let neighbors = map.all_neighbors(p);
        for (i, neighbor) in neighbors.into_iter().enumerate() {
            if let Some(n) = neighbor {
                if ORDER[i].0.contains(ch) && ORDER[i].1.contains(&map[n]) && vm[n] == '.' {
                    queue.push(n)
                }
            }
        }
    }

    // Count points "inside" pipes in vm by edge counting
    let mut sum = 0;
    let (mut counting, mut above, mut below);
    for line in vm.lines() {
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
