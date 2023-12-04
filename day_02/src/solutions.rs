pub fn pt_1(str_input: &str) {
    let mut sum = 0;
    'line: for (idx, unparsed_line) in str_input.lines().enumerate() {
        let (_, line) = unparsed_line.split_once(':').unwrap();
        for pull in line.split(';') {
            for color in pull.split(',') {
                match color.trim().split_once(' ') {
                    Some((s1, "red")) if s1.parse::<i32>().unwrap() > 12 => continue 'line,
                    Some((s1, "green")) if s1.parse::<i32>().unwrap() > 13 => continue 'line,
                    Some((s1, "blue")) if s1.parse::<i32>().unwrap() > 14 => continue 'line,
                    Some(_) => (),
                    None => panic!("Invalid line {}", color),
                }
            }
        }
        sum += idx + 1;
    }
    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    let mut res = 0;
    for unparsed_line in str_input.lines() {
        let (_, line) = unparsed_line.split_once(':').unwrap();
        let mut max_counts = (0, 0, 0);
        for pull in line.split(';') {
            let mut counts = (0, 0, 0);
            for color in pull.split(',') {
                match color.trim().split_once(' ') {
                    Some((s1, "red")) => counts.0 += s1.parse::<i32>().unwrap(),
                    Some((s1, "green")) => counts.1 += s1.parse::<i32>().unwrap(),
                    Some((s1, "blue")) => counts.2 += s1.parse::<i32>().unwrap(),
                    _ => panic!("Invalid line {}", color),
                }
            }
            max_counts.0 = max_counts.0.max(counts.0);
            max_counts.1 = max_counts.1.max(counts.1);
            max_counts.2 = max_counts.2.max(counts.2);
        }
        res += max_counts.0 * max_counts.1 * max_counts.2
    }
    println!("Part 2 result: {}", res)
}

// pub fn pt_2(str_input: &str) {
//     let res = str_input
//         .lines()
//         .map(|unparsed_line| {
//             let (_, line) = unparsed_line.split_once(':').unwrap();
//             let vals = line.split(';').fold((0, 0, 0), |acc, pull| {
//                 let counts = pull.split(',').fold((0, 0, 0), |mut counts, color| {
//                     match color.trim().split_once(' ') {
//                         Some((s1, "red")) => counts.0 += s1.parse::<i32>().unwrap(),
//                         Some((s1, "green")) => counts.1 += s1.parse::<i32>().unwrap(),
//                         Some((s1, "blue")) => counts.2 += s1.parse::<i32>().unwrap(),
//                         _ => panic!("Invalid line {}", color),
//                     };
//                     counts
//                 });
//                 (
//                     acc.0.max(counts.0),
//                     acc.1.max(counts.1),
//                     acc.2.max(counts.2),
//                 )
//             });
//             vals.0 * vals.1 * vals.2
//         })
//         .sum::<i32>();
//     println!("Part 2 result: {}", res)
// }
