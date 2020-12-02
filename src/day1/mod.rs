use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use itertools::iproduct;
use std::collections::HashSet;

type Input = HashSet<i32>;

#[aoc_generator(day1)]
fn get_input(input: &str) -> Result<Input> {
    let nums = input
        .lines()
        .map(&str::trim)
        .map(|l| l.parse().unwrap())
        .collect();
    Ok(nums)
}

#[aoc(day1, part1, contains)]
fn part1_contains(input: &Input) -> Result<i32> {
    let mut products = input
        .into_iter()
        .map(|&a| (a, 2020 - a))
        .filter(|(_, b)| input.contains(&b))
        .map(|(a, b)| a * b);
    Ok(products.next().unwrap())
}

#[aoc(day1, part2, contains)]
fn part2_contains(input: &Input) -> Result<i32> {
    let mut products = iproduct!(input.iter(), input.iter())
        .filter(|(&a, &b)| a + b < 2020)
        .map(|(&a, &b)| (a, b, 2020 - a - b))
        .filter(|(_, _, c)| input.contains(&c))
        .map(|(a, b, c)| a * b * c);
    Ok(products.next().unwrap())
}

