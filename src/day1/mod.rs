use anyhow::Result;
use itertools::iproduct;
use std::collections::HashSet;

use crate::Runner;

pub struct Day01;

impl Runner for Day01 {
    type Input = HashSet<i32>;
    type Output = i32;

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let nums = input
            .lines()
            .map(&str::trim)
            .map(|l| l.parse().unwrap())
            .collect();
        Ok(nums)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut products = input
            .into_iter()
            .map(|&a| (a, 2020 - a))
            .filter(|(_, b)| input.contains(&b))
            .map(|(a, b)| a * b);
        Ok(products.next().unwrap())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut products = iproduct!(input.iter(), input.iter())
            .filter(|(&a, &b)| a + b < 2020)
            .map(|(&a, &b)| (a, b, 2020 - a - b))
            .filter(|(_, _, c)| input.contains(&c))
            .map(|(a, b, c)| a * b * c);
        Ok(products.next().unwrap())
    }
}
