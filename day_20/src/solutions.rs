use shared::math::lcm;
use shared::Solution;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Module {
    Empty,
    Broadcast,
    Flip(bool),
    Conj(Vec<bool>),
}

impl Module {
    fn process(&mut self, i: usize, h: bool, s: usize, m: &[Vec<usize>]) -> Option<bool> {
        match self {
            Module::Empty => None,
            Module::Broadcast => Some(false),
            Module::Flip(_) if h => None,
            Module::Flip(ref mut on) => {
                *on = !*on;
                Some(*on)
            }
            Module::Conj(ref mut mem) => {
                mem[s] = h;
                Some(
                    !mem.iter()
                        .enumerate()
                        .all(|(ni, s)| !m[ni].contains(&i) || *s),
                )
            }
        }
    }
}

pub fn pt_1(str_input: &str) -> Solution {
    let mut broadcast_idx = 0;
    let (mut module_names, mut modules): (Vec<&str>, Vec<Module>) = str_input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let (name, _) = s.split_once(' ').unwrap();
            match name.as_bytes()[0] {
                b'%' => (&name[1..], Module::Flip(false)),
                b'&' => (&name[1..], Module::Conj(vec![])),
                b'b' => {
                    broadcast_idx = i;
                    (name, Module::Broadcast)
                }
                _ => panic!("Invalid name"),
            }
        })
        .unzip();

    let mut module_dests: Vec<Vec<usize>> = str_input
        .lines()
        .map(|s| {
            let (_, dest_list) = s.split_once(" ->").unwrap();
            dest_list
                .split(',')
                .map(|s| {
                    if let Some((i, _)) = module_names
                        .iter()
                        .enumerate()
                        .find(|(_, n)| &s[1..] == **n)
                    {
                        i
                    } else {
                        module_names.push(s);
                        modules.push(Module::Empty);
                        module_names.len() - 1
                    }
                })
                .collect()
        })
        .collect();

    for module in modules.iter_mut() {
        if let Module::Conj(ref mut memory) = module {
            *memory = vec![false; module_names.len()]
        }
    }

    // add empty vec for additional "output" node
    module_dests.push(vec![]);

    let mut counts = [0; 2];
    for _button_press in 0..1000 {
        // "press" button
        let mut signals = VecDeque::from([(100, false, broadcast_idx)]);
        counts[0] += 1;

        while let Some((src, high, idx)) = signals.pop_front() {
            if let Some(high) = modules[idx].process(idx, high, src, &module_dests) {
                for d in &module_dests[idx] {
                    signals.push_back((idx, high, *d));
                }
                counts[high as usize] += module_dests[idx].len();
            }
        }
    }

    (counts[0] * counts[1]).into()
}

pub fn pt_2(str_input: &str) -> Solution {
    // Find broadcast node, create vectors of module names/objects.
    let mut broadcast_idx = 0;
    let (mut module_names, mut modules): (Vec<&str>, Vec<Module>) = str_input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let (name, _) = s.split_once(' ').unwrap();
            match name.as_bytes()[0] {
                b'%' => (&name[1..], Module::Flip(false)),
                b'&' => (&name[1..], Module::Conj(vec![])),
                b'b' => {
                    broadcast_idx = i;
                    (name, Module::Broadcast)
                }
                _ => panic!("Invalid name"),
            }
        })
        .unzip();

    // Create the destination vector for each module and mark the nodes at the end of the graph
    let mut before_rx_idx = 0;
    let mut rx_idx = 0;
    let mut extras = 0;
    let mut module_dests: Vec<Vec<usize>> = str_input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.split_once(" ->")
                .unwrap()
                .1
                .split(',')
                .map(|s| {
                    if let Some((i, _)) = module_names
                        .iter()
                        .enumerate()
                        .find(|(_, n)| &s[1..] == **n)
                    {
                        i
                    } else {
                        extras += 1;
                        module_names.push(&s[1..]);
                        if &s[1..] == "rx" {
                            rx_idx = module_names.len() - 1;
                            before_rx_idx = i
                        };
                        module_names.len() - 1
                    }
                })
                .collect()
        })
        .collect();

    // Add space in other arrays to account for nodes that weren't defined earlier
    for _ in 0..extras {
        modules.push(Module::Empty);
        module_dests.push(vec![]);
    }

    // "Finish" Conjunction modules, since we didn't know the # of modules before.
    for module in modules.iter_mut() {
        if let Module::Conj(ref mut memory) = module {
            *memory = vec![false; module_names.len()]
        }
    }

    // Separate graph into subgraphs
    let mut graphs: Vec<(usize, usize, Vec<bool>)> = vec![];
    for child in &module_dests[broadcast_idx] {
        let mut end_idx = 0;
        let mut module_is_in_subgraph = vec![false; modules.len()];
        let mut queue: Vec<usize> = vec![*child];
        while let Some(i) = queue.pop() {
            for d in &module_dests[i] {
                if *d == before_rx_idx {
                    end_idx = i;
                } else if !module_is_in_subgraph[*d] {
                    queue.push(*d);
                    module_is_in_subgraph[*d] = true;
                }
            }
        }
        graphs.push((*child, end_idx, module_is_in_subgraph))
    }

    // Look for cycles in the graphs
    let mut cycle_idxs: Vec<usize> = vec![];
    for (start_idx, end_idx, graph) in graphs {
        let mut button_press = 0;
        cycle_idxs.push('button: loop {
            // Press button
            button_press += 1;
            let mut signals = VecDeque::from([(100, false, start_idx)]);

            // Loop until signals are done propagating
            while let Some((src, high, idx)) = signals.pop_front() {
                if let Some(high) = modules[idx].process(idx, high, src, &module_dests) {
                    for d in &module_dests[idx] {
                        if idx == end_idx && high {
                            break 'button button_press;
                        } else if graph[*d] {
                            signals.push_back((idx, high, *d));
                        }
                    }
                }
            }
        });
    }

    let res = cycle_idxs.iter().fold(1, |acc, i| lcm(&acc, i));
    res.into()
}
