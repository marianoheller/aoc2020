use std::{borrow::Borrow, collections::HashMap};
use utils::read_lines;

fn main() {
    let lines = read_lines("src/inputs/day_06.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let result = solve_p1(&lines);
    println!("Part 1: {:?}", result);

    let result = solve_p2(&lines);
    println!("Part 2: {:?}", result);
}

pub fn solve_p1<T: AsRef<str> + Borrow<str>>(lines: &Vec<T>) -> usize {
    lines
        .split(|s| s.as_ref() == "")
        .map(|group| {
            let mut chars = group.join("").chars().collect::<Vec<_>>();
            chars.sort();
            chars.dedup();
            chars.len()
        })
        .fold(0, |acc, v| acc + v)
}

pub fn solve_p2<T: AsRef<str> + Borrow<str>>(lines: &Vec<T>) -> usize {
    lines
        .split(|s| s.as_ref() == "")
        .map(|group| {
            let people_qty = group.len();
            group
                .join("")
                .chars()
                .fold(HashMap::new(), |mut acc, ch| {
                    let counter = acc.entry(ch).or_insert(0);
                    *counter += 1;
                    acc
                })
                .into_iter()
                .filter(|(_, count)| *count == people_qty)
                .collect::<Vec<_>>()
                .len()
        })
        .fold(0, |acc, v| acc + v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let lines = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ];

        assert_eq!(solve_p1(&lines), 11);
    }

    #[test]
    fn example_p2() {
        let lines = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ];

        assert_eq!(solve_p2(&lines), 6);
    }
}
