use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use ndarray::Array2;

type Input = Array2<bool>;

#[aoc_generator(day3)]
fn get_input(input: &str) -> Result<Input> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let vec = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#'))
        .flatten()
        .collect();
    let array = Array2::from_shape_vec((height, width), vec);

    Ok(array?)
}

fn get_trees(input: &Input, run: usize, rise: usize) -> Result<usize> {
    Ok((0..input.dim().0)
        // .take_while(|&i| i < input.dim().1)
        .map(|i| ((i * run) % input.dim().1, i * rise))
        .into_iter()
        .filter(|&(_, x)| x < input.dim().0)
        .filter(|&(y, x)| input[(x, y)])
        .count())
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> Result<usize> {
    get_trees(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> Result<usize> {
    Ok(&[
        get_trees(input, 1, 1)?,
        get_trees(input, 3, 1)?,
        get_trees(input, 5, 1)?,
        get_trees(input, 7, 1)?,
        get_trees(input, 1, 2)?,
    ]
    .iter()
    .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = get_input(
            "..##.......\n\
             #...#...#..\n\
             .#....#..#.\n\
             ..#.#...#.#\n\
             .#...##..#.\n\
             ..#.##.....\n\
             .#.#.#....#\n\
             .#........#\n\
             #.##...#...\n\
             #...##....#\n\
             .#..#...#.#",
        )
        .unwrap();
        assert_eq!(input.dim(), (11, 11));
        assert_eq!(7, part1(&input).unwrap());
        Ok(())
    }
}
