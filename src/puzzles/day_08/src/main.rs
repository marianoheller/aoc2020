use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}
#[derive(Debug, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
    ip: usize,
    acc: usize,
    lines_executed: Vec<usize>,
}

impl Program {
    pub fn run_safe(&mut self) -> usize {
        if self.lines_executed.contains(&self.ip) {
            self.acc
        } else {
            self.lines_executed.push(self.ip);
            match self.instructions[self.ip] {
                Instruction::Acc(v) => {
                    self.acc = ((self.acc as i32) + v) as usize;
                    self.ip += 1;
                }
                Instruction::Jmp(v) => {
                    self.ip = ((self.ip as i32) + v) as usize;
                }
                Instruction::Nop => {
                    self.ip += 1;
                }
            };
            self.run_safe()
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
            "nop" => Instruction::Nop,
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
        assert_eq!(program.run_safe(), 5);
    }
}
