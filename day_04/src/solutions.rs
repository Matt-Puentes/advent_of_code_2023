pub fn pt_1(str_input: &str) {
    let mut sum = 0;
    for line in str_input.lines() {
        let (s1, s2) = line[8..].split_once(" | ").unwrap();

        let winners: Vec<_> = s1.split_ascii_whitespace().collect();
        let val = s2
            .split_ascii_whitespace()
            .filter(|elm| winners.contains(elm))
            .count();
        if val > 0 {
            sum += 2_usize.pow(val as u32 - 1)
        };
    }
    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    let num_cards = str_input.lines().count();
    let mut counts = vec![1; num_cards];
    for (idx, line) in str_input.lines().enumerate() {
        let (s1, s2) = line[8..].split_once(" | ").unwrap();
        let winners = s1.split_ascii_whitespace().collect::<Vec<_>>();
        let score = s2
            .split_ascii_whitespace()
            .filter(|elm| winners.contains(elm))
            .count();
        let num = counts[idx];
        for count in counts.iter_mut().skip(idx + 1).take(score) {
            *count += num;
        }
    }
    println!("Part 2 result: {}", counts.iter().sum::<i32>())
}
