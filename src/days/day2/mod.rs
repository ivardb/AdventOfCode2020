use std::str::FromStr;
use std::num::ParseIntError;

pub mod part1;
pub mod part2;

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn run() {
    part1::run();
    part2::run();
}

pub fn parse_input(input : &str) -> Vec<Row> {
    input.lines().map(|l| {Row::from_str(l).unwrap()}).collect()
}

pub struct Row {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for Row {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.trim().split(|c| { c == '-' || c == ' '}).collect();
        let min : usize = split[0].parse()?;
        let max : usize = split[1].parse()?;
        let letter : char = split[2].as_bytes()[0] as char;
        let password = String::from(split[3].trim());
        Ok(Row {min, max, letter, password})
    }
}

impl Row {
    fn is_valid(&self) -> bool {
        let letter_count = self.password.matches(self.letter).count();
        letter_count <= self.max && letter_count >= self.min
    }

    fn is_valid2(&self) -> bool {
        let chars : Vec<char> = self.password.chars().collect();
        (chars[(self.min-1) as usize] == self.letter) ^ (chars[(self.max-1) as usize] == self.letter)
    }
}