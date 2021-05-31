extern crate nom;
use std::usize;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    error::VerboseError,
    multi::many0,
    sequence::delimited,
    sequence::tuple,
    IResult,
};
use utils::read_lines;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn solve_p1<T: AsRef<str>>(lines: &Vec<T>) -> usize {
    let rules = lines
        .into_iter()
        .map(OuterBag::from_str)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let mut result: Vec<String> = Vec::new();
    let mut found: Vec<String> = vec!["shiny gold".to_string()];
    loop {
        found = rules
            .iter()
            .filter(|bag| {
                bag.contents
                    .iter()
                    .any(|inner| found.contains(&inner.color))
            })
            .map(|bag| bag.color.clone())
            .collect::<Vec<_>>();

        result.append(&mut found.clone());

        if found.is_empty() {
            break;
        }
    }
    result.sort();
    result.dedup();

    result.len()
}

fn solve_p2<T: AsRef<str>>(lines: &Vec<T>) -> usize {
    let rules = lines
        .into_iter()
        .map(OuterBag::from_str)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    rules
        .iter()
        .find(|b| b.color == "shiny gold")
        .unwrap()
        .contents
        .iter()
        .map(|b| b.count_bags(&rules, b.qty))
        .sum()
}

fn main() {
    let lines = read_lines("src/inputs/day_07.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let result = solve_p1(&lines);
    println!("Part 1: {:?}", result);

    let result = solve_p2(&lines);
    println!("Part 2: {:?}", result);
}

#[derive(Debug, PartialEq, Eq)]
pub struct InnerBags {
    color: String,
    qty: usize,
}

impl InnerBags {
    pub fn count_bags(&self, rules: &Vec<OuterBag>, prev_count: usize) -> usize {
        let target = rules.iter().find(|b| b.color == self.color).unwrap();
        target
            .contents
            .iter()
            .map(|ib| ib.count_bags(&rules, ib.qty * prev_count))
            .sum::<usize>()
            + prev_count
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OuterBag {
    color: String,
    contents: Vec<InnerBags>,
}

impl OuterBag {
    pub fn from_str<T: AsRef<str>>(str: T) -> Option<Self> {
        outer_bag(str.as_ref()).map(|v| v.1).ok()
    }
}

fn inner_bag(input: &str) -> Res<&str, InnerBags> {
    let end = alt((tag("."), tag(", ")));
    let bag = alt((tag("bags"), tag("bag")));
    let color = tuple((alpha1, tag(" "), alpha1));
    let mut parser = tuple((digit1, delimited(tag(" "), color, tag(" ")), bag, end));

    parser(input).and_then(|(next, output)| {
        let res = InnerBags {
            color: vec![output.1 .0, output.1 .2].join(" "),
            qty: output.0.parse().expect("Unable to parse number"),
        };
        Ok((next, res))
    })
}

fn outer_bag(input: &str) -> Res<&str, OuterBag> {
    let color = tuple((alpha1, tag(" "), alpha1));
    let noise = tag(" bags contain ");
    let mut parser = tuple((color, noise, many0(inner_bag)));

    parser(input).map(|(next, output)| {
        let res = OuterBag {
            color: vec![output.0 .0, output.0 .2].join(" "),
            contents: output.2,
        };
        (next, res)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inner_bag_examples() {
        let result = inner_bag("1 bright white bag, ").unwrap().1;
        assert_eq!(
            result,
            InnerBags {
                color: "bright white".to_string(),
                qty: 1,
            },
        );
    }

    #[test]
    fn multi_inner_bags_examples() {
        let result = many0(inner_bag)("1 bright white bag, 2 muted yellow bags.")
            .unwrap()
            .1;
        assert_eq!(
            result,
            vec![
                InnerBags {
                    color: "bright white".to_string(),
                    qty: 1,
                },
                InnerBags {
                    color: "muted yellow".to_string(),
                    qty: 2,
                },
            ]
        );
    }

    #[test]
    fn examples_p1() {
        let result = outer_bag("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(
            result.unwrap().1,
            OuterBag {
                color: "light red".to_string(),
                contents: vec![
                    InnerBags {
                        color: "bright white".to_string(),
                        qty: 1,
                    },
                    InnerBags {
                        color: "muted yellow".to_string(),
                        qty: 2,
                    },
                ],
            }
        );
    }

    #[test]
    fn no_inner_bag_examples() {
        let result = outer_bag("dotted black bags contain no other bags.")
            .unwrap()
            .1;
        assert_eq!(
            result,
            OuterBag {
                color: "dotted black".to_string(),
                contents: vec![],
            },
        );
    }

    #[test]
    fn example_p1() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        assert_eq!(solve_p1(&input), 4);
    }

    #[test]
    fn example_p2() {
        let input = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];
        assert_eq!(solve_p2(&input), 126);
    }
}
