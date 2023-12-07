pub fn pt_1(str_input: &str) {
    let (time, distance) = str_input.split_once('\n').unwrap();
    let times = time[9..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());
    let distances = distance[9..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());

    let product: i64 = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(t, d)| {
            let solve1 = -(-t + ((t.powi(2) + 4.0 * (-1.0 - d)).sqrt())) / 2.0;
            let solve2 = -(-t - ((t.powi(2) + 4.0 * (-1.0 - d)).sqrt())) / 2.0;
            ((solve2.floor() - solve1.ceil()) + 1.0) as i64
        })
        .product();

    println!("Part 1 result: {}", product)
}

pub fn pt_2(str_input: &str) {
    let (time_str, distance_str) = str_input.split_once('\n').unwrap();
    let time: f64 = time_str
        .chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: f64 = distance_str
        .chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();

    let solve1 = -(-time + ((time.powi(2) + 4.0 * (-1.0 - distance)).sqrt())) / 2.0;
    let solve2 = -(-time - ((time.powi(2) + 4.0 * (-1.0 - distance)).sqrt())) / 2.0;

    println!("Part 2 result: {}", (solve2.floor() - solve1.ceil()) + 1.0)
}
