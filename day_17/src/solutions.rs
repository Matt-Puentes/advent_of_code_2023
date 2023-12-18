#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    N(u8),
    S(u8),
    E(u8),
    W(u8),
}

impl Dir {
    fn val(&self) -> usize {
        match self {
            Dir::N(_) => 0,
            Dir::S(_) => 1,
            Dir::E(_) => 2,
            Dir::W(_) => 3,
        }
    }
}

pub fn pt_1(str_input: &str) {
    let map: Vec<Vec<u8>> = str_input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let h = map.len();
    let w = map[0].len();

    // Set of discovered nodes with path info
    let mut open: Vec<(usize, usize, Option<Dir>)> = vec![(0, 0, None)];
    // for each node, the node preceeding it on the cheapest path
    let mut cheapest: Vec<Vec<Vec<Option<(usize, usize, Dir)>>>> = vec![vec![vec![None; 4]; w]; h];
    // the cost of the cheapest path from start to point
    let mut score: Vec<Vec<usize>> = vec![vec![10000000000000000000; w]; h];
    score[0][0] = 0;
    // the estimated cost of the cheapest path from start to end through this point
    let mut full_score: Vec<Vec<usize>> = vec![vec![10000000000000000000; w]; h];
    full_score[0][0] = map[0][0] as usize + h + w;

    let goaldir = loop {
        open.sort_by(|(l1, c1, _), (l2, c2, _)| full_score[*l2][*c2].cmp(&full_score[*l1][*c1]));
        let Some((l, c, dir)) = open.pop() else {
            panic!("Out of nodes but goal hasn't been found");
        };
        // println!("Node ({}, {}) facing {:?}", l, c, dir);
        if l == h - 1 && c == w - 1 {
            // we did it reddit
            println!("We reached the goal, score of {}", score[l][c]);
            break dir.unwrap();
        }

        // determine neighbors
        let neighbors = match dir {
            Some(Dir::N(2)) => vec![Dir::E(0), Dir::W(0)],
            Some(Dir::N(c)) => vec![Dir::N(c + 1), Dir::E(0), Dir::W(0)],
            Some(Dir::S(2)) => vec![Dir::E(0), Dir::W(0)],
            Some(Dir::S(c)) => vec![Dir::S(c + 1), Dir::E(0), Dir::W(0)],
            Some(Dir::E(2)) => vec![Dir::N(0), Dir::S(0)],
            Some(Dir::E(c)) => vec![Dir::N(0), Dir::S(0), Dir::E(c + 1)],
            Some(Dir::W(2)) => vec![Dir::N(0), Dir::S(0)],
            Some(Dir::W(c)) => vec![Dir::N(0), Dir::S(0), Dir::W(c + 1)],
            None => vec![Dir::N(0), Dir::S(0), Dir::E(0), Dir::W(0)],
        };

        println!("Neighbors for {l},{c},{dir:?}: {neighbors:?}");
        for neighbor in neighbors {
            let (nl, nc) = match neighbor {
                Dir::N(_) if l != 0 => (l - 1, c),
                Dir::S(_) if l != h - 1 => (l + 1, c),
                Dir::E(_) if c != 0 => (l, c - 1),
                Dir::W(_) if c != w - 1 => (l, c + 1),
                _ => continue,
            };

            let tentative_score = score[l][c] + (map[nl][nc] as usize);
            if tentative_score < score[nl][nc] {
                cheapest[nl][nc][neighbor.val()] = Some((l, c, neighbor.clone()));
                score[nl][nc] = tentative_score;
                full_score[nl][nc] = tentative_score + (map[nl][nc] as usize) + (h - nl) + (w - nc);
                let entry = (nl, nc, Some(neighbor));
                if !open.contains(&entry) {
                    open.push(entry)
                }
            }
        }
    };

    let mut path = vec![(h - 1, w - 1, goaldir.clone())];
    let mut cursor = (h - 1, w - 1, goaldir);
    let mut sum = map[cursor.0][cursor.1] as usize;
    loop {
        let Some(cheapest) = cheapest[cursor.0][cursor.1][cursor.2.val()] else {
            break;
            // panic!("While reconstructing, {cursor:?} never recorded a previous node")
        };
        cursor = cheapest;
        path.push(cheapest);
        // println!("point {:?} cost {}", cursor, map[cursor.0][cursor.1]);

        sum += map[cursor.0][cursor.1] as usize;
    }
    println!("Path: {:?}", path);

    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    println!("Part 2 result: {}", str_input.len())
}
