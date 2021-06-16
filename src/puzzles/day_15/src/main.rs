use std::collections::HashMap;

pub struct Game {
    turn_number: usize,
    last_value: usize,
    records: HashMap<usize, usize>,
}

impl Game {
    fn new(vals: &Vec<usize>) -> Self {
        Game {
            turn_number: vals.len() as usize,
            last_value: *vals.iter().last().unwrap() as usize,
            records: vals
                .iter()
                .enumerate()
                .map(|(i, v)| (*v as usize, i + 1))
                .collect(),
        }
    }

    pub fn play_to_turn(&mut self, target_turn: usize) {
        let initial = self.turn_number;
        for i in initial..target_turn {
            let second_last_turn = self.records.get(&self.last_value);

            let new_value = match second_last_turn {
                Some(second_pos) => i - *second_pos,
                None => 0,
            };

            self.records.insert(self.last_value, i);
            self.last_value = new_value as usize;
        }
    }
}

pub fn solve_p1(vals: &Vec<usize>) -> usize {
    let mut game = Game::new(vals);
    game.play_to_turn(2020);
    game.last_value
}

pub fn solve_p2(vals: &Vec<usize>) -> usize {
    let mut game = Game::new(vals);
    game.play_to_turn(30000000);
    game.last_value
}

fn main() {
    let input = vec![0, 6, 1, 7, 2, 19, 20];
    println!("Part 1: {:?}", solve_p1(&input));
    println!("Part 2: {:?}", solve_p2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        assert_eq!(solve_p1(&vec![1, 3, 2]), 1);
        assert_eq!(solve_p1(&vec![2, 1, 3]), 10);
        assert_eq!(solve_p1(&vec![1, 2, 3]), 27);
        assert_eq!(solve_p1(&vec![2, 3, 1]), 78);
        assert_eq!(solve_p1(&vec![3, 2, 1]), 438);
        assert_eq!(solve_p1(&vec![3, 1, 2]), 1836);
    }

    #[test]
    #[ignore]
    fn example_p2() {
        assert_eq!(solve_p2(&vec![0, 3, 6]), 175594);
        assert_eq!(solve_p2(&vec![1, 3, 2]), 2578);
        assert_eq!(solve_p2(&vec![2, 1, 3]), 3544142);
        assert_eq!(solve_p2(&vec![1, 2, 3]), 261214);
        assert_eq!(solve_p2(&vec![2, 3, 1]), 6895259);
        assert_eq!(solve_p2(&vec![3, 2, 1]), 18);
        assert_eq!(solve_p2(&vec![3, 1, 2]), 362);
    }
}
