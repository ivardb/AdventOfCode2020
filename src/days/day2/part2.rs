use crate::days::day2::Row;
use std::str::FromStr;

pub fn run() {
    let input = include_str!("input");
    println!("{}", password_validator_str(input).unwrap());
}

pub fn password_validator_str(input : &str) -> Result<usize, ()> {
    Ok(input.lines().map(|l| {Row::from_str(l).unwrap()}).filter(Row::is_valid2).count())
}