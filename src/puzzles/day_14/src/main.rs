use std::collections::HashMap;

use regex::Regex;
use utils::read_lines;

pub enum Instruction {
    Mask { zero: u64, one: u64, floating: u64 },
    Write { address: u64, value: u64 },
}

type Program = Vec<Instruction>;

#[derive(PartialEq, Eq, Debug)]
pub struct Machine {
    pub memory: HashMap<u64, u64>,
    pub mask_0: u64,
    pub mask_1: u64,
    pub mask_x: u64,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            memory: HashMap::new(),
            mask_0: 0,
            mask_1: 0,
            mask_x: 0,
        }
    }

    fn write_floating_address(&mut self, address: &u64, value: &u64, bit_index: usize) {
        let bit_mask = 1 << bit_index;
        if self.mask_x & bit_mask != 0 {
            [address & !bit_mask, address | bit_mask]
                .iter()
                .for_each(|address| {
                    self.memory.insert(*address, *value);
                    self.write_floating_address(address, value, bit_index + 1);
                });
        } else if self.mask_x >> bit_index != 0 {
            self.write_floating_address(address, value, bit_index + 1)
        }
    }

    pub fn run(&mut self, prog: &Program) {
        prog.iter().for_each(|instruction| match instruction {
            &Instruction::Mask {
                one,
                zero,
                floating,
            } => {
                self.mask_0 = zero;
                self.mask_1 = one;
                self.mask_x = floating;
            }
            &Instruction::Write { address, value } => {
                let mem_val = self.memory.entry(address).or_insert(0);
                *mem_val = value & self.mask_0 | self.mask_1;
            }
        });
    }

    pub fn run_v2(&mut self, prog: &Program) {
        prog.iter().for_each(|instruction| match instruction {
            &Instruction::Mask {
                one,
                zero,
                floating,
            } => {
                self.mask_0 = zero;
                self.mask_1 = one;
                self.mask_x = floating;
            }
            &Instruction::Write { address, value } => {
                let address = address & !(self.mask_x) | self.mask_1;
                self.write_floating_address(&address, &value, 0);
            }
        });
    }
}

pub fn solve_p1(prog: &Program) -> u64 {
    let mut machine = Machine::new();
    machine.run(&prog);
    machine.memory.values().sum()
}

pub fn solve_p2(prog: &Program) -> u64 {
    let mut machine = Machine::new();
    machine.run_v2(&prog);
    machine.memory.values().sum()
}

pub fn parse_input<T: AsRef<str>>(lines: &Vec<T>) -> Program {
    let re_mask = Regex::new(r"^mask = (.*)$").unwrap();
    let re_write = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    lines
        .iter()
        .map(|line| {
            if let Some(cap) = re_mask.captures_iter(line.as_ref()).next() {
                let mask = &cap[1];

                let zero = mask.replace("X", "1");
                let zero = isize::from_str_radix(&zero[..], 2).unwrap() as u64;

                let one = mask.replace("X", "0");
                let one = isize::from_str_radix(&one[..], 2).unwrap() as u64;

                let floating = mask.replace("1", "0").replace("X", "1");
                let floating = isize::from_str_radix(&floating[..], 2).unwrap() as u64;

                Instruction::Mask {
                    zero,
                    one,
                    floating,
                }
            } else if let Some(cap) = re_write.captures_iter(line.as_ref()).next() {
                let address = *&cap[1].parse::<u64>().unwrap();
                let value = *&cap[2].parse::<u64>().unwrap();

                Instruction::Write { address, value }
            } else {
                panic!("Invalid instruction")
            }
        })
        .collect()
}

fn main() {
    let lines = read_lines("src/inputs/day_14.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let program: Program = parse_input(&lines);

    println!("Part 1: {:?}", solve_p1(&program));
    println!("Part 2: {:?}", solve_p2(&program));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let input = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];
        let program = parse_input(&input);

        assert_eq!(solve_p1(&program), 165)
    }

    #[test]
    fn example_p2() {
        let input = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ];

        let program = parse_input(&input);

        assert_eq!(solve_p2(&program), 208)
    }
}
