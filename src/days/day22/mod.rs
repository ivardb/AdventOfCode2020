use std::collections::VecDeque;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> (VecDeque<i64>, VecDeque<i64>) {
    let mut items = input.split("\n\n").map(|text| {
        text.lines().skip(1).map(|n| n.parse().unwrap()).collect()
    });
    (items.next().unwrap(), items.next().unwrap())
}