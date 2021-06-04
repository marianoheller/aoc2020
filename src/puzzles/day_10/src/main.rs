use std::collections::HashMap;

use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Connections {
    value: i32,
    connected_to: Vec<Connections>,
}

impl Connections {
    pub fn new(vals: Vec<i32>) -> Self {
        fn go(prev: i32, list: &Vec<i32>) -> Vec<Connections> {
            let (ok_list, rest) =
                list.iter()
                    .fold((vec![], vec![]), |(mut ok_list, mut rest), v| {
                        let diff = *v - prev;
                        let is_ok = diff <= 3 && diff > 0;
                        if is_ok {
                            ok_list.push(*v);
                        } else {
                            rest.push(*v);
                        }
                        (ok_list, rest)
                    });

            ok_list
                .iter()
                .map(|value| {
                    let mut other_ok = ok_list
                        .iter()
                        .filter_map(|v| if *v > *value { Some(*v) } else { None })
                        .collect::<Vec<_>>();
                    let mut local_rest = rest.clone();
                    other_ok.append(&mut local_rest);

                    Connections {
                        value: *value,
                        connected_to: go(*value, &other_ok),
                    }
                })
                .collect()
        }

        Connections {
            value: 0,
            connected_to: go(0, &vals),
        }
    }

    pub fn from_str<T: AsRef<str>>(s: T) -> Connections {
        let mut vals = s
            .as_ref()
            .split("\n")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        vals.sort();
        Connections::new(vals)
    }

    pub fn iterations(&mut self) -> Vec<Vec<i32>> {
        let conns = self.connected_to.clone();
        conns
            .into_iter()
            .flat_map(|mut conn| {
                if conn.connected_to.is_empty() {
                    vec![vec![self.value, conn.value]]
                } else {
                    conn.iterations()
                        .into_iter()
                        .map(|mut v| {
                            v.insert(0, self.value);
                            v
                        })
                        .collect::<Vec<Vec<_>>>()
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn longest(&mut self) -> Option<Vec<i32>> {
        self.iterations()
            .into_iter()
            .fold(None, |acc, conn| match acc {
                None => Some(conn),
                Some(max) => {
                    if max.len() >= conn.len() {
                        Some(max)
                    } else {
                        Some(conn)
                    }
                }
            })
    }
}

pub fn solve_p1<T: AsRef<str>>(input: T) -> i32 {
    let mut vals = Connections::from_str(input).longest().unwrap();
    let last_val = vals[vals.len() - 1];
    vals.push(last_val + 3);

    let result = vals
        .iter()
        .enumerate()
        .map(|(i, v)| if i == 0 { *v } else { *v - vals[i - 1] }) //.collect::<Vec<_>>();
        .fold(HashMap::new(), |mut acc, v| {
            let counter = acc.entry(v).or_insert(0);
            *counter += 1;
            acc
        });

    result.get(&1).unwrap() * result.get(&3).unwrap()
}

fn main() {
    let lines = read_lines("src/inputs/day_10.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let result1 = solve_p1(lines.join("\n"));
    println!("Part 1: {:?}", result1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1_a() {
        let input = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"].join("\n");
        assert_eq!(solve_p1(input), 35);
    }

    #[test]
    fn example_p1_b() {
        let input = vec![
            "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
            "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
            "10", "3",
        ]
        .join("\n");
        assert_eq!(solve_p1(input), 220);
    }
}
