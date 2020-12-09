use crate::days::day9::{parse_input, default_input};

pub fn run() {
    println!("{}", xmas_str(default_input()).unwrap())
}

pub fn xmas_str(input : &str) -> Result<i64, ()> {
    xmas(parse_input(input))
}

pub fn xmas(nums : Vec<i64>) -> Result<i64, ()> {
    for (i, n) in nums.iter().enumerate().skip(25) {
        if !find_sum(&nums[(i as isize - 25) as usize..i], *n) {
            return Ok(*n)
        }
    }
    Err(())
}

fn find_sum(nums : &[i64], sum : i64) -> bool {
    for i in 0..nums.len() {
        for j in 1..nums.len() {
            if nums[i] + nums[j] == sum {
                return true
            }
        } 
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day9::default_input;

    #[test]
    pub fn part1_answer() {
        assert_eq!(1038347917, xmas_str(default_input()).unwrap())
    }
}