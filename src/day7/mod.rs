use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use anyhow::Result;
use multimap::MultiMap;
use regex::Regex;

use crate::Runner;

lazy_static::lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\w+\s\w+) bags contain ((\s*(\d+)\s*(\w+\s\w+) bags?,?)+\.|no other bags\.)").unwrap();
    static ref CONTAINS_RE: Regex = Regex::new(r"(\d+) (\w+\s\w+)").unwrap();
}

pub struct Day07;

// TODO: Use `nom`, instead of regex

impl Runner for Day07 {
    type Input = HashMap<u64, Vec<(usize, u64)>>;
    type Output = usize;

    fn day() -> usize {
        7
    }

    fn comment() -> &'static str {
        "Regex"
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|l| {
                let captures = BAG_RE.captures(l).unwrap();
                let bag: String = captures.get(1).unwrap().as_str().into();
                let mut hasher = DefaultHasher::new();
                bag.hash(&mut hasher);
                (hasher.finish(), captures.get(2).unwrap())
            })
            .map(|(bag, contains)| {
                let v: Vec<(usize, u64)> = match contains.as_str() {
                    "no other bags." => vec![],
                    other => CONTAINS_RE
                        .captures_iter(other)
                        .map(|c| {
                            let bag: String = c.get(2).unwrap().as_str().into();
                            let mut hasher = DefaultHasher::new();
                            bag.hash(&mut hasher);
                            (
                                c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                                hasher.finish(),
                            )
                        })
                        .collect(),
                };
                (bag, v)
            })
            .collect::<HashMap<_, _>>())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        let mut hasher = DefaultHasher::new();
        "shiny gold".hash(&mut hasher);
        let shiny_gold = hasher.finish();

        let contained_in = input
            .iter()
            .map(|(&bag, contains)| {
                contains
                    .into_iter()
                    .map(|(_, contain)| (*contain, bag))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<MultiMap<u64, u64>>();
        let mut count = 0;
        let mut queue: VecDeque<u64> = VecDeque::new();
        let mut visited: HashSet<u64> = HashSet::new();
        visited.insert(shiny_gold);
        queue.push_back(shiny_gold);
        while !queue.is_empty() {
            let bag = queue.pop_front().unwrap();
            if !visited.contains(&bag) {
                count += 1;
            }
            visited.insert(bag.clone());
            let contained_in = contained_in.get_vec(&bag);
            for o in contained_in.unwrap_or(&vec![]) {
                queue.push_back(*o);
            }
        }
        Ok(count)
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let mut hasher = DefaultHasher::new();
        "shiny gold".hash(&mut hasher);
        let shiny_gold = hasher.finish();

        let mut queue = VecDeque::<(usize, u64)>::new();
        queue.push_back((1, shiny_gold));
        let mut count: usize = 0;
        let empty = Vec::new();
        while !queue.is_empty() {
            let (i, bag) = queue.pop_front().unwrap();
            count += i as usize;
            let inside = input.get(&bag).unwrap_or(&empty);
            for (o, o_bag) in inside {
                queue.push_back((i * o, o_bag.clone()));
            }
        }
        Ok(count - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags.";

        let input = Day07::get_input(input)?;
        println!("{:#?}", input);
        assert_eq!(4, Day07::part1(&input)?);
        assert_eq!(32, Day07::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "shiny gold bags contain 2 dark red bags.\n\
                     dark red bags contain 2 dark orange bags.\n\
                     dark orange bags contain 2 dark yellow bags.\n\
                     dark yellow bags contain 2 dark green bags.\n\
                     dark green bags contain 2 dark blue bags.\n\
                     dark blue bags contain 2 dark violet bags.\n\
                     dark violet bags contain no other bags.";

        let input = Day07::get_input(input)?;
        println!("{:#?}", input);
        // assert_eq!(4, Day07::part1(&input)?);
        assert_eq!(126, Day07::part2(&input)?);
        Ok(())
    }
}
