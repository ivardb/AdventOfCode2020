use std::io;
use aoc2020;

fn main() {
    aoc2020::run(1);
    loop {
        println!("Enter the day number: ");
        let mut day = String::new();
        io::stdin().read_line(&mut day).expect("Failed to read input");
        let day : usize = match day.trim().parse() {
            Ok(n) => n,
            Err(_) => break
        };
        aoc2020::run(day);
    }
}
