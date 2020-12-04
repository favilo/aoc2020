// use std::borrow::Cow;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use itertools::Itertools;

// I would love to get either of these working
// #[derive(Default, Debug, Clone)]
// struct Passport<'a> {
//     birth_year: Option<Cow<'a, str>>,
//     issue_year: Option<Cow<'a, str>>,
//     expire_year: Option<Cow<'a, str>>,
//     height: Option<Cow<'a, str>>,
//     hair_color: Option<Cow<'a, str>>,
//     eye_color: Option<Cow<'a, str>>,
//     passport_id: Option<Cow<'a, str>>,
//     country_id: Option<Cow<'a, str>>,
// }
// type Input<'a> = Vec<Passport<'a>>;
// type Input<'a> = Vec<HashMap<&str, &str>>;

#[aoc_generator(day4)]
fn get_input(input: &str) -> Result<Vec<HashMap<String, String>>> {
    let entries = input
        .split("\n\n")
        .map(|entry| {
            entry
                .split_whitespace()
                .flat_map(|p| p.split(":"))
                .map(&str::to_owned)
                .tuples()
                .collect()
        })
        .collect::<Vec<HashMap<_, _>>>();
    Ok(entries)
}

#[aoc(day4, part1)]
fn part1(input: &Vec<HashMap<String, String>>) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|pass| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| pass.contains_key(*k))
        })
        .count())
}

#[aoc(day4, part2)]
fn part2(input: &Vec<HashMap<String, String>>) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|pass| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| pass.contains_key(*k))
        })
        .filter(|pass| {
            pass.iter().all(|(k, v)| match k.as_str() {
                "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
                "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
                "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
                "hcl" => {
                    v.starts_with('#')
                        && v.len() == 7
                        && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
                "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
                "cid" => true,
                "hgt" => {
                    let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
                    match &v[(v.len() - 2)..] {
                        "cm" => (150..=193).contains(&height),
                        "in" => (59..=76).contains(&height),
                        _ => false,
                    }
                }
                _ => unreachable!(),
            })
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = get_input(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
             byr:1937 iyr:2017 cid:147 hgt:183cm\n\
             \n\
             iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
             hcl:#cfa07d byr:1929\n\
             \n\
             hcl:#ae17e1 iyr:2013\n\
             eyr:2024\n\
             ecl:brn pid:760753108 byr:1931\n\
             hgt:179cm\n\
             \n\
             hcl:#cfa07d eyr:2025 pid:166559648\n\
             iyr:2011 ecl:brn hgt:59in",
        )
        .unwrap();
        assert_eq!(2, part1(&input)?);
        assert_eq!(2, part2(&input)?);
        Ok(())
    }

    #[test]
    fn invalid() -> Result<()> {
        let input = get_input(
            "eyr:1972 cid:100\n\
             hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
             \n\
             iyr:2019\n\
             hcl:#602927 eyr:1967 hgt:170cm\n\
             ecl:grn pid:012533040 byr:1946\n\
             \n\
             hcl:dab227 iyr:2012\n\
             ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
             \n\
             hgt:59cm ecl:zzz\n\
             eyr:2038 hcl:74454a iyr:2023\n\
             pid:3556412378 byr:2007 ",
        )
        .unwrap();
        assert_eq!(0, part2(&input)?);
        Ok(())
    }

    #[test]
    fn valid() -> Result<()> {
        let input = get_input(
            "pid:087499704 hgt:73in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
             hcl:#623a2f\n\
             \n\
             eyr:2029 ecl:blu cid:129 byr:1989\n\
             iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
             \n\
             hcl:#888785\n\
             hgt:164cm byr:2001 iyr:2015 cid:88\n\
             pid:545766238 ecl:hzl\n\
             eyr:2022\n\
             \n\
             iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        )
        .unwrap();
        assert_eq!(4, part2(&input)?);
        Ok(())
    }
}
