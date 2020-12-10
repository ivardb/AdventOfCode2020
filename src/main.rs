use std::io;
use aoc2020;

fn main() {
    aoc2020::run(11);
    loop {
        println!("Enter input: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input : Vec<&str> = input.split_ascii_whitespace().collect();
        if input.len() == 1 {
            if let Ok(day) = input[0].trim().parse() {
                aoc2020::run(day)
            } else {
                break
            }
        } else if input.len() > 1 {
            if input[0].contains("benchmark") {
                if let Ok(day) = input[1].trim().parse() {
                    if input.len() == 3 {
                        if let Ok(iter) = input[2].trim().parse() {
                            aoc2020::benchmark(day, iter)
                        } else {
                            aoc2020::benchmark(day, 500)
                        }
                    } else {
                        aoc2020::benchmark(day, 500);
                    }
                } else {
                    break
                }
            }
        } else {
            break
        }
    }
}
