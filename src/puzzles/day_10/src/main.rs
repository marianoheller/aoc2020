use std::{cmp::max, collections::HashMap};

use utils::read_lines;

pub fn solve_p1(_adapters: &Vec<i64>) -> i64 {
    let mut adapters = _adapters.clone();
    let last_val = adapters[adapters.len() - 1];
    adapters.push(last_val + 3);

    let result = adapters
        .iter()
        .enumerate()
        .map(|(i, v)| if i == 0 { *v } else { *v - adapters[i - 1] })
        .fold(HashMap::new(), |mut acc, v| {
            let counter = acc.entry(v).or_insert(0);
            *counter += 1;
            acc
        });
    result.get(&1).unwrap() * result.get(&3).unwrap()
}

pub fn solve_p2(_adapters: &Vec<i64>) -> i64 {
    let mut adapters = _adapters.clone();
    adapters.insert(0, 0);

    let mut valid_arrangements: Vec<i64> = vec![];
    valid_arrangements.resize(adapters.len() - 1, 0);
    valid_arrangements.insert(0, 1);

    for index in 1..adapters.len() {
        for src_index in max(0, (index as i64) - 3) as usize..index {
            if adapters[index] - adapters[src_index] <= 3 {
                valid_arrangements[index] += valid_arrangements[src_index]
            }
        }
    }

    valid_arrangements[valid_arrangements.len() - 1]
}

fn main() {
    let lines = read_lines("src/inputs/day_10.txt").unwrap().flatten();

    let mut adapters = lines.map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
    adapters.sort();

    println!("Part 1: {:?}", solve_p1(&adapters));
    println!("Part 2: {:?}", solve_p2(&adapters));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1_a() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();

        assert_eq!(solve_p1(&adapters), 35);
    }

    #[test]
    fn example_p1_b() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        adapters.sort();

        assert_eq!(solve_p1(&adapters), 220);
    }

    #[test]
    fn example_p2_a() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();

        assert_eq!(solve_p2(&adapters), 8);
    }

    #[test]
    fn example_p2_b() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        adapters.sort();

        assert_eq!(solve_p2(&adapters), 19208);
    }
}
