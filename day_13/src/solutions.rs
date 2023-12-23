use shared::{grid::Grid, Solution};
pub fn pt_1(str_input: &str) -> Solution {
    let maps: Vec<Grid<char>> = str_input.split("\n\n").map(Grid::<char>::from).collect();

    let mut sum = 0;
    for map in maps {
        let h = map.height;
        let w = map.width;

        // Try flipping horizontally
        for i in 1..w {
            let s = i.min(w - i);
            if (i - s..i)
                .zip((i..i + s).rev())
                .all(|(ir, il)| (0..h).all(|x| map[(x, ir)] == map[(x, il)]))
            {
                sum += i;
                continue;
            }
        }

        // Try flipping vertically
        for i in 1..h {
            let s = i.min(h - i);
            if (i - s..i)
                .zip((i..i + s).rev())
                .all(|(it, ib)| (0..w).all(|x| map[(it, x)] == map[(ib, x)]))
            {
                sum += i * 100;
            }
        }
    }

    // 33728
    sum.into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let maps: Vec<Grid<char>> = str_input.split("\n\n").map(Grid::<char>::from).collect();

    let mut sum = 0;
    for map in maps {
        let h = map.height;
        let w = map.width;

        // Try flipping horizontally
        for i in 1..w {
            let s = i.min(w - i);

            let mistakes: usize = (i - s..i)
                .zip((i..i + s).rev())
                .map(|(ir, il)| {
                    (0..h)
                        .map(|x| (map[(x, ir)] != map[(x, il)]) as usize)
                        .sum::<usize>()
                })
                .sum();
            if mistakes == 1 {
                sum += i;
                continue;
            }
        }

        // Try flipping vertically
        for i in 1..h {
            let s = i.min(h - i);
            let mistakes: usize = (i - s..i)
                .zip((i..i + s).rev())
                .map(|(it, ib)| {
                    (0..w)
                        .map(|x| (map[(it, x)] != map[(ib, x)]) as usize)
                        .sum::<usize>()
                })
                .sum();
            if mistakes == 1 {
                sum += i * 100;
            }
        }
    }

    // 28235
    sum.into()
}
