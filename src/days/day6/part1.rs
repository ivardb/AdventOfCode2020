use crate::days::day6::{default_input, parse_input};

pub fn run() {
    println!("{}", customs_groups_str(default_input()).unwrap())
}

pub fn customs_groups_str(input : &str) -> Result<usize, ()> {
    customs_groups(parse_input(input))
}

pub fn customs_groups(groups : Vec<Vec<&str>>) -> Result<usize, ()> {
    Ok(groups.iter().map(|group| {
        let mut letters = [false; 26];
        for p in group {
            for c in p.chars() {
                letters[(c as u8 - b'a') as usize] = true;
            }
        }
        letters.iter().filter(|b| **b).count()
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(6612, customs_groups_str(default_input()).unwrap())
    }
}