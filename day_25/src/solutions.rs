use shared::Solution;
pub fn pt_1(str_input: &str) -> Solution {
    let mut node_vec: Vec<&str> = vec![];
    let mut node_connections: Vec<Vec<usize>> = vec![];

    for line in str_input.lines() {
        let [name, others @ ..] = &line.split(' ').collect::<Vec<_>>()[..] else {
            panic!("Bad line")
        };
        // name has the colon in it so use name[..name.len() - 1];
        let index = match node_vec.iter().position(|s| **s == name[..name.len() - 1]) {
            Some(i) => i,
            None => {
                node_vec.push(&name[..name.len() - 1]);
                node_connections.push(vec![]);
                node_vec.len() - 1
            }
        };
        for o_name in others {
            let o_index = match node_vec.iter().position(|s| s == o_name) {
                Some(i) => i,
                None => {
                    node_vec.push(o_name);
                    node_connections.push(vec![]);
                    node_vec.len() - 1
                }
            };
            node_connections[index].push(o_index);
            node_connections[o_index].push(index);
        }
    }

    println!("nodes: {node_vec:?}");
    println!("nodes: {node_connections:?}");

    for node in node_connections.iter() {
        if node.len() < 4 {
            panic!("len of {node:?} not 4")
        }
    }

    let mut closeness_matrix: Vec<Vec<Option<usize>>> =
        vec![vec![None; node_vec.len()]; node_vec.len()];
    fn find_dist(
        node_connections: &[Vec<usize>],
        closeness_matrix: &mut [Vec<Option<usize>>],
        node: usize,
        goal: usize,
    ) -> usize {
        if node == goal {
            0
        } else if closeness_matrix[node][goal].is_some() {
            closeness_matrix[node][goal].unwrap()
        } else {
            node_connections[node]
                .iter()
                .map(|child| {
                    let d = find_dist(node_connections, closeness_matrix, *child, goal);
                    closeness_matrix[*child][goal] = Some(d);
                    closeness_matrix[goal][*child] = Some(d);
                    d
                })
                .min()
                .unwrap()
                + 1
        }
    }

    for i in 0..node_vec.len() {
        for j in 0..node_vec.len() {
            if i != j {
                let d = find_dist(&node_connections, &mut closeness_matrix, i, j);
            }
        }
    }

    // let mut possible_splits = vec![];
    // for (i, node) in node_vec.iter().enumerate() {
    //     for connection in &node_connections[i] {
    //         if !node_connections[i]
    //             .iter()
    //             .any(|i| node_connections[*connection].contains(i))
    //         {
    //             println!(
    //                 "Possible split found between {} and {}",
    //                 node_vec[i], node_vec[*connection]
    //             );
    //             // println!(
    //             //     "Possible split found between {:?} and {:?}",
    //             //     node_connections[i], node_connections[*connection]
    //             // );
    //             possible_splits.push((i, connection))
    //         }
    //     }
    // }

    Solution::None(str_input)
}

pub fn pt_2(str_input: &str) -> Solution {
    Solution::None(str_input)
}
