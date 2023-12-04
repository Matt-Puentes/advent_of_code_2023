pub fn pt_1(str_input: &str) {
    let mut sum = 0;
    for s in str_input.lines() {
        let nums: Vec<_> = s.chars().filter(|s| s.is_numeric()).collect();
        let mut num = String::new();
        num.push(*nums.first().unwrap());
        num.push(*nums.last().unwrap());
        sum += num.parse::<u32>().unwrap();
    }
    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    let numbers = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut sum = 0;
    for s in str_input.lines() {
        let mut num = String::new();
        'forward: for i in 0..s.len() {
            for (idx, number) in numbers.iter().enumerate() {
                if s[i..].starts_with(number) {
                    num += numbers[idx % 9];
                    break 'forward;
                }
            }
        }
        'backward: for i in (0..s.len()).rev() {
            for (idx, number) in numbers.iter().enumerate() {
                if s[i..].starts_with(number) {
                    num += numbers[idx % 9];
                    break 'backward;
                }
            }
        }
        sum += num.parse::<u32>().unwrap();
    }
    println!("Part 2 result: {}", sum)
}
