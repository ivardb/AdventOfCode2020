use crate::days::day9::{parse_input, default_input};

pub fn run() {
    println!("{}", xmas_str(default_input()).unwrap())
}

pub fn xmas_str(input : &str) -> Result<i64, ()> {
    let nums = parse_input(input);
    let target = super::part1::xmas(nums.clone());
    xmas(nums, target.unwrap())
}

pub fn xmas(nums : Vec<i64>, target : i64) -> Result<i64, ()> {
    for i in 0..nums.len() {
        let mut sum = nums[i];
        let mut j = i + 1;
        while sum < target {
            sum += nums[j];
            j += 1
        }
        if sum == target {
            let mut min = i64::max_value();
            let mut max = i64::min_value();
            for k in i..j {
                if nums[k] > max {
                    max = nums[k]
                }
                if nums[k] < min {
                    min = nums[k]
                }
            }
            return Ok(min + max)
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day9::default_input;

    #[test]
    pub fn part2_answer() {
        assert_eq!(137394018, xmas_str(default_input()).unwrap())
    }
}