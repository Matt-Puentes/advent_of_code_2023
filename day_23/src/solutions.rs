use std::collections::VecDeque;

use shared::{
    grid::{Dir, Grid, Pos},
    Solution,
};

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Path,
    RRamp,
    DRamp,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Path,
            '>' => Tile::RRamp,
            'v' => Tile::DRamp,
            _ => panic!(),
        }
    }
}

pub fn pt_1(str_input: &str) -> Solution {
    let map: Grid<Tile> = str_input.into();

    // Prev spot, current spot,
    let mut queue: VecDeque<(Pos, Pos, u16)> = VecDeque::from([(Pos(0, 1), Pos(0, 1), 0)]);

    let mut walk_len = 0;
    'walks: loop {
        let Some((mut prev_spot, mut pos, mut cost)) = queue.pop_back() else {
            break 'walks;
        };

        'walk: loop {
            let [n, ns @ ..] = &map
                .neighbors_iter_with_dir(pos)
                .filter(|(d, p)| {
                    p != &prev_spot
                        && map[*p] != Tile::Wall
                        && !(d == &Dir::U && map[*p] == Tile::DRamp)
                        && !(d == &Dir::L && map[*p] == Tile::RRamp)
                })
                .collect::<Vec<_>>()[..]
            else {
                panic!("Node {} had no neighbors", pos)
            };

            cost += 1;

            if n.1 == Pos(map.height - 1, map.width - 2) {
                walk_len = walk_len.max(cost);
                break 'walk;
            }

            prev_spot = pos;
            pos = n.1;
            queue.extend(ns.iter().map(|(d, p)| (prev_spot, *p, cost)));
        }
    }

    Solution::NumUsize(walk_len as usize)
}

pub fn pt_2(str_input: &str) -> Solution {
    let map: Grid<Tile> = str_input.into();
    // let names = [
    //     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    //     's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    // ];
    // let node_names = [
    //     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    //     'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    // ];
    // let mut display_map: Grid<char> = str_input.into();

    // the # of the path that visited it
    let mut visited: Grid<Option<usize>> = Grid::new_default(map.width, map.height);

    // Parent intersection/path, current spot, prev spot
    let mut queue: Vec<(Option<usize>, Pos, Pos)> = vec![(None, Pos(0, 1), Pos(0, 1))];
    // A "path" is a sequence with no branches, an "Edge" in this graph
    let mut paths: Vec<usize> = vec![];

    // list of path intersections, a "node" in this graph. An intersection is a list of paths it connects to.
    let mut intersections: Vec<Vec<usize>> = vec![];
    let mut intersection_coords: Vec<Pos> = vec![];

    let start_path = 0;
    let mut goal_path = 0;
    'walks: loop {
        let Some((prev_intersection, mut pos, mut prev_pos)) = queue.pop() else {
            break 'walks;
        };

        if let Some(new_path) = visited[pos] {
            if let Some(intersection) = prev_intersection {
                // This path has been visited before, don't walk it, but mark the intersection it reaches
                if !intersections[intersection].contains(&new_path) {
                    intersections[intersection].push(new_path);
                }
            }
            continue;
        }
        // create new path
        paths.push(0);
        let path_idx = paths.len() - 1;
        // Add path to intersection
        if let Some(intersection) = prev_intersection {
            intersections[intersection].push(path_idx)
        }

        'walk: loop {
            match &map
                .neighbors_iter(pos)
                .filter(|p| p != &prev_pos && map[*p] != Tile::Wall)
                .collect::<Vec<_>>()[..]
            {
                [] => panic!("Found node with no neighbors at {pos}"),
                [n] => {
                    visited[pos] = Some(path_idx);
                    paths[path_idx] += 1;
                    // display_map[pos] = names[path_idx];
                    if n == &Pos(map.height - 1, map.width - 2) {
                        // We hit the end, this path has no connections
                        // display_map[*n] = names[path_idx];
                        paths[path_idx] += 1;
                        goal_path = path_idx;
                        break 'walk;
                    }
                    prev_pos = pos;
                    pos = *n;
                }
                ns => {
                    if let Some((i, _)) = intersection_coords
                        .iter()
                        .enumerate()
                        .find(|(_, p)| **p == pos)
                    {
                        // Intersection already visited, just add this path to it.
                        if !intersections[i].contains(&path_idx) {
                            intersections[i].push(path_idx);
                        }
                    } else {
                        // there's a new intersection, add it to the list and start progressing down the branches.
                        intersections.push(vec![path_idx]);
                        intersection_coords.push(pos);

                        // This point has multiple neighbors, so it doesn't
                        // "count" for any path (all paths get +1 at the end).
                        for neighbor in ns.iter() {
                            if visited[*neighbor].is_none() {
                                queue.push((Some(intersections.len() - 1), *neighbor, pos));
                            }
                        }
                    };

                    visited[pos] = Some(path_idx);
                    break 'walk;
                }
            }
        }
    }

    // A map of nodes
    let mut nodes: Vec<Vec<(usize, usize)>> = vec![vec![], vec![]];

    for (ni, node) in intersections.iter().enumerate() {
        let mut connected_nodes = vec![];
        for connected_path in node {
            let connected_node_idx: usize;
            if *connected_path == goal_path {
                connected_nodes.push((0, paths[*connected_path]));
            } else if *connected_path == start_path {
                nodes[1].push((ni + 2, paths[*connected_path]));
                connected_nodes.push((1, paths[*connected_path]));
            } else {
                // adjust for goal+start node at beginning of list
                connected_node_idx = intersections
                    .iter()
                    .enumerate()
                    .find(|(i, n)| *i != ni && n.contains(connected_path))
                    .map(|(i, n)| i)
                    .unwrap()
                    + 2;
                connected_nodes.push((connected_node_idx, paths[*connected_path]));
            }
        }
        nodes.push(connected_nodes);
    }

    let new_node_names = [
        "goal", "start", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    for (i, node) in nodes.iter().enumerate() {
        for (n, c) in node {
            println!("node{i} -> node{n} [label={c}]")
        }
    }

    // Now that we've built the graph, we can run the algorithm on it.
    // this link has an alogithm
    // https://stackoverflow.com/a/2525353/13274599

    // a path has a history and a total cost
    let mut highest_cost = 0;
    let mut paths: Vec<(Vec<usize>, usize)> = vec![(vec![1], 0)];
    loop {
        let Some((path, cost)) = paths.pop() else {
            //No more paths to check
            break;
        };

        if path[path.len() - 1] == 0 {
            // -3 because the start node, goal node don't count towards cost,
            // plus the final path doesn't hit an intersection.
            // println!("cost {} Path: {path:?}", cost + path.len() - 3);

            highest_cost = highest_cost.max(cost + path.len() - 3);
        }

        // Vec<neighbor, cost>
        let neighbors = &nodes[path[path.len() - 1]];
        for (neighbor, edge_cost) in neighbors {
            if !path.contains(neighbor) {
                let mut new_path = path.clone();
                new_path.push(*neighbor);
                paths.push((new_path, cost + edge_cost))
            }
        }
    }

    // 6450
    Solution::NumUsize(highest_cost)
}
