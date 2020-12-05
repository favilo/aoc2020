use std::{fmt::Debug, fs::read_to_string, time::Instant};

use anyhow::Result;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn run() -> Result<()> {
    day1::Day01::run()?;
    day2::Day02::run()?;
    day3::Day03::run()?;
    day4::Day04::run()?;
    day4::Day04Slow::run()?;
    Ok(())
}

pub trait Runner {
    type Input;
    type Output: Debug;

    fn run() -> Result<()> {
        let comment = Self::comment();
        let comment = if comment.is_empty() {
            comment.to_owned()
        } else {
            format!(" : {}", comment)
        };
        log::info!("Day {}{}\n", Self::day(), comment);
        let input = read_to_string(format!("input/2020/day{}.txt", Self::day()))?;
        let input = Self::get_input(&input)?;

        let now = Instant::now();
        let output1 = Self::part1(&input);
        let elapsed1 = now.elapsed();
        let output1 = output1?;

        let now = Instant::now();
        let output2 = Self::part2(&input);
        let elapsed2 = now.elapsed();
        let output2 = output2?;

        log::info!("Part 1 - {:?}", output1);
        log::info!("Took {:?}", elapsed1);
        log::info!("Part 2 - {:?}", output2);
        log::info!("Took {:?}\n", elapsed2);
        Ok(())
    }

    fn day() -> usize;
    fn comment() -> &'static str {
        ""
    }

    fn get_input(_: &str) -> Result<Self::Input>;
    fn part1(_: &Self::Input) -> Result<Self::Output>;
    fn part2(_: &Self::Input) -> Result<Self::Output>;
}
