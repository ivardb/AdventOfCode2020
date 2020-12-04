pub mod part1;
pub mod part2;

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn run() {
    part1::run();
    part2::run();
}

pub fn parse_input(input : &str) -> Vec<i64> {
    input.lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap()
}