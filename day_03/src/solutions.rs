use shared::Solution;

pub fn pt_1(str_input: &str) -> Solution {
    let mut symbols: Vec<(usize, usize)> = vec![];
    let mut numbers: Vec<((usize, usize), usize, i32)> = vec![];
    for (lnum, line) in str_input.lines().enumerate() {
        let mut numbuf = String::new();
        let mut cnum = 0;
        let chars = line.chars().collect::<Vec<_>>();
        while cnum < chars.len() {
            match chars[cnum] {
                '.' => (),
                c if c.is_numeric() => {
                    numbuf.push(c);
                    if cnum + 1 == chars.len() || !chars[cnum + 1].is_numeric() {
                        // end number
                        let val: i32 = numbuf.parse().unwrap();
                        numbers.push(((lnum, cnum - (numbuf.len() - 1)), numbuf.len(), val));
                        numbuf.clear()
                    }
                }
                _ => symbols.push((lnum, cnum)),
            }
            cnum += 1;
        }
    }

    let res = numbers
        .into_iter()
        .filter_map(|((line, col), len, val)| {
            if symbols.iter().any(|(sline, scol)| {
                (line >= *sline - 1 && line <= *sline + 1)
                    && (col <= (*scol + 1) && (scol - 1) <= (col + (len - 1)))
            }) {
                Some(val)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    res.iter().sum::<i32>().into()
}

pub fn pt_2(str_input: &str) -> Solution {
    let mut symbols: Vec<(usize, usize)> = vec![];
    let mut numbers: Vec<((usize, usize), usize, i32)> = vec![];
    for (lnum, line) in str_input.lines().enumerate() {
        let mut numbuf = String::new();
        let mut cnum = 0;
        let chars = line.chars().collect::<Vec<_>>();
        while cnum < chars.len() {
            match chars[cnum] {
                '.' => (),
                '*' => symbols.push((lnum, cnum)),
                c if c.is_numeric() => {
                    numbuf.push(c);
                    if cnum + 1 == chars.len() || !chars[cnum + 1].is_numeric() {
                        // end number
                        let val: i32 = numbuf.parse().unwrap();
                        numbers.push(((lnum, cnum - (numbuf.len() - 1)), numbuf.len(), val));
                        numbuf.clear()
                    }
                }
                _ => (),
            }
            cnum += 1;
        }
    }

    let res = symbols
        .into_iter()
        .filter_map(|(sline, scol)| {
            let nums: Vec<_> = numbers
                .iter()
                .filter(|((line, col), len, _)| {
                    (*line >= sline - 1 && *line <= sline + 1)
                        && (*col <= (scol + 1) && (scol - 1) <= (*col + (len - 1)))
                })
                .collect();

            // Only count the gear if there are exactly 2 matching numbers
            if nums.len() == 2 {
                Some(nums[0].2 * nums[1].2)
            } else {
                None
            }
        })
        .sum::<i32>();

    res.into()
}
