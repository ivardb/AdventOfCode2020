use crate::days::day2::{Row, default_input};
use std::str::FromStr;

pub fn run() {
    let input = default_input();
    println!("{}", password_validator_str(&*input).unwrap());
}

pub fn password_validator_str(input : &str) -> Result<usize, ()> {
    Ok(input.lines().map(|l| {Row::from_str(l).unwrap()}).filter(Row::is_valid2).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(249, password_validator_str(&*default_input()).unwrap())
    }
}