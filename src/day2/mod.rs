use anyhow::Result;
use regex::Regex;

use crate::Runner;

pub struct Day02;

type Pair = ((usize, usize), u8);
impl Runner for Day02 {
    type Input = Vec<(Pair, Vec<u8>)>;
    type Output = usize;

    fn day() -> usize {
        2
    }

    #[inline]
    fn get_input(input: &str) -> Result<Self::Input> {
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
        let lines = lines.collect::<Self::Input>();
        log::info!("{}", lines.len());
        Ok(lines)
    }

    #[inline]
    fn part1(input: &Self::Input) -> Result<usize> {
        let correct = input.into_iter().filter(|&(((s, e), c), pass)| -> bool {
            (*s..=*e).contains(&pass.iter().filter(|o| o == &c).count())
        });
        Ok(correct.count())
    }

    #[inline]
    fn part2(input: &Self::Input) -> Result<usize> {
        let correct = input
            .into_iter()
            .map(|(((f, s), c), ref pass)| (pass[*f - 1] == *c as u8, pass[*s - 1] == *c as u8))
            .filter(|(a, b)| a != b);
        Ok(correct.count())
    }
}
