use crate::days::day6::{default_input, parse_input};

pub fn run() {
    println!("{}", customs_groups_str(default_input()).unwrap())
}

pub fn customs_groups_str(input : &str) -> Result<usize, ()> {
    customs_groups(parse_input(input))
}

pub fn customs_groups(groups : Vec<Vec<&str>>) -> Result<usize, ()> {
    Ok(groups.iter().fold(0, |acc, g| {
        acc + g[0].chars().filter(|c| {g.iter().all(|s| {s.contains(*c)})}).count()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_answer() {
        assert_eq!(3268, customs_groups_str(default_input()).unwrap())
    }
}