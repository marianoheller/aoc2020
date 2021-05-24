use regex::Regex;
use utils::read_lines;

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    target: char,
    lower: i32,
    upper: i32,
}

impl Rule {
    pub fn new(s: String) -> Self {
        let re = Regex::new(r#"^([0-9]+)-([0-9]+) (.*)$"#).unwrap();

        let captures = re.captures_iter(&s[..]).collect::<Vec<_>>();
        let lower: i32 = captures[0].get(1).unwrap().as_str().parse().unwrap();
        let upper: i32 = captures[0].get(2).unwrap().as_str().parse().unwrap();
        let target = captures[0].get(3).unwrap().as_str().chars().next().unwrap();

        Rule {
            target,
            lower,
            upper,
        }
    }

    pub fn check_part_one_rules(&self, pass: String) -> bool {
        let qty = pass
            .match_indices(&self.target.to_string()[..])
            .collect::<Vec<_>>()
            .len();
        qty >= self.lower as usize && qty <= self.upper as usize
    }

    pub fn check_part_two_rules(&self, pass: String) -> bool {
        let pass_chars = pass.chars().collect::<Vec<_>>();
        let lower = (self.lower - 1) as usize;
        let upper = (self.upper - 1) as usize;
        match (
            pass_chars[lower] == self.target,
            pass_chars[upper] == self.target,
        ) {
            (true, true) => false,
            (false, false) => false,
            _ => true,
        }
    }
}

pub fn parse_lines(lines: Vec<String>) -> Vec<(String, String)> {
    lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(": ");
            let rule = split.nth(0).unwrap();
            let password = split.nth(0).unwrap();
            (rule.to_owned(), password.to_owned())
        })
        .collect()
}

pub fn count_valid_p1(lines: Vec<String>) -> i32 {
    parse_lines(lines)
        .into_iter()
        .fold(0, |acc, (rule, password)| {
            if Rule::new(rule).check_part_one_rules(password) {
                acc + 1
            } else {
                acc
            }
        })
}

pub fn count_valid_p2(lines: Vec<String>) -> i32 {
    parse_lines(lines)
        .into_iter()
        .fold(0, |acc, (rule, password)| {
            if Rule::new(rule).check_part_two_rules(password) {
                acc + 1
            } else {
                acc
            }
        })
}

fn main() {
    let lines = read_lines("src/inputs/day_02.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let p1 = count_valid_p1(lines.clone());
    println!("Part 1: {:?}", p1);

    let p2 = count_valid_p2(lines.clone());
    println!("Part 1: {:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_rule() {
        let proto_rule = "1-3 a".to_owned();
        assert_eq!(
            Rule::new(proto_rule),
            Rule {
                target: 'a',
                lower: 1,
                upper: 3
            }
        )
    }

    #[test]
    fn example_p1() {
        let input = vec![
            "1-3 a: abcde".to_owned(),
            "1-3 b: cdef".to_owned(),
            "2-9 c: ccccccccc".to_owned(),
        ];
        assert_eq!(count_valid_p1(input), 2);
    }

    #[test]
    fn example_p2() {
        let input = vec![
            "1-3 a: abcde".to_owned(),
            "1-3 b: cdef".to_owned(),
            "2-9 c: ccccccccc".to_owned(),
        ];
        assert_eq!(count_valid_p2(input), 1);
    }
}
