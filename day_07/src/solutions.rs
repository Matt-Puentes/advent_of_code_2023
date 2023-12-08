#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,  // 5 type (1,1,1,1,1)
    OnePair,   // 4 type (2,1,1,1)
    TwoPair,   // 3 type (2,2,1)
    ThreeKind, // 3 type (3,1,1)
    FullHouse, // 2 type (3,2)
    FourKind,  // 2 type (4,1)
    FiveKind,  // 1 type (5)
}

pub fn pt_1(str_input: &str) {
    // To prevent reallocation, declare it here and just clear it each loop
    // let mut counts: HashMap<u32, usize> = HashMap::new();
    let mut hands: Vec<(HandType, [u32; 5], u32)> = str_input
        .lines()
        .map(|s| {
            let mut counts = [0; 15];
            let (hand, bid) = s.split_once(' ').unwrap();
            let parsed_hand: [u32; 5] = hand
                .chars()
                .map(|c| match c {
                    d @ '2'..='9' => d.to_digit(10).unwrap(),
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let parsed_bid = bid.parse::<u32>().unwrap();
            for n in parsed_hand.iter() {
                counts[*n as usize] += 1;
            }
            let mut count = counts.iter().filter(|s| **s != 0).collect::<Vec<_>>();
            count.sort();
            let hand_type = match &count[..] {
                [1, 1, 1, 1, 1] => HandType::HighCard,
                [1, 1, 1, 2] => HandType::OnePair,
                [1, 2, 2] => HandType::TwoPair,
                [1, 1, 3] => HandType::ThreeKind,
                [2, 3] => HandType::FullHouse,
                [1, 4] => HandType::FourKind,
                [5] => HandType::FiveKind,
                _ => unreachable!(),
            };
            (hand_type, parsed_hand, parsed_bid)
        })
        .collect();

    hands.sort_by(|(h1, c1, _), (h2, c2, _)| {
        if h1 != h2 {
            h1.partial_cmp(h2).unwrap()
        } else {
            for (ch1, ch2) in c1.iter().zip(c2.iter()) {
                if ch1 != ch2 {
                    return ch1.partial_cmp(ch2).unwrap();
                }
            }
            std::cmp::Ordering::Equal
        }
    });

    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(r, (_, _, b))| (((r as u32) + 1) * *b))
        .sum();

    println!("Part 1 result: {}", sum)
}

pub fn pt_2(str_input: &str) {
    // Parse the hand types, hand cards, and bids.
    let mut hands: Vec<(HandType, [u32; 5], u32)> = str_input
        .lines()
        .map(|s| {
            let (hand, bid) = s.split_once(' ').unwrap();
            let parsed_hand: [u32; 5] = hand
                .chars()
                .map(|c| match c {
                    d @ '2'..='9' => d.to_digit(10).unwrap(),
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 0,
                    'T' => 10,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let mut joker_count = 0;
            let mut counts = [0; 15];
            for n in parsed_hand.iter() {
                if *n == 0 {
                    joker_count += 1
                } else {
                    counts[*n as usize] += 1;
                }
            }
            let mut count = counts.iter_mut().filter(|s| **s != 0).collect::<Vec<_>>();
            count.sort_by(|a, b| b.cmp(a));

            let hand_type = if count.is_empty() {
                HandType::FiveKind
            } else {
                *count[0] += joker_count;
                match &count[..] {
                    [1, 1, 1, 1, 1] => HandType::HighCard,
                    [2, 1, 1, 1] => HandType::OnePair,
                    [2, 2, 1] => HandType::TwoPair,
                    [3, 1, 1] => HandType::ThreeKind,
                    [3, 2] => HandType::FullHouse,
                    [4, 1] => HandType::FourKind,
                    [5] => HandType::FiveKind,
                    _ => unreachable!(),
                }
            };

            let parsed_bid = bid.parse::<u32>().unwrap();
            (hand_type, parsed_hand, parsed_bid)
        })
        .collect();

    // Sort the hands
    hands.sort_by(|(h1, c1, _), (h2, c2, _)| {
        if h1 != h2 {
            h1.partial_cmp(h2).unwrap()
        } else {
            // Go through character-by-character, first one to "win" resolves the sort.
            for (ch1, ch2) in c1.iter().zip(c2.iter()) {
                if ch1 != ch2 {
                    return ch1.partial_cmp(ch2).unwrap();
                }
            }
            // If nothing broke the tie, they're equal
            std::cmp::Ordering::Equal
        }
    });

    // Sum Rank * bid
    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(r, (_, _, b))| (((r as u32) + 1) * *b))
        .sum();

    println!("Part 2 result: {}", sum)
}
