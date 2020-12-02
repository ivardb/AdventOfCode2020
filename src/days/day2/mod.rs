use std::str::FromStr;
use std::num::ParseIntError;

pub mod part1;
pub mod part2;

pub fn default_input() -> String {
    String::from(include_str!("input"))
}

pub fn run() {
    part1::run();
    part2::run();
}

struct Row {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

impl FromStr for Row {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_ascii_whitespace().collect();
        let numsplit : Vec<&str> = split[0].split("-").collect();
        let min : i32 = numsplit[0].trim().parse()?;
        let max : i32 = numsplit[1].trim().parse()?;
        let chars : Vec<char> = split[1].trim().chars().collect();
        let letter = chars[0];
        let password = String::from(split[2].trim());
        Ok(Row {min, max, letter, password})
    }
}

impl Row {
    fn is_valid(&self) -> bool {
        let mut letter_count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                letter_count += 1;
            }
        }
        letter_count <= self.max && letter_count >= self.min
    }

    fn is_valid2(&self) -> bool {
        let chars : Vec<char> = self.password.chars().collect();
        (chars[(self.min-1) as usize] == self.letter) ^ (chars[(self.max-1) as usize] == self.letter)
    }
}