use utils::read_lines;

struct Seq {
    numbers: Vec<usize>,
    buff_len: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum ValidationResult {
    Valid,
    Invalid(usize),
    TooShort,
}

impl ValidationResult {
    fn invalid_value(self) -> usize {
        if let ValidationResult::Invalid(c) = self {
            c
        } else {
            panic!("Not an invalid validation result")
        }
    }
}

impl Seq {
    pub fn new(buff_len: usize) -> Self {
        Seq {
            numbers: vec![],
            buff_len,
        }
    }

    pub fn push(&mut self, v: usize) {
        self.numbers.push(v);
    }

    pub fn from_str<T: AsRef<str>>(str: T, len: usize) -> Self {
        let mut seq = Seq::new(len);
        str.as_ref().split("\n").for_each(|line| {
            seq.push(line.parse::<usize>().unwrap());
        });
        seq
    }

    pub fn validate(&self) -> ValidationResult {
        let total = self.numbers.len();
        if total < self.buff_len {
            ValidationResult::TooShort
        } else {
            (self.buff_len..total).fold(ValidationResult::Valid, |acc, i| {
                if acc != ValidationResult::Valid {
                    acc
                } else {
                    let nums = self.numbers[i - self.buff_len..i].to_vec();
                    let target = self.numbers[i];
                    let is_valid = nums.iter().any(|x| {
                        let mut result = false;
                        for y in &nums {
                            if *x != *y && x + y == target {
                                result = true
                            }
                        }
                        result
                    });

                    if is_valid {
                        ValidationResult::Valid
                    } else {
                        ValidationResult::Invalid(self.numbers[i])
                    }
                }
            })
        }
    }

    pub fn find_sum_set(&self, target: usize) -> Option<Vec<usize>> {
        let mut result = None;
        for n in 0..self.numbers.len() {
            if result.is_none() {
                let (sum, set) = self.numbers[n..self.numbers.len()].into_iter().fold(
                    (0, vec![]),
                    |(acc, mut set), v| {
                        if acc >= target {
                            (acc, set)
                        } else {
                            set.push(*v);
                            (acc + *v, set)
                        }
                    },
                );
                if sum == target && set.len() > 1 {
                    result = Some(set)
                }
            }
        }
        result
    }

    pub fn find_sum_set_result(&self, target: usize) -> Option<usize> {
        self.find_sum_set(target).and_then(|mut vec| {
            if vec.len() < 2 {
                None
            } else {
                vec.sort();
                Some(vec[0] + vec[vec.len() - 1])
            }
        })
    }
}

fn main() {
    let lines = read_lines("src/inputs/day_09.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let seq = Seq::from_str(lines.join("\n"), 25);
    let result1 = seq.validate();
    println!("Part 1: {:?}", result1);

    let invalid_value = ValidationResult::invalid_value(result1);
    println!("Part 2: {:?}", seq.find_sum_set_result(invalid_value));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let input = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .join("\n");

        let seq = Seq::from_str(input, 5);
        assert_eq!(seq.validate(), ValidationResult::Invalid(127));
    }

    #[test]
    fn example_p2() {
        let input = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .join("\n");

        let seq = Seq::from_str(input, 5);
        assert_eq!(seq.find_sum_set_result(127), Some(62));
    }
}
