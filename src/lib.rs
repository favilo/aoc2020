use std::{fmt::Debug, fs::read_to_string, time::Instant};

use anyhow::Result;

pub mod day1;
pub mod day2;
// pub mod day3;
// pub mod day4;

pub trait Runner {
    type Input;
    type Output: Debug;

    fn run() -> Result<()> {
        let input = read_to_string(format!("input/2020/day{}.txt", Self::day()))?;
        let input = Self::get_input(&input)?;
        let now = Instant::now();

        let output1 = Self::part1(&input)?;
        let elapsed1 = now.elapsed();

        let now = Instant::now();
        let output2 = Self::part2(&input)?;
        let elapsed2 = now.elapsed();

        log::info!("Part 1 - {:?}", output1);
        log::info!("Took {:?}", elapsed1);
        log::info!("Part 2 - {:?}", output2);
        log::info!("Took {:?}", elapsed2);
        Ok(())
    }

    fn day() -> usize;

    fn get_input(_: &str) -> Result<Self::Input>;
    fn part1(_: &Self::Input) -> Result<Self::Output>;
    fn part2(_: &Self::Input) -> Result<Self::Output>;
}

pub fn run() -> Result<()> {
    day1::Day01::run()?;
    day2::Day02::run()?;
    // day3::Runner::run()?;
    // day4::Runner::run()?;
    Ok(())
}
