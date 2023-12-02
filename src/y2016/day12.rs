use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let part1 = run(&instructions, false);
    let part2 = run(&instructions, true);

    (part1, part2)
}

fn run(instructions: &[Instruction], part2: bool) -> i64 {
    let mut pc = 0;
    let mut registers = [0; 4];
    if part2 {
        registers[2] = 1;
    }
    loop {
        if pc >= instructions.len() {
            break;
        }
        let instruction = &instructions[pc];
        match instruction {
            Instruction::Copy { source, dest } => {
                let value = match source {
                    Source::Value(v) => *v,
                    Source::Register(i) => registers[*i],
                };
                registers[*dest] = value;
                pc += 1;
            }
            Instruction::Increment { register } => {
                registers[*register] += 1;
                pc += 1;
            }
            Instruction::Decrement { register } => {
                registers[*register] -= 1;
                pc += 1;
            }
            Instruction::JumpNZ { source, offset } => {
                let value = match source {
                    Source::Value(v) => *v,
                    Source::Register(i) => registers[*i],
                };
                if value != 0 {
                    pc = (pc as i64 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
        }
    }
    registers[0]
}

#[derive(Debug)]
enum Instruction {
    Copy { source: Source, dest: usize },
    Increment { register: usize },
    Decrement { register: usize },
    JumpNZ { source: Source, offset: i64 },
}

#[derive(Debug)]
enum Source {
    Value(i64),
    Register(usize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        Ok(match parts[0] {
            "cpy" => Instruction::Copy {
                source: parts[1].parse()?,
                dest: parts[2].chars().next().unwrap() as usize - 'a' as usize,
            },
            "inc" => Instruction::Increment {
                register: parts[1].chars().next().unwrap() as usize - 'a' as usize,
            },
            "dec" => Instruction::Decrement {
                register: parts[1].chars().next().unwrap() as usize - 'a' as usize,
            },
            "jnz" => Instruction::JumpNZ {
                source: parts[1].parse()?,
                offset: parts[2].parse()?,
            },
            _ => unreachable!(),
        })
    }
}

impl FromStr for Source {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        Ok(if c.is_ascii_lowercase() {
            Source::Register(c as usize - 'a' as usize)
        } else {
            Source::Value(s.parse()?)
        })
    }
}
