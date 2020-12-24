pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<i64> {
    input.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Node<T> {
    pub val : T
}

impl<T> Node<T> {
    pub fn new(val : T) -> Self {
        Node { val }
    }
}