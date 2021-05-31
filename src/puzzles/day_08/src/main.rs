use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FinishCondition {
    Correctly,
    InfiniteLoop,
    Overflow
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    ip: usize,
    acc: i32,
    lines_executed: Vec<usize>,
}

impl Program {
    pub fn run_safe(&mut self) -> (i32, FinishCondition) {
        if self.lines_executed.contains(&self.ip) {
            (self.acc, FinishCondition::InfiniteLoop)
        } else if self.lines_executed.len() == self.instructions.len() {
            (self.acc, FinishCondition::Correctly)
        } else if self.ip >= self.instructions.len() {
            (self.acc, FinishCondition::Overflow)
        } else {
            self.lines_executed.push(self.ip);
            match self.instructions[self.ip] {
                Instruction::Acc(v) => {
                    self.acc += v;
                    self.ip += 1;
                }
                Instruction::Jmp(v) => {
                    self.ip = ((self.ip as i32) + v) as usize;
                }
                Instruction::Nop(_) => {
                    self.ip += 1;
                }
            };
            self.run_safe()
        }
    }

    pub fn replace_instruction(&mut self, i: usize) {
        match self.instructions[i] {
            Instruction::Nop(val) => self.instructions[i] = Instruction::Jmp(val),
            Instruction::Jmp(val) => self.instructions[i] = Instruction::Nop(val),
            _ => {}
        }
    }
}

impl<T: AsRef<str>> From<T> for Program {
    fn from(lines: T) -> Self {
        let instructions = lines
            .as_ref()
            .split("\n")
            .map(|line| Instruction::from(line))
            .collect::<Vec<_>>();

        Program {
            instructions,
            ip: 0,
            acc: 0,
            lines_executed: Vec::new(),
        }
    }
}

impl<T: AsRef<str>> From<T> for Instruction {
    fn from(line: T) -> Self {
        let mut iter = line.as_ref().split(" ");
        let code = iter.next().unwrap();

        match code {
            "acc" => {
                let val = iter.next().unwrap().parse::<i32>().unwrap();
                Instruction::Acc(val)
            }
            "jmp" => {
                let val = iter.next().unwrap().parse::<i32>().unwrap();
                Instruction::Jmp(val)
            }
            "nop" => {
                let val = iter.next().unwrap().parse::<i32>().unwrap();
                Instruction::Nop(val)
            }
            _ => panic!("Invalid input"),
        }
    }
}

fn main() {
    let lines = read_lines("src/inputs/day_08.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let mut program = Program::from(lines.join("\n"));
    println!("Part 1: {:?}", program.run_safe());

    let program = Program::from(lines.join("\n"));
    let result = program
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, v)| match v {
            Instruction::Acc(_) => false,
            _ => true,
        })
        .map(|(i, _)| {
            let mut p = program.clone();
            p.replace_instruction(i);
            p.run_safe()
        })
        .filter(|(_, condition)| *condition != FinishCondition::InfiniteLoop)
        .collect::<Vec<_>>();
    println!("Part 2: {:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let input = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .join("\n");

        let mut program = Program::from(input);
        assert_eq!(program.run_safe(), (5, FinishCondition::InfiniteLoop));
    }
}
