use std::collections::HashMap;

use regex::Regex;
use utils::read_lines;

pub fn cartesian_product(al: &Vec<String>, bl: &Vec<String>) -> Vec<String> {
    let mut out = vec![];
    for a in al.iter() {
        for b in bl.iter() {
            let mut new_str = a.clone();
            new_str.push_str(b);
            out.push(new_str);
        }
    }
    out
}

pub fn cartesian_products(lists: Vec<Vec<String>>) -> Vec<String> {
    let mut out = lists[0].clone();

    for list in lists[1..].iter() {
        out = cartesian_product(&out, list)
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Validator {
    rules: HashMap<usize, String>,
}

impl Validator {
    pub fn generate(&self) -> Vec<String> {
        let mut result: HashMap<usize, Vec<String>> = HashMap::new();

        while result.keys().len() < self.rules.len() {
            for (n, rule) in self.rules.iter() {
                if !result.contains_key(n) {
                    if rule.starts_with("\"") {
                        let char = rule.replace("\"", "");
                        result.insert(*n, vec![char]);
                    } else {
                        let all_children_parsed =
                            rule.clone().replace(" | ", " ").split(" ").all(|s| {
                                let key = s.parse::<usize>().unwrap();
                                result.contains_key(&key)
                            });
                        if all_children_parsed {
                            for and_target_rules in rule.split(" | ") {
                                // println!("result  {:?}", result);
                                // println!("and_target_rules  {:?}", and_target_rules);

                                let line_rules = and_target_rules
                                    .split(" ")
                                    .map(|c| {
                                        let key = c.parse::<usize>().unwrap();
                                        result.get(&key).unwrap().clone()
                                    })
                                    .collect::<Vec<_>>();

                                let mut calced = cartesian_products(line_rules);
                                let entry = result.entry(*n).or_insert(Vec::new());
                                (*entry).append(&mut calced);
                            }
                        }
                    }
                }
            }

            println!("keys  {:?}", result.keys());
        }

        // println!("result  {:?}", result);

        let mut zero = result.get(&0).unwrap().clone();
        zero.sort();
        zero.dedup();

        zero
    }

    pub fn from_lines<T: AsRef<str>>(lines: Vec<T>) -> Self {
        let re = Regex::new(r#"^(\d+): (.*)$"#).unwrap();
        let rules = lines
            .iter()
            .map(|line| {
                let cap = re.captures_iter(line.as_ref()).next().unwrap();
                let n = cap[1].parse::<usize>().unwrap();
                (n, cap[2].to_owned())
            })
            .collect::<HashMap<usize, String>>();

        Validator { rules }
    }
}

fn main() {
    let lines = read_lines("src/inputs/day_19.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let mut it = lines.split(|s| s == "").take(2);

    let rules = it.next().unwrap().to_vec();
    let msgs = it.next().unwrap().to_vec();

    let validator = Validator::from_lines(rules);
    let valids = validator.generate();

    let found_valids = msgs
        .into_iter()
        .filter(|m| valids.contains(m))
        .collect::<Vec<_>>();

    println!("P1: Got total valids {:?}", found_valids.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let lines = vec!["0: 1 2", "1: \"a\"", "2: 1 3 | 3 1", "3: \"b\""];
        let validator = Validator::from_lines(lines);
        let valids = validator.generate();
        assert_eq!(valids, vec!["aab", "aba"]);

        let lines = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
        ];
        let validator = Validator::from_lines(lines);
        let valids = validator.generate();
        [
            "aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb",
        ]
        .iter()
        .for_each(|valid| assert!(valids.contains(&(*valid).to_owned())))
    }

    #[test]
    fn example_p1_first_last() {
        let lines = vec!["1: \"a\"", "2: 1 3 | 3 1", "3: \"b\"", "0: 1 2"];
        let validator = Validator::from_lines(lines);
        let valids = validator.generate();
        assert_eq!(valids, vec!["aab", "aba"]);

        let lines = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
        ];
        let validator = Validator::from_lines(lines);
        let valids = validator.generate();
        [
            "aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb",
        ]
        .iter()
        .for_each(|valid| assert!(valids.contains(&(*valid).to_owned())))
    }
}
