mod days;
use days::*;
use std::time::Instant;
use criterion::black_box;

pub fn run(day: usize) {
    match day {
        1 => {
            day1::part1::run();
            day1::part2::run()
        }
        2 => {
            day2::part1::run();
            day2::part2::run();
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn run_with_input(day : usize, input : &str) {
    match day {
        1 => {
            println!("Part 1: {}", day1::part1::expense_rapport_str(input).unwrap());
            println!("Part 2: {}", day1::part2::expense_rapport_str(input).unwrap());
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn benchmark(day : usize) {
    println!("Benchmarking day: {}", day);
    let input : String;
    let p1 : &dyn Fn(&str) -> Result<_, _>;
    let p2 : &dyn Fn(&str) -> Result<_, _>;
    match day {
        1 => {
            input = day1::default_input();
            p1 = &day1::part1::expense_rapport_str;
            p2 = &day1::part2::expense_rapport_str;
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
    println!("Part 1 took: {} seconds", benchmark_function(&*input, p1));
    println!("Part 2 took: {} seconds", benchmark_function(&*input, p2));
}

fn benchmark_function<T, E>(input : &str, f : &dyn Fn(&str) -> Result<T, E>) -> f64 {
    let start = Instant::now();
    black_box(f(input));
    start.elapsed().as_micros() as f64/1000000f64
}