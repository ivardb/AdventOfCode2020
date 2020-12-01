pub fn run() {
    let input = include_str!("input");
    println!("Part 1: {}", expense_rapport_str(input))
}

pub fn expense_rapport_str(input : &str) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    for i in input.lines() {
        nums.push(i.trim().parse().unwrap())
    }
    for num in &nums {
        for num2 in &nums {
            if num + num2 == 2020 {
                return num * num2;
            }
        }
    }
    -1
}