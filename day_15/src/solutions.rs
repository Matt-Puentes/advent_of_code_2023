pub fn pt_1(str_input: &str) {
    let instructions = str_input.split(',').map(|s| s.to_string());
    let res = instructions
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|s| s as u32)
                .fold(0, |acc, c| ((acc + c) * 17) % 256)
        })
        .sum::<u32>();

    //513158
    println!("Part 1 result: {}", res)
}

#[derive(Debug)]
enum Op<'a> {
    Remove(&'a str, u8),
    Add(&'a str, u8, u8),
}

pub fn pt_2(str_input: &str) {
    let instructions = str_input.split(',').map(|s| {
        if let Some((l, n)) = s.split_once('=') {
            Op::Add(
                l,
                l.chars()
                    .map(|s| s as u32)
                    .fold(0, |acc, c| ((acc + c) * 17) % 256) as u8,
                n.parse().unwrap(),
            )
        } else {
            Op::Remove(
                &s[0..s.len() - 1],
                s[0..s.len() - 1]
                    .chars()
                    .map(|s| s as u32)
                    .fold(0, |acc, c| ((acc + c) * 17) % 256) as u8,
            )
        }
    });

    const EMPTY_VEC: Vec<(&str, u8)> = vec![];
    let mut boxes = [EMPTY_VEC; 256];

    for instruction in instructions {
        match instruction {
            Op::Remove(lens_label, box_num) => {
                let b = boxes.get_mut(box_num as usize).unwrap();
                if let Some(index) = b.iter().position(|(x, _)| *x == lens_label) {
                    b.remove(index);
                }
            }
            Op::Add(lens_label, box_num, focal_len) => {
                let b = boxes.get_mut(box_num as usize).unwrap();
                if let Some((_, f)) = b.iter_mut().find(|(x, _)| *x == lens_label) {
                    *f = focal_len
                } else {
                    b.push((lens_label, focal_len))
                }
            }
        }
    }

    let sum = boxes.iter().enumerate().fold(0, |acc, (i, b)| {
        acc + b.iter().enumerate().fold(0, |acc, (sn, (_, f))| {
            acc + ((i + 1) * (sn + 1) * *f as usize)
        })
    });

    //200277
    println!("Part 2 result: {}", sum)
}
