use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::{criterion_bench_part1, criterion_bench_part2};

pub fn day1_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/days/day1/input");
    c.bench_function("Day 1, Part 1", |b| b.iter(|| criterion_bench_part1(1, &input)));
    c.bench_function("Day 1, Part 2", |b| b.iter(|| criterion_bench_part2(1, &input)));
}

pub fn day2_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/days/day2/input");
    c.bench_function("Day 2, Part 1", |b| b.iter(|| criterion_bench_part1(2, &input)));
    c.bench_function("Day 2, Part 2", |b| b.iter(|| criterion_bench_part2(2, &input)));
}

pub fn day3_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/days/day3/input");
    c.bench_function("Day 3, Part 1", |b| b.iter(|| criterion_bench_part1(3, &input)));
    c.bench_function("Day 3, Part 2", |b| b.iter(|| criterion_bench_part2(3, &input)));
}

pub fn day4_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day4/input");
    c.bench_function("Day 4, Part 1", |b| b.iter(|| criterion_bench_part1(4, &input)));
    c.bench_function("Day 4, Part 2", |b| b.iter(|| criterion_bench_part2(4, &input)));
}

pub fn day5_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day5/input");
    c.bench_function("Day 5, Part 1", |b| b.iter(|| criterion_bench_part1(5, &input)));
    c.bench_function("Day 5, Part 2", |b| b.iter(|| criterion_bench_part2(5, &input)));
}

pub fn day6_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day6/input");
    c.bench_function("Day 6, Part 1", |b| b.iter(|| criterion_bench_part1(6, &input)));
    c.bench_function("Day 6, Part 2", |b| b.iter(|| criterion_bench_part2(6, &input)));
}

pub fn day7_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day7/input");
    c.bench_function("Day 7, Part 1", |b| b.iter(|| criterion_bench_part1(7, &input)));
    c.bench_function("Day 7, Part 2", |b| b.iter(|| criterion_bench_part2(7, &input)));
}

pub fn day8_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day8/input");
    c.bench_function("Day 8, Part 1", |b| b.iter(|| criterion_bench_part1(8, &input)));
    c.bench_function("Day 8, Part 2", |b| b.iter(|| criterion_bench_part2(8, &input)));
}

pub fn day9_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day9/input");
    c.bench_function("Day 9, Part 1", |b| b.iter(|| criterion_bench_part1(9, &input)));
    c.bench_function("Day 9, Part 2", |b| b.iter(|| criterion_bench_part2(9, &input)));
}

pub fn day10_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day10/input");
    c.bench_function("Day 10, Part 1", |b| b.iter(|| criterion_bench_part1(10, &input)));
    c.bench_function("Day 10, Part 2", |b| b.iter(|| criterion_bench_part2(10, &input)));
}

pub fn day11_benchmark(c : &mut Criterion) {
    let input = include_str!("../src/days/day11/input");
    c.bench_function("Day 11, Part 1", |b| b.iter(|| criterion_bench_part1(11, &input)));
    c.bench_function("Day 11, Part 2", |b| b.iter(|| criterion_bench_part2(11, &input)));
}

criterion_group!(benches, day11_benchmark);
criterion_main!(benches);