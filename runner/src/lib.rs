use shared::Solution;
pub type Job<'a> = (fn(&'a str) -> Solution<'a>, &'static str, &'static str);

// For each package name, create 2 tuples of (function, name, input).
macro_rules! day {
    ($($num:ident),*) => {
        [$((
            $num::pt_1,
            concat!(stringify!($num), "_pt1"),
            include_str!(concat!("../../", stringify!($num), "/input.txt")),
        ),
        (
            $num::pt_2,
            concat!(stringify!($num), "_pt2"),
            include_str!(concat!("../../", stringify!($num), "/input.txt")),
        ),
        )*]
    };
}

pub fn jobs() -> &'static [Job<'static>] {
    &day!(
        day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11,
        day_12, day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20, day_21, day_22,
        day_23, day_24, day_25
    )
}
