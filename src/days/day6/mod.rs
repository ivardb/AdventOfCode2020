pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<Vec<&str>> {
    input.split("\n\n").map(|g| {g.split("\n").collect::<Vec<_>>()}).collect()
}
