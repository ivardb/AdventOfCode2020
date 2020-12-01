use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::{criterion_bench_part1, criterion_bench_part2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/days/day1/input");
    c.bench_function("Day 1, Part 1", |b| b.iter(|| criterion_bench_part1(1, &input)));
    c.bench_function("Day 1, Part 2", |b| b.iter(|| criterion_bench_part2(1, &input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);