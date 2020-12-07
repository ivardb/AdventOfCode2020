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
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn benchmark(day : usize, iter : usize) {
    println!("Benchmarking day: {} with {} iterations", day, iter);
    let mut tp : f64 = 0f64;
    let t1 : f64;
    let t2 : f64;
    match day {
        1 => {
            let input = day1::default_input();
            t1 = benchmark_function(input, &day1::part1::expense_rapport_str, iter);
            t2 = benchmark_function(input, &day1::part2::expense_rapport_str, iter);
            tp = benchmark_function(input, &day1::parse_input, iter)
        }
        2 => {
            let input = day2::default_input();
            t1 = benchmark_function(input, &day2::part1::password_validator_str, iter);
            t2 = benchmark_function(input, &day2::part2::password_validator_str, iter);
            tp = benchmark_function(input, &day2::parse_input, iter)
        }
        3 => {
            let input = day3::default_input();
            t1 = benchmark_function(input, &day3::part1::route_str, iter);
            t2 = benchmark_function(input, &day3::part2::route_str, iter);
            tp = benchmark_function(input, &day3::parse_input, iter)
        }
        4 => {
            let input = day4::default_input();
            t1 = benchmark_function(input, &day4::part1::password_system_str, iter);
            t2 = benchmark_function(input, &day4::part2::password_system_str, iter);
            tp = benchmark_function(input, &day4::parse_input, iter);
        }
        5 => {
            let input = day5::default_input();
            t1 = benchmark_function(input, &day5::part1::boarding_pass_str, iter);
            t2 = benchmark_function(input, &day5::part2::boarding_pass_str, iter);
        }
        6 => {
            let input = day6::default_input();
            t1 = benchmark_function(input, &day6::part1::customs_groups_str, iter);
            t2 = benchmark_function(input, &day6::part2::customs_groups_str, iter);
            tp = benchmark_function(input, &day6::parse_input, iter);
        }
        7 => {
            let input = day7::default_input();
            t1 = benchmark_function(input, &day7::part1::color_bags_str, iter);
            t2 = benchmark_function(input, &day7::part2::color_bags_str, iter);
            tp = benchmark_function(input, &day7::parse_input, iter);
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
fn benchmark_function<'a, T, F: Fn(&'a str) -> T>(input : &'a str, f : F, iter : usize) -> f64 {
    let mut scores = Vec::new();
    for _ in 0..iter {
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
        5 => {
            day5::part1::boarding_pass_str(input).unwrap();
        }
        6 => {
            day6::part1::customs_groups_str(input).unwrap();
        }
        7 => {
            day7::part1::color_bags_str(input).unwrap();
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
        5 => {
            day5::part2::boarding_pass_str(input).unwrap();
        }
        6 => {
            day6::part2::customs_groups_str(input).unwrap();
        }
        7 => {
            day7::part2::color_bags_str(input).unwrap();
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}