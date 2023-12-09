pub fn pt_1(str_input: &str) {
    let sum = str_input
        .lines()
        .map(|hist| {
            let mut ends = vec![];
            let mut line = hist
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>();
            ends.push(*line.last().unwrap());

            // Add to lines until a line that's all the same is discovered
            loop {
                let new_line: Vec<i32> = line.windows(2).map(|w| w[1] - w[0]).collect();
                // If it's all the same number, break
                if new_line.windows(2).all(|w| w[0] == w[1]) {
                    ends.push(*new_line.last().unwrap());
                    break;
                }
                ends.push(*new_line.last().unwrap());
                line = new_line;
            }
            ends.iter().sum::<i32>()
        })
        .sum::<i32>();

    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    let sum = str_input
        .lines()
        .map(|hist| {
            let mut starts = vec![];
            let mut line = hist
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>();
            starts.push(line[0]);

            // Add to lines until a line that's all the same is discovered
            loop {
                let new_line: Vec<i32> = line.windows(2).map(|w| w[1] - w[0]).collect();
                // If it's all the same number, break
                if new_line.windows(2).all(|w| w[0] == w[1]) {
                    starts.push(new_line[0]);
                    break;
                }
                starts.push(new_line[0]);
                line = new_line;
            }

            starts.iter().rev().fold(0, |acc, s| s - acc)
        })
        .sum::<i32>();

    println!("Part 2 result: {}", sum)
}
