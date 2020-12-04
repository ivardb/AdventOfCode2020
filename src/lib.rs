pub mod days;
use days::*;
use std::time::Instant;
use criterion::black_box;

pub fn run(day: usize) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn benchmark(day : usize) {
    println!("Benchmarking day: {}", day);
    let mut tp : f64 = 0f64;
    let t1 : f64;
    let t2 : f64;
    match day {
        1 => {
            let input = day1::default_input();
            t1 = benchmark_function(input, &day1::part1::expense_rapport_str);
            t2 = benchmark_function(input, &day1::part2::expense_rapport_str);
        }
        2 => {
            let input = day2::default_input();
            t1 = benchmark_function(input, &day2::part1::password_validator_str);
            t2 = benchmark_function(input, &day2::part2::password_validator_str);
        }
        3 => {
            let input = day3::default_input();
            t1 = benchmark_function(input, &day3::part1::route_str);
            t2 = benchmark_function(input, &day3::part2::route_str);
            tp = benchmark_function(input, &day3::parse_input)
        }
        4 => {
            let input = day4::default_input();
            t1 = benchmark_function(input, &day4::part1::password_system_str);
            t2 = benchmark_function(input, &day4::part2::password_system_str);
            tp = benchmark_function(input, &day4::parse_input);
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
    println!("Parsing the input takes: {} ms", tp);
    println!("Part 1 took: {} ms or without parsing: {} ms", t1, t1 - tp);
    println!("Part 2 took: {} ms or without parsing: {} ms", t2, t2 - tp);
}


#[allow(unused_must_use)]
fn benchmark_function<T, E>(input : &str, f : &dyn Fn(&str) -> Result<T, E>) -> f64 {
    let mut scores = Vec::new();
    for _ in 0..500 {
        let start = Instant::now();
        black_box(f(input));
        scores.push(start.elapsed().as_micros() as f64 / i32::pow(10, 3) as f64);
    }
    scores.iter().sum::<f64>() / scores.len() as f64
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
        4 => {
            day4::part1::password_system_str(input).unwrap();
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
        4 => {
            day4::part2::password_system_str(input).unwrap();
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}