mod days;
use days::*;
use std::time::Instant;
use criterion::black_box;

pub fn run(day: usize) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
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
        2 => {
            println!("Part 1: {}", day2::part1::password_validator_str(input).unwrap());
            println!("Part 2: {}", day2::part2::password_validator_str(input).unwrap());
        }
        3 => {
            println!("Part 1: {}", day3::part1::route_str(input).unwrap());
            println!("Part 2: {}", day3::part2::route_str(input).unwrap());
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn benchmark(day : usize) {
    println!("Benchmarking day: {}", day);
    let t1 : f64;
    let t2 : f64;
    match day {
        1 => {
            let input = day1::default_input();
            t1 = benchmark_function(&*input, &day1::part1::expense_rapport_str);
            t2 = benchmark_function(&*input, &day1::part2::expense_rapport_str);
        }
        2 => {
            let input = day2::default_input();
            t1 = benchmark_function(&*input, &day2::part1::password_validator_str);
            t2 = benchmark_function(&*input, &day2::part2::password_validator_str);
        }
        3 => {
            let input = day3::default_input();
            t1 = benchmark_function(&*input, &day3::part1::route_str);
            t2 = benchmark_function(&*input, &day3::part2::route_str);
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
    println!("Part 1 took: {} seconds", t1);
    println!("Part 2 took: {} seconds", t2);
}


#[allow(unused_must_use)]
fn benchmark_function<T, E>(input : &str, f : &dyn Fn(&str) -> Result<T, E>) -> f64 {
    let start = Instant::now();
    black_box(f(input));
    start.elapsed().as_micros() as f64/i32::pow(10, 6) as f64
}

pub fn criterion_bench_part1(day : usize, input : &str) {
    match day {
        1 => {
            day1::part1::expense_rapport_str(input).unwrap();
        }
        2 => {
            day2::part1::password_validator_str(input).unwrap();
        }
        3 => {
            day3::part1::route_str(input).unwrap();
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn criterion_bench_part2(day : usize, input : &str) {
    match day {
        1 => {
            day1::part2::expense_rapport_str(input).unwrap();
        }
        2 => {
            day2::part2::password_validator_str(input).unwrap();
        }
        3 => {
            day3::part2::route_str(input).unwrap();
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}