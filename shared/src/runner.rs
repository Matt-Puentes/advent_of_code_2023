use std::fmt::Display;

pub enum Solution<'a> {
    NumI32(i32),
    NumI64(i64),
    NumF32(f32),
    NumF64(f64),
    NumU32(u32),
    NumU64(u64),
    NumUsize(usize),
    Str(&'a str),
    // Prevents "unused variable" warning for the str_input
    None(&'a str),
}

impl<'a> From<i32> for Solution<'a> {
    fn from(value: i32) -> Self {
        Solution::NumI32(value)
    }
}
impl<'a> From<i64> for Solution<'a> {
    fn from(value: i64) -> Self {
        Solution::NumI64(value)
    }
}
impl<'a> From<f32> for Solution<'a> {
    fn from(value: f32) -> Self {
        Solution::NumF32(value)
    }
}
impl<'a> From<f64> for Solution<'a> {
    fn from(value: f64) -> Self {
        Solution::NumF64(value)
    }
}
impl<'a> From<u32> for Solution<'a> {
    fn from(value: u32) -> Self {
        Solution::NumU32(value)
    }
}
impl<'a> From<u64> for Solution<'a> {
    fn from(value: u64) -> Self {
        Solution::NumU64(value)
    }
}
impl<'a> From<usize> for Solution<'a> {
    fn from(value: usize) -> Self {
        Solution::NumUsize(value)
    }
}
impl<'a> From<&'a str> for Solution<'a> {
    fn from(value: &'a str) -> Self {
        Solution::Str(value)
    }
}

impl<'a> Display for Solution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::None(_) => write!(f, "None"),
            Solution::Str(c) => write!(f, "{}", c),
            Solution::NumI32(c) => write!(f, "{}", c),
            Solution::NumI64(c) => write!(f, "{}", c),
            Solution::NumUsize(c) => write!(f, "{}", c),
            Solution::NumU32(c) => write!(f, "{}", c),
            Solution::NumU64(c) => write!(f, "{}", c),
            Solution::NumF32(c) => write!(f, "{}", c),
            Solution::NumF64(c) => write!(f, "{}", c),
        }
    }
}

#[macro_export]
macro_rules! runner {
    ($day:expr, $($inputs:expr), +) => {
        use std::env;
        mod solutions;
        use solutions::{pt_1, pt_2};

        pub enum Test<'a> {
            One(&'a str),
            Two(&'a str),
            Both(&'a str),
        }

        pub fn parse_args() -> Test<'static> {
            let args: Vec<String> = env::args().collect();
            let text = if args.len() > 2 {
                $(if args[2].eq($inputs) {
                    include_str!(concat!("../",$inputs,".txt"))
                } else)* {
                    panic!(concat!("second argument can only be one of "$(, $inputs, ", ")*))
                }
            } else {
                include_str!("../input.txt")
            };
            if args.len() < 2 {
                Test::Both(text)
            } else {
                match &args[1].parse() {
                    Ok(1) => Test::One(text),
                    Ok(2) => Test::Two(text),
                    Ok(0) => Test::Both(text),
                    _ => panic!("invalid test number"),
                }
            }
        }

        pub fn main() {
            println!("Running day {}", $day);
            match parse_args() {
                Test::One(text) => println!("Part 1 Result: {}", pt_1(text)),
                Test::Two(text) => println!("Part 2 Result: {}", pt_2(text)),
                Test::Both(text) => {
                    println!("Part 1 Result: {}", pt_1(text));
                    println!("Part 2 Result: {}", pt_2(text));
                }
            }
        }
    };
}
