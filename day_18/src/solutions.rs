pub fn pt_1(str_input: &str) {
    // Line, col
    let mut cursor: (i32, i32) = (0, 0);
    let mut sum: i32 = 0;
    let mut perimeter = 0;
    for instruction in str_input.lines() {
        let (dir_s, len_color) = instruction.split_once(' ').unwrap();
        let (len_s, _) = len_color.split_once(' ').unwrap();

        let dir = dir_s.chars().next().unwrap();
        let len = len_s.parse::<u32>().unwrap();

        perimeter += len;

        let new_pos = match dir {
            'U' => (cursor.0 - len as i32, cursor.1),
            'D' => (cursor.0 + len as i32, cursor.1),
            'R' => (cursor.0, cursor.1 + len as i32),
            'L' => (cursor.0, cursor.1 - len as i32),
            _ => unreachable!(),
        };
        sum += (cursor.1 - new_pos.1) * (cursor.0);
        cursor = new_pos;
    }

    println!("Part 1 result: {}", sum + (perimeter as i32 / 2) + 1)
}

pub fn pt_2(str_input: &str) {
    // Line, col
    let mut cursor: (i64, i64) = (0, 0);
    let mut sum: i64 = 0;
    let mut perimeter = 0;
    for instruction in str_input.lines() {
        let (_, len_color) = instruction.split_once(' ').unwrap();
        let (_, color_s) = len_color.split_once(' ').unwrap();

        let color = &color_s[2..8];
        let len = u64::from_str_radix(&color[0..5], 16).unwrap();
        let dir = color.chars().nth(5).unwrap();
        perimeter += len;

        let new_pos = match dir {
            '3' => (cursor.0 - len as i64, cursor.1),
            '1' => (cursor.0 + len as i64, cursor.1),
            '0' => (cursor.0, cursor.1 + len as i64),
            '2' => (cursor.0, cursor.1 - len as i64),
            _ => unreachable!(),
        };

        sum += (cursor.1 - new_pos.1) * (cursor.0);

        cursor = new_pos;
    }

    println!("Part 2 result: {}", sum + (perimeter as i64 / 2) + 1)
}
