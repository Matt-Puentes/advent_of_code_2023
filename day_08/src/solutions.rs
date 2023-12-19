use std::{collections::HashMap, vec};

pub fn pt_1(str_input: &str) {
    let (instruction_str, map_str) = str_input.split_once("\n\n").unwrap();
    // Right == true
    let instructions: Vec<bool> = instruction_str.chars().map(|s| s == 'R').collect();

    let map = map_str
        .lines()
        .map(|s| {
            let (name, map) = s.split_once(" = ").unwrap();
            let (left, right) = map.split_once(',').unwrap();
            (name, (&left[1..], &right.trim()[..right.trim().len() - 1]))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut pointer: &str = "AAA";

    for (idx, side) in instructions.iter().cycle().enumerate() {
        if *side {
            pointer = map[pointer].1
        } else {
            pointer = map[pointer].0
        }
        if pointer == "ZZZ" {
            println!("Part 1 result: {}", idx + 1);
            break;
        }
    }
}

fn gcd(i: &u64, o: &u64) -> u64 {
    // Use Stein's algorithm
    let mut m = *i;
    let mut n = *o;
    if m == 0 || n == 0 {
        return m | n;
    }

    // find common factors of 2
    let shift = (m | n).trailing_zeros();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

fn lcm(i: &u64, o: &u64) -> u64 {
    if *i == 0 && *o == 0 {
        return 0;
    }
    let gcd = gcd(i, o);
    *i * (*o / gcd)
}

pub fn pt_2(str_input: &str) {
    let (instruction_str, map_str) = str_input.split_once("\n\n").unwrap();
    let instructions: Vec<bool> = instruction_str.chars().map(|s| s == 'R').collect();

    let map = map_str
        .lines()
        .map(|s| {
            let (name, map) = s.split_once(" = ").unwrap();
            let (left, right) = map.split_once(',').unwrap();
            (name, (&left[1..], &right.trim()[..right.trim().len() - 1]))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    // In the data, starting from 'A' and going to 'Z' is the same as just starting at 'Z'
    let mut ends = map.keys().filter(|s| s.ends_with('Z')).collect::<Vec<_>>();

    let mut cycle_lens: Vec<u64> = vec![];
    let mut idxs: Vec<usize> = vec![];
    for (len, side) in instructions.iter().cycle().enumerate() {
        // All the loops are perfect cycles with no overlap so we don't need to check for duplication,
        //  just how long it takes to find one
        for (idx, pointer) in ends.iter_mut().enumerate() {
            if *side {
                *pointer = &map[*pointer].1;
            } else {
                *pointer = &map[*pointer].0;
            };
            // There's only 1 Z node per loop
            if len != 0 && pointer.ends_with('Z') {
                cycle_lens.push((len + 1) as u64);
                idxs.push(idx)
            }
        }
        for i in idxs.iter() {
            ends.remove(*i);
        }
        if ends.is_empty() {
            break;
        }
        idxs.clear();
    }
    let res: u64 = cycle_lens.iter().fold(1, |acc, i| lcm(&acc, i));
    println!("Part 2 result: {:?}", res);
}
