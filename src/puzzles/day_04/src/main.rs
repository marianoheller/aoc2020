use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};
use utils::read_lines;

fn main() {
    let lines = read_lines("src/inputs/day_04.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let p1 = count_valid(&lines);
    println!("Part 1: {:?}", p1);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    pub fn custom_deserialize<T: AsRef<str>>(str: T) -> Option<Passport> {
        str.as_ref()
            .split(|c| c == ' ' || c == '\n')
            .map(|field| -> Option<(&str, &str)> {
                let mut it_split = field.split(":");
                let key = it_split.next()?;
                let value = it_split.next()?;
                Some((key, value))
            })
            .collect::<Option<HashMap<_, _>>>()
            .and_then(|values| -> Option<Passport> {
                let json_str = serde_json::to_string(&values).ok()?;
                let result: Passport = serde_json::from_str(&json_str).ok()?;
                Some(result)
            })
    }

    pub fn from_multiline_input<T: AsRef<str> + Borrow<str>>(
        arr: &Vec<T>,
    ) -> Vec<Option<Passport>> {
        arr.split(|s| s.as_ref() == "")
            .map(|s| Passport::custom_deserialize(s.join("\n")))
            .collect::<Vec<_>>()
    }
}

pub fn count_valid<T: AsRef<str> + Borrow<str>>(input: &Vec<T>) -> usize {
    Passport::from_multiline_input(input)
        .into_iter()
        .filter(|v| v.is_some())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [&str; 13] = [
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in",
    ];

    #[test]
    fn success_deserialize_without_newline() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffff byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(Passport::custom_deserialize(&input).is_some());
    }

    #[test]
    fn success_deserialize_with_newline() {
        let input = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_some());
    }

    #[test]
    fn success_deserialize_without_cid() {
        let input = [
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_some());
    }

    #[test]
    fn failue_deserialize_without_mandatory_fields() {
        let input = [
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_none());
    }

    #[test]
    fn failue_deserialize_without_other_mandatory_fields() {
        let input = [
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_none());
    }

    #[test]
    fn example_p1() {
        assert_eq!(count_valid(&INPUT.to_vec()), 2);
    }
}
