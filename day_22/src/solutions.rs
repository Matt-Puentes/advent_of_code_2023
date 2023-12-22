use shared::Solution;
#[derive(Debug)]
struct Brick([u16; 3], [u16; 3]); // [usize; 3]);

impl<'a> From<&'a str> for Brick {
    fn from(value: &'a str) -> Self {
        let (face1_s, face2_s) = value.split_once('~').unwrap();
        Brick(
            face1_s
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            face2_s
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Brick {
    fn xy_overlap(&self, other: ([u16; 3], [u16; 3])) -> bool {
        self.1[0] >= other.0[0]
            && self.0[0] <= other.1[0]
            && self.1[1] >= other.0[1]
            && self.0[1] <= other.1[1]
    }
}

// Got the idea to search for highest Z once (instead of calculating overlap each loop) from
// https://old.reddit.com/r/adventofcode/comments/18o7014/2023_day_22_solutions/keh2zb2/

pub fn pt_1(str_input: &str) -> Solution {
    let mut bricks: Vec<Brick> = str_input.lines().map(Brick::from).collect();
    // Sort bricks so the lowest to the ground are first
    bricks.sort_by(|f1, f2| f1.0[2].cmp(&(f2.0[2])));
    // map of block index to a tuple of:
    //  how many blocks are supporting the block and a vec of the indexes the block is supporting.
    let mut supporting: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); bricks.len()];

    // "Drop" blocks from bottom to top, tracking which blocks are supporting each other
    for b_idx in 0..bricks.len() {
        // Copy brick out of brick array so we don't borrow
        let Brick(mut p1, mut p2) = bricks[b_idx];

        // find the highest Z of the block below this block that has an x,y intersection
        let mut highest_z = 1;
        let mut highest_blocks = vec![];
        for (i, block) in bricks[..b_idx]
            .iter()
            .enumerate()
            .filter(|(_, b)| b.xy_overlap((p1, p2)))
        {
            let highest_z_above_block = &block.1[2] + 1;
            match highest_z.cmp(&highest_z_above_block) {
                std::cmp::Ordering::Less => {
                    highest_blocks.clear();
                    highest_z = highest_z_above_block;
                    highest_blocks.push(i)
                }
                std::cmp::Ordering::Equal => {
                    highest_z = highest_z_above_block;
                    highest_blocks.push(i)
                }
                std::cmp::Ordering::Greater => (),
            }
        }
        for resting_on in highest_blocks {
            supporting[resting_on].1.push(b_idx);
            supporting[b_idx].0 += 1;
        }

        // fall to highest_z
        p2[2] -= p1[2] - highest_z;
        p1[2] = highest_z;

        // Update brick value
        bricks[b_idx] = Brick(p1, p2);
    }

    let disintegrated = bricks
        .iter()
        .enumerate()
        .filter(|(idx, _)| supporting[*idx].1.iter().all(|si| supporting[*si].0 > 1))
        .count();

    // 386
    disintegrated.into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let mut bricks: Vec<Brick> = str_input.lines().map(Brick::from).collect();
    // Sort bricks so the lowest to the ground are first
    bricks.sort_by(|f1, f2| f1.0[2].cmp(&(f2.0[2])));
    // map of block index to a tuple of:
    //  how many blocks are supporting the block and a vec of the indexes the block is supporting.
    let mut supporting: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); bricks.len()];

    // "Drop" blocks from bottom to top, tracking which blocks are supporting each other
    for b_idx in 0..bricks.len() {
        // Copy brick out of brick array so we don't borrow
        let Brick(mut p1, mut p2) = bricks[b_idx];

        // find the highest Z of the block below this block that has an x,y intersection
        let mut highest_z = 1;
        let mut highest_blocks = vec![];
        for (i, block) in bricks[..b_idx]
            .iter()
            .enumerate()
            .filter(|(_, b)| b.xy_overlap((p1, p2)))
        {
            let highest_z_above_block = &block.1[2] + 1;
            match highest_z.cmp(&highest_z_above_block) {
                std::cmp::Ordering::Less => {
                    highest_blocks.clear();
                    highest_z = highest_z_above_block;
                    highest_blocks.push(i)
                }
                std::cmp::Ordering::Equal => {
                    highest_z = highest_z_above_block;
                    highest_blocks.push(i)
                }
                std::cmp::Ordering::Greater => (),
            }
        }
        for resting_on in highest_blocks {
            supporting[resting_on].1.push(b_idx);
            supporting[b_idx].0 += 1;
        }

        // fall to highest_z
        p2[2] -= p1[2] - highest_z;
        p1[2] = highest_z;

        // Update brick value
        bricks[b_idx] = Brick(p1, p2);
    }

    let mut falling = 0;
    for idx in 0..bricks.len() {
        let mut brickqueue = vec![idx];
        let mut supports_disintegrated: Vec<usize> = vec![0; bricks.len()];
        while let Some(idx) = brickqueue.pop() {
            for s in &supporting[idx].1 {
                supports_disintegrated[*s] += 1;
                if idx != *s && supporting[*s].0 <= supports_disintegrated[*s] {
                    falling += 1;
                    brickqueue.push(*s)
                }
            }
        }
    }

    //39933
    falling.into()
}
