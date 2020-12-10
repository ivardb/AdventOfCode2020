use crate::days::day10::{parse_input, default_input};

pub fn run() {
    println!("{}", adapters_str(default_input()).unwrap())
}

pub fn adapters_str(input : &str) -> Result<i64, ()> {
    adapters(parse_input(input))
}

pub fn adapters(mut adapters: Vec<i64>) -> Result<i64, ()> {
    adapters.sort_unstable();
    let mut count1 = 0;
    let mut count3 = 1;
    match adapters[0] {
        1 => count1 += 1,
        3 => count3 += 1,
        _ => {}
    }
    for i in 1..adapters.len() {
        match adapters[i] - adapters[i-1]  {
            1 => count1 += 1,
            3 => count3 += 1,
            _ => {}
        }
    }
    Ok(count1 * count3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(2310, adapters_str(default_input()).unwrap())
    }
}