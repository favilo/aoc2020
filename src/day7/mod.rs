use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use anyhow::Result;
// use daggy::Dag;
use multimap::MultiMap;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};
use regex::Regex;

use crate::Runner;

lazy_static::lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\w+\s\w+) bags contain ((\s*(\d+)\s*(\w+\s\w+) bags?,?)+\.|no other bags\.)").unwrap();
    static ref CONTAINS_RE: Regex = Regex::new(r"(\d+) (\w+\s\w+)").unwrap();
}

#[allow(dead_code)]
fn contains_parser<'a>(input: &'a str) -> IResult<&'a str, (usize, String)> {
    let (input, (num, bag, _bag, _s, _comma)) = tuple((
        digit1,
        take_until("bag"),
        tag("bag"),
        opt(tag("s")),
        opt(tag(", ")),
    ))(input)?;
    Ok((input, (num.parse().unwrap(), bag.into())))
}

#[allow(dead_code)]
fn bag_parser<'a>(input: &'a str) -> IResult<&'a str, (String, Vec<(usize, String)>)> {
    let (input, name) = take_until(" bags contain ")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    let (input, opt_none) = opt(tag("no other bags"))(input)?;
    let (input, v) = if opt_none.is_some() {
        (input, Vec::new())
    } else {
        many1(contains_parser)(input)?
    };
    let (input, _dot) = tag(".")(input)?;
    assert_eq!("", input);
    Ok((input, (name.into(), v)))
}

pub struct Day07;
pub struct Day07Dag;

// impl Runner for Day07Dag {
//     type Input = ();

//     type Output = usize;

//     fn day() -> usize {
//         7
//     }

//     fn comment() -> &'static str {
//         "DAG"
//     }

//     fn get_input(input: &str) -> Result<Self::Input> {
//         // let mut dag = Dag::<String, usize>::new();
//         let bags = input
//             .lines()
//             .map(bag_parser)
//             .map(Result::unwrap)
//             .map(|(b, vec)| {
//                 todo!()
//             })
//             .collect::<Vec<(&str, &str, usize)>>();
//         let dag: Dag<&str, usize> = bags.iter().collect();
//         println!("{:#?}", dag);
//         todo!()
//     }

//     fn part1(_input: &Self::Input) -> Result<Self::Output> {
//         todo!()
//     }

//     fn part2(_input: &Self::Input) -> Result<Self::Output> {
//         todo!()
//     }
// }

// TODO: Use `nom`, instead of regex

impl Runner for Day07 {
    type Input = HashMap<String, Vec<(usize, String)>>;
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
            .map(bag_parser)
            .map(Result::unwrap)
            .map(|(_, b)| b)
            // .cloned()
            // .map(|l| {
            //     let captures = BAG_RE.captures(l).unwrap();
            //     let bag: String = captures.get(1).unwrap().as_str().into();
            //     let mut hasher = DefaultHasher::new();
            //     bag.hash(&mut hasher);
            //     (hasher.finish(), captures.get(2).unwrap())
            // })
            // .map(|(bag, contains)| {
            //     let v: Vec<(usize, u64)> = match contains.as_str() {
            //         "no other bags." => vec![],
            //         other => CONTAINS_RE
            //             .captures_iter(other)
            //             .map(|c| {
            //                 let bag: String = c.get(2).unwrap().as_str().into();
            //                 let mut hasher = DefaultHasher::new();
            //                 bag.hash(&mut hasher);
            //                 (
            //                     c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            //                     hasher.finish(),
            //                 )
            //             })
            //             .collect(),
            //     };
            //     (bag, v)
            // })
            .collect::<HashMap<_, _>>())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        let mut hasher = DefaultHasher::new();
        "shiny gold".hash(&mut hasher);
        let shiny_gold: String = "shiny gold".into();

        let contained_in = input
            .iter()
            .map(|(bag, contains)| {
                contains
                    .into_iter()
                    .map(|(_, contain)| (contain.clone(), bag.clone()))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<MultiMap<String, String>>();
        let mut count = 0;
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(shiny_gold.clone());
        queue.push_back(shiny_gold);
        while !queue.is_empty() {
            let bag = queue.pop_front().unwrap();
            if !visited.contains(&bag) {
                count += 1;
            }
            visited.insert(bag.clone());
            let contained_in = contained_in.get_vec(&bag);
            for o in contained_in.unwrap_or(&vec![]) {
                queue.push_back(o.clone());
            }
        }
        Ok(count)
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let mut hasher = DefaultHasher::new();
        "shiny gold".hash(&mut hasher);
        let shiny_gold = "shiny gold".into();

        let mut queue = VecDeque::<(usize, String)>::new();
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

    // #[test]
    // fn sample1_dag() -> Result<()> {
    //     let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
    //                  dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
    //                  bright white bags contain 1 shiny gold bag.\n\
    //                  muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
    //                  shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
    //                  dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
    //                  vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
    //                  faded blue bags contain no other bags.\n\
    //                  dotted black bags contain no other bags.";

    //     let input = Day07Dag::get_input(input)?;
    //     println!("{:#?}", input);
    //     assert_eq!(4, Day07Dag::part1(&input)?);
    //     assert_eq!(32, Day07Dag::part2(&input)?);
    //     Ok(())
    // }

    // #[test]
    // fn sample2_dag() -> Result<()> {
    //     let input = "shiny gold bags contain 2 dark red bags.\n\
    //                  dark red bags contain 2 dark orange bags.\n\
    //                  dark orange bags contain 2 dark yellow bags.\n\
    //                  dark yellow bags contain 2 dark green bags.\n\
    //                  dark green bags contain 2 dark blue bags.\n\
    //                  dark blue bags contain 2 dark violet bags.\n\
    //                  dark violet bags contain no other bags.";

    //     let input = Day07Dag::get_input(input)?;
    //     println!("{:#?}", input);
    //     // assert_eq!(4, Day07Dag::part1(&input)?);
    //     // assert_eq!(126, Day07Dag::part2(&input)?);
    //     Ok(())
    // }
}
