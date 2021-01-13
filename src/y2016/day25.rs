use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let part1 = (0..).find(|&i| run(&instructions, i)).unwrap();

    (part1, "")
}

fn run(instructions: &[Instruction], i: i64) -> bool {
    let mut instructions = instructions.to_vec();
    let mut pc = 0;
    let mut registers = [0; 4];
    registers[0] = i;
    let mut expecting_1 = false;
    let mut count = 0;
    loop {
        if count >= 1000 {
            return true;
        }
        if pc >= instructions.len() {
            break;
        }
        let instruction = &instructions[pc];
        match instruction {
            Instruction::Copy { source, dest } => {
                if let Data::Register(dest) = dest {
                    let value = match source {
                        Data::Value(v) => *v,
                        Data::Register(i) => registers[*i],
                    };
                    registers[*dest] = value;
                }
                pc += 1;
            }
            Instruction::Increment { register } => {
                if let Data::Register(register) = register {
                    registers[*register] += 1;
                }
                pc += 1;
            }
            Instruction::Decrement { register } => {
                if let Data::Register(register) = register {
                    registers[*register] -= 1;
                }
                pc += 1;
            }
            Instruction::JumpNZ { source, offset } => {
                let offset = match offset {
                    Data::Value(v) => *v,
                    Data::Register(i) => registers[*i],
                };
                let value = match source {
                    Data::Value(v) => *v,
                    Data::Register(i) => registers[*i],
                };
                if value != 0 {
                    pc = (pc as i64 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::Toggle { offset } => {
                let offset = match offset {
                    Data::Value(v) => *v,
                    Data::Register(i) => registers[*i],
                };
                let offset = pc as i64 + offset;
                if offset >= 0 && offset < instructions.len() as i64 {
                    let instr_ref = &mut instructions[offset as usize];
                    *instr_ref = match std::mem::replace(
                        instr_ref,
                        Instruction::Toggle {
                            offset: Data::Value(0),
                        },
                    ) {
                        Instruction::Copy { source, dest } => Instruction::JumpNZ {
                            source,
                            offset: dest,
                        },
                        Instruction::Increment { register } => Instruction::Decrement { register },
                        Instruction::Decrement { register } => Instruction::Increment { register },
                        Instruction::JumpNZ { source, offset } => Instruction::Copy {
                            source,
                            dest: offset,
                        },
                        Instruction::Toggle { offset } => {
                            Instruction::Increment { register: offset }
                        }
                        Instruction::Out { x } => Instruction::Increment { register: x },
                    };
                }
                pc += 1;
            }
            Instruction::Out { x } => {
                let value = match x {
                    Data::Value(v) => *v,
                    Data::Register(i) => registers[*i],
                };
                match value {
                    0 => {
                        if expecting_1 {
                            return false;
                        } else {
                            expecting_1 = true;
                            count += 1;
                        }
                    }
                    1 => {
                        if !expecting_1 {
                            return false;
                        } else {
                            expecting_1 = false;
                            count += 1;
                        }
                    }
                    _ => {
                        return false;
                    }
                }

                pc += 1;
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
enum Instruction {
    Copy { source: Data, dest: Data },
    Increment { register: Data },
    Decrement { register: Data },
    JumpNZ { source: Data, offset: Data },
    Toggle { offset: Data },
    Out { x: Data },
}

#[derive(Debug, Clone)]
enum Data {
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
                dest: parts[2].parse()?,
            },
            "inc" => Instruction::Increment {
                register: parts[1].parse()?,
            },
            "dec" => Instruction::Decrement {
                register: parts[1].parse()?,
            },
            "jnz" => Instruction::JumpNZ {
                source: parts[1].parse()?,
                offset: parts[2].parse()?,
            },
            "tgl" => Instruction::Toggle {
                offset: parts[1].parse()?,
            },
            "out" => Instruction::Out {
                x: parts[1].parse()?,
            },
            _ => unreachable!(),
        })
    }
}

impl FromStr for Data {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        if ('a'..='z').contains(&c) {
            Ok(Data::Register(c as usize - 'a' as usize))
        } else {
            Ok(Data::Value(s.parse()?))
        }
    }
}
