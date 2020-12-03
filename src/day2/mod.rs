use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use regex::Regex;

type Pair = ((usize, usize), u8);
type Input = Vec<(Pair, Vec<u8>)>;

#[aoc_generator(day2)]
fn get_input(input: &str) -> Result<Input> {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let lines = lines.filter(|l| re.is_match(&l)).map(|line| {
        let caps = re.captures(&line).unwrap();
        let start = caps[1].parse::<usize>().unwrap();
        let end = caps[2].parse::<usize>().unwrap();
        let c = caps[3].parse::<char>().unwrap() as u8;
        let pass = caps[4].to_owned();
        (((start, end), c), pass.into_bytes())
    });
    let lines = lines.collect::<Input>();
    log::info!("{}", lines.len());
    Ok(lines)
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> Result<usize> {
    let correct = input.into_iter().filter(|&(((s, e), c), pass)| -> bool {
        (*s..=*e).contains(&pass.iter().filter(|o| o == &c).count())
    });
    Ok(correct.count())
}

#[aoc(day2, part1, fold)]
fn part1_fold(input: &Input) -> Result<usize> {
    let correct = input.into_iter().filter(|&(((s, e), c), pass)| -> bool {
        (*s..=*e).contains(
            &pass
                .iter()
                .fold(0, |acc, o| if o == c { acc + 1 } else { acc }),
        )
    });
    Ok(correct.count())
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> Result<usize> {
    let correct = input
        .into_iter()
        .map(|(((f, s), c), ref pass)| (pass[*f - 1] == *c as u8, pass[*s - 1] == *c as u8))
        .filter(|(a, b)| a != b);
    Ok(correct.count())
}
