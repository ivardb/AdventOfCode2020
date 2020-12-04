use crate::days::day2::{Row, default_input, parse_input};

pub fn run() {
    let input = default_input();
    println!("{}", password_validator_str(input).unwrap());
}

pub fn password_validator_str(input : &str) -> Result<usize, ()> {
    password_validator(parse_input(input))
}

pub fn password_validator(rows: Vec<Row>) -> Result<usize, ()> {
    Ok(rows.iter().filter(|r| {r.is_valid()}).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(447, password_validator_str(default_input()).unwrap())
    }
}