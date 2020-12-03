use crate::days::day1::default_input;

pub fn run() {
    let input = default_input();
    println!("Part 1: {}", expense_rapport_str(input).unwrap())
}

pub fn expense_rapport_str(input : &str) -> Result<i32, ()> {
    let nums: Vec<i32> = input.lines().map(str::parse).collect::<Result<Vec<i32>, _>>().unwrap();
    for num in &nums {
        for num2 in &nums {
            if num + num2 == 2020 {
                return Ok(num * num2);
            }
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(918339, expense_rapport_str(default_input()).unwrap())
    }
}