use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2020::{day1, day2, day3, day4, day5, Runner};

fn day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("day01");
    let input = read_to_string(format!("input/2020/day{}.txt", day1::Day01::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day1::Day01 as Runner>::get_input(&input))
    });
    let input = <day1::Day01 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day1::Day01 as Runner>::part1(&input))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day1::Day01 as Runner>::part2(&input))
    });
    group.finish();
}

fn day02(c: &mut Criterion) {
    let mut group = c.benchmark_group("day02");
    let input = read_to_string(format!("input/2020/day{}.txt", day2::Day02::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day2::Day02 as Runner>::get_input(black_box(&input)))
    });
    let input = <day2::Day02 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day2::Day02 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day2::Day02 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day03(c: &mut Criterion) {
    let mut group = c.benchmark_group("day03");
    let input = read_to_string(format!("input/2020/day{}.txt", day3::Day03::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day3::Day03 as Runner>::get_input(black_box(&input)))
    });
    let input = <day3::Day03 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day3::Day03 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day3::Day03 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day04(c: &mut Criterion) {
    let mut group = c.benchmark_group("day04");
    let input = read_to_string(format!("input/2020/day{}.txt", day4::Day04::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day4::Day04 as Runner>::get_input(black_box(&input)))
    });
    let input = <day4::Day04 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day4::Day04 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day4::Day04 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day05(c: &mut Criterion) {
    let mut group = c.benchmark_group("day05");
    let input = read_to_string(format!("input/2020/day{}.txt", day5::Day05::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day5::Day05 as Runner>::get_input(black_box(&input)))
    });
    let input = <day5::Day05 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day5::Day05 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day5::Day05 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

criterion_group!(benches, day01, day02, day03, day04, day05);
criterion_main!(benches);
