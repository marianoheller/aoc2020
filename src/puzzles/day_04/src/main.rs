use regex::Regex;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::{borrow::Borrow, collections::HashMap};
use utils::read_lines;

fn main() {
    let lines = read_lines("src/inputs/day_04.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let result = count_valid(&lines);
    println!("Part: {:?}", result);
}
#[allow(dead_code)]
enum EnumPassportParsed {
    Parsed,
}
#[allow(dead_code)]
enum EnumPassportUnparsed {
    Unparsed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Passport<A> {
    #[serde(skip)]
    __status: PhantomData<A>,
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

type PassportParsed = Passport<EnumPassportParsed>;
type PassportUnparsed = Passport<EnumPassportUnparsed>;

impl Passport<PassportParsed> {
    // Part 1
    pub fn custom_deserialize<T: AsRef<str>>(str: T) -> Option<PassportParsed> {
        str.as_ref()
            .split(|c| c == ' ' || c == '\n')
            .map(|field| -> Option<(&str, &str)> {
                let mut it_split = field.split(":");
                let key = it_split.next()?;
                let value = it_split.next()?;
                Some((key, value))
            })
            .collect::<Option<HashMap<_, _>>>()
            .and_then(|values| -> Option<PassportUnparsed> {
                let json_str = serde_json::to_string(&values).ok()?;
                let result: PassportUnparsed = serde_json::from_str(&json_str).ok()?;
                Some(result)
            })
            .and_then(|unparsed| -> Option<PassportParsed> {
                Some(PassportParsed {
                    __status: PhantomData,
                    byr: bound_numeric_str(unparsed.byr, 1920, 2002)?,
                    iyr: bound_numeric_str(unparsed.iyr, 2010, 2020)?,
                    eyr: bound_numeric_str(unparsed.eyr, 2020, 2030)?,
                    hgt: Passport::hgt(unparsed.hgt)?,
                    hcl: Passport::parse_hcl(unparsed.hcl)?,
                    ecl: Passport::parse_ecl(unparsed.ecl)?,
                    pid: Passport::parse_pid(unparsed.pid)?,
                    cid: unparsed.cid,
                })
            })
    }

    pub fn from_multiline_input_p1<T: AsRef<str> + Borrow<str>>(
        arr: &Vec<T>,
    ) -> Vec<Option<PassportParsed>> {
        arr.split(|s| s.as_ref() == "")
            .map(|s| Passport::custom_deserialize(s.join("\n")))
            .collect::<Vec<_>>()
    }

    fn hgt<T: AsRef<str>>(__str: T) -> Option<T> {
        let str = __str.as_ref();
        let re_cm = Regex::new(r#"^([0-9]+)cm$"#).unwrap();
        let re_in = Regex::new(r#"^([0-9]+)in$"#).unwrap();

        if re_cm.is_match(str.as_ref()) {
            let caps = re_cm.captures_iter(str.as_ref()).collect::<Vec<_>>();
            let n = &caps[0][1].parse::<u32>().ok()?;
            if *n >= 150 && *n <= 193 {
                Some(__str)
            } else {
                None
            }
        } else if re_in.is_match(str.as_ref()) {
            let caps = re_in.captures_iter(str.as_ref()).collect::<Vec<_>>();
            let n = &caps[0][1].parse::<u32>().ok()?;
            if *n >= 59 && *n <= 76 {
                Some(__str)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn parse_hcl<T: AsRef<str>>(str: T) -> Option<T> {
        let re = Regex::new(r#"^#[a-fA-F0-9]{6}$"#).unwrap();
        if re.is_match(str.as_ref()) {
            Some(str)
        } else {
            None
        }
    }

    pub fn parse_ecl<T: AsRef<str>>(str: T) -> Option<T> {
        let re = Regex::new(r#"^(amb|blu|brn|gry|grn|hzl|oth)$"#).unwrap();
        if re.is_match(str.as_ref()) {
            Some(str)
        } else {
            None
        }
    }

    pub fn parse_pid<T: AsRef<str>>(str: T) -> Option<T> {
        let re = Regex::new(r#"^[0-9]{9}$"#).unwrap();
        if re.is_match(str.as_ref()) {
            Some(str)
        } else {
            None
        }
    }
}

fn bound_numeric_str<T: AsRef<str>>(str: T, lower: u32, upper: u32) -> Option<T> {
    let n = str.as_ref().parse::<u32>().ok()?;
    if n >= lower && n <= upper {
        Some(str)
    } else {
        None
    }
}

pub fn count_valid<T: AsRef<str> + Borrow<str>>(input: &Vec<T>) -> usize {
    Passport::from_multiline_input_p1(input)
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
    #[ignore]
    fn success_deserialize_without_newline() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffff byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(Passport::custom_deserialize(&input).is_some());
    }

    #[test]
    #[ignore]
    fn success_deserialize_with_newline() {
        let input = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_some());
    }

    #[test]
    #[ignore]
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
    #[ignore]
    fn failue_deserialize_without_mandatory_fields() {
        let input = [
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_none());
    }

    #[test]
    #[ignore]
    fn failue_deserialize_without_other_mandatory_fields() {
        let input = [
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .join("\n");
        assert!(Passport::custom_deserialize(&input).is_none());
    }

    #[test]
    #[ignore]
    fn example_p1() {
        assert_eq!(count_valid(&INPUT.to_vec()), 2);
    }

    #[test]
    fn valid_pid() {
        assert!(Passport::parse_pid("000000001").is_some());
    }

    #[test]
    fn invalid_pid() {
        assert!(Passport::parse_pid("0123456789").is_none());
    }

    #[test]
    fn all_valids_p2() {
        let input = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];
        assert_eq!(count_valid(&input.to_vec()), 4);
    }
    #[test]
    fn all_invalids_p2() {
        let input = [
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ];
        assert_eq!(count_valid(&input.to_vec()), 0);
    }
}
