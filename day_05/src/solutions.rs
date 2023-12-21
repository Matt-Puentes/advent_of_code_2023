use shared::Solution;
pub fn pt_1(str_input: &str) -> Solution {
    // Read in the eight "blocks" of text
    let [seed_str, se_so, so_fe, fe_wa, wa_li, li_te, te_hu, hu_lo]: [&str; 8] = str_input
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    // Turn the seed text into seed values
    let mut seeds: Vec<u32> = seed_str[7..]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    // Parse each map as tuples of (dest start, src start, len)
    let maps: Vec<Vec<(u32, u32, u32)>> = vec![se_so, so_fe, fe_wa, wa_li, li_te, te_hu, hu_lo]
        .into_iter()
        .map(|ms| {
            let (_title, map) = ms.split_once(":\n").unwrap();
            map.lines()
                .map(|s| {
                    let [ds, ss, r] = s
                        .split(' ')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()[..3]
                    else {
                        panic!("more than 3 numbers in range")
                    };
                    (ds, ss, r)
                })
                .collect()
        })
        .collect();

    // One map at a time
    for map in maps.into_iter() {
        // For each seed
        'seed: for seed in seeds.iter_mut() {
            // For each range, check if it matches the string
            for (ds, ss, r) in map.iter() {
                if *seed >= *ss && *seed <= ss + r {
                    *seed = ds + (*seed - ss);
                    continue 'seed;
                }
            }
        }
    }
    let res = *seeds.iter().min().unwrap();
    res.into()
}

pub fn pt_2(str_input: &str) -> Solution {
    // Read in the eight "blocks" of text
    let [seed_str, se_so, so_fe, fe_wa, wa_li, li_te, te_hu, hu_lo]: [&str; 8] = str_input
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    // Turn the seed text into ranges of seed values
    let mut seed_ranges: Vec<(u32, u32)> = seed_str[7..]
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|s| (s[0], s[1]))
        .collect();
    // Parse each map as tuples of (dest start, src start, len)
    let maps: Vec<Vec<(u32, u32, u32)>> = vec![se_so, so_fe, fe_wa, wa_li, li_te, te_hu, hu_lo]
        .into_iter()
        .map(|ms| {
            let (_title, map) = ms.split_once(":\n").unwrap();
            map.lines()
                .map(|s| {
                    let [ds, ss, r] = s
                        .split(' ')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()[..3]
                    else {
                        panic!("more than 3 numbers in range")
                    };
                    (ds, ss, r)
                })
                .collect()
        })
        .collect();

    // One map at a time
    for map in maps.into_iter() {
        let mut parsed_ranges: Vec<(u32, u32)> = vec![];
        // For each seed
        // Seed range start, seed range len, seed range end.
        'seed_range: while let Some((sr_s, sr_l)) = seed_ranges.pop() {
            let sr_e = sr_s + (sr_l - 1);

            // For each range, check if it matches the string
            for (ds, ss, l) in map.iter() {
                let se = ss + (l - 1);
                if sr_s <= se && *ss <= sr_e {
                    // range before overlap, if any
                    if sr_s < *ss {
                        seed_ranges.push((sr_s, (ss - sr_s)))
                    }

                    // overlap end - overlap start
                    let overlap_len = ((se).min(sr_e) - ss.max(&sr_s)) + 1;
                    let mapped_start = ds + (ss.max(&sr_s) - ss);

                    parsed_ranges.push((mapped_start, overlap_len));

                    // range after overlap, if any
                    if sr_e > se {
                        seed_ranges.push((se + 1, sr_e - (se + 1)))
                    }
                    continue 'seed_range;
                }
            }
            // If the range wasn't included in any maps, it's good to go.
            parsed_ranges.push((sr_s, sr_l))
        }
        seed_ranges = parsed_ranges.clone();
        parsed_ranges.clear();
    }
    seed_ranges
        .into_iter()
        .map(|(s, _r)| s)
        .min()
        .unwrap()
        .into()
}
