use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use regex::Regex;

type Pair = ((usize, usize), char);
type Input = Vec<(Pair, String)>;

#[aoc_generator(day2)]
fn get_input(input: &str) -> Result<Input> {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(\w+)").unwrap();
    let lines = lines.filter(|l| re.is_match(&l)).map(|line| {
        let caps = re.captures(&line).unwrap();
        let start = caps[1].parse::<usize>().unwrap();
        let end = caps[2].parse::<usize>().unwrap();
        let c = caps[3].parse::<char>().unwrap();
        let pass = caps[4].to_owned();
        (((start, end), c), pass)
    });
    let lines = lines.collect::<Input>();
    log::info!("{}", lines.len());
    Ok(lines)
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> Result<usize> {
    let correct = input.into_iter().filter(|((range, c), pass)| {
        ((range.0)..=(range.1)).contains(&pass.chars().filter(|o| o == c).count())
    });
    Ok(correct.count())
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> Result<usize> {
    let correct = input
        .into_iter()
        .map(|(((f, s), c), ref pass)| {
            (
                pass.chars().nth(*f - 1) == Some(*c),
                pass.chars().nth(*s - 1) == Some(*c),
            )
        })
        .filter(|(a, b)| a != b);
    Ok(correct.count())
}
