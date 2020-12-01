use crate::days::day1::default_input;

pub fn run() {
    let input = default_input();
    println!("Part 1: {}", expense_rapport_str(&*input).unwrap())
}

pub fn expense_rapport_str(input : &str) -> Result<i64, ()> {
    let mut nums: Vec<i64> = Vec::new();
    let mut smallest = 2020;
    for i in input.lines() {
        let n: i64 = i.trim().parse().unwrap();
        if n < smallest {
            smallest = n;
        }
        nums.push(n);
    }
    for num in &nums {
        for num2 in &nums {
            let sum = num + num2;
            if sum + smallest < 2020 {
                for num3 in &nums {
                    if sum + num3 == 2020 {
                        return Ok(num * num2 * num3);
                    }
                }
            } else if sum + smallest == 2020 {
                return Ok(num * num2 * smallest);
            }
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_answer() {
        assert_eq!(23869440, expense_rapport_str(&*default_input()).unwrap())
    }
}