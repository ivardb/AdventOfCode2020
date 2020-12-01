pub fn run() {
    let input = include_str!("input");
    println!("Part 1: {}", expense_rapport_str(input))
}

pub fn expense_rapport_str(input : &str) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let mut smallest = 2020;
    for i in input.lines() {
        let n: i32 = i.trim().parse().unwrap();
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
                        return num * num2 * num3;
                    }
                }
            } else if sum + smallest == 2020 {
                return num * num2 * smallest;
            }
        }
    }
    -1
}