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
        if args[2].eq("example") {
            include_str!("../example.txt")
        } else {
            panic!("second argument can only be 'example'")
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
    println!("Running day 16");
    match parse_args() {
        Test::One(text) => pt_1(text),
        Test::Two(text) => pt_2(text),
        Test::Both(text) => {
            pt_1(text);
            pt_2(text);
        }
    }
}
