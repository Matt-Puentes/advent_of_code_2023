use itertools::Itertools;
use shared::Solution;
pub fn pt_1(str_input: &str) -> Solution {
    let mut stars = vec![];
    let w = str_input.find('\n').unwrap();
    let h = str_input.chars().filter(|s| *s == '\n').count() + 1;

    let mut line_stars = vec![false; h];
    let mut col_stars = vec![false; w];
    for (l, line) in str_input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                stars.push((l, c));
                line_stars[l] = true;
                col_stars[c] = true;
            }
        }
    }
    let f: fn((usize, &bool)) -> Option<usize> = |(i, b)| if !*b { Some(i) } else { None };
    let duped_lines: Vec<usize> = line_stars.iter().enumerate().filter_map(f).collect();
    let duped_cols: Vec<usize> = col_stars.iter().enumerate().filter_map(f).collect();

    let stars_gaps = stars.into_iter().map(|(l, c)| {
        let lc = duped_lines.iter().filter(|dl| **dl < l).count();
        let cc = duped_cols.iter().filter(|dc| **dc < c).count();
        ((l, c), (lc, cc))
    });

    let mut sum = 0;
    for pair in stars_gaps.into_iter().combinations(2) {
        let [((l1, c1), (lc1, cc1)), ((l2, c2), (lc2, cc2))] = pair[..] else {
            panic!("Bad pair")
        };
        sum += l1.abs_diff(l2) + c1.abs_diff(c2);
        sum += lc1.abs_diff(lc2) + cc1.abs_diff(cc2);
    }

    sum.into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let mut stars = vec![];

    let w = str_input.find('\n').unwrap();
    let h = str_input.chars().filter(|s| *s == '\n').count() + 1;

    let mut line_stars = vec![false; h];
    let mut col_stars = vec![false; w];
    for (l, line) in str_input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                stars.push((l, c));
                line_stars[l] = true;
                col_stars[c] = true;
            }
        }
    }
    let f: fn((usize, &bool)) -> Option<usize> = |(i, b)| if !*b { Some(i) } else { None };
    let duped_lines: Vec<usize> = line_stars.iter().enumerate().filter_map(f).collect();
    let duped_cols: Vec<usize> = col_stars.iter().enumerate().filter_map(f).collect();

    let stars_gaps = stars.into_iter().map(|(l, c)| {
        let lc = duped_lines.iter().filter(|dl| **dl < l).count();
        let cc = duped_cols.iter().filter(|dc| **dc < c).count();
        ((l, c), (lc, cc))
    });

    let mut sum = 0;
    for pair in stars_gaps.into_iter().combinations(2) {
        let [((l1, c1), (lc1, cc1)), ((l2, c2), (lc2, cc2))] = pair[..] else {
            panic!("Bad pair")
        };
        sum += l1.abs_diff(l2) + c1.abs_diff(c2);
        sum += 999_999 * lc1.abs_diff(lc2) + 999_999 * cc1.abs_diff(cc2);
    }

    sum.into()
}
