use std::str::FromStr;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Instruction>>();
    let mut mul_invoked = 0_u32;
    let mut state = State::init();
    loop {
        if state.pc < 0 || state.pc >= instructions.len() as i64 {
            break;
        }
        let cur_instr = &instructions[state.pc as usize];
        match cur_instr {
            Instruction::Set { register, value } => {
                let value = state.get_value(value);
                state.set_register(*register, value);
                state.pc += 1;
            }
            Instruction::Subtract { register, value } => {
                let value = state.get_value(value);
                let original = state.get_register(*register);
                state.set_register(*register, original - value);
                state.pc += 1;
            }
            Instruction::Multiply { register, value } => {
                let value = state.get_value(value);
                let original = state.get_register(*register);
                state.set_register(*register, original * value);
                state.pc += 1;

                mul_invoked += 1;
            }
            Instruction::JumpNZ { value_a, value_b } => {
                let value_b = state.get_value(value_b);
                if state.get_value(value_a) != 0 {
                    state.pc += value_b;
                } else {
                    state.pc += 1;
                }
            }
        }
    }
    let part1 = mul_invoked;

    let lower = 100
        * match &instructions[0] {
            Instruction::Set { value, .. } => match value {
                Source::Constant(value) => value,
                _ => panic!(),
            },
            _ => panic!(),
        };
    let lower = lower
        - match &instructions[5] {
            Instruction::Subtract { value, .. } => match value {
                Source::Constant(value) => value,
                _ => panic!(),
            },
            _ => panic!(),
        };
    let upper = lower
        - match &instructions[7] {
            Instruction::Subtract { value, .. } => match value {
                Source::Constant(value) => value,
                _ => panic!(),
            },
            _ => panic!(),
        };

    let part2 = (lower..=upper)
        .step_by(17)
        .filter(|i| (2..=(i / 2)).any(|factor| i % factor == 0))
        .count();

    (part1, part2)
}

#[derive(Debug)]
struct State {
    registers: Vec<i64>,
    pc: i64,
}

impl State {
    fn init() -> Self {
        State {
            registers: vec![0; 8],
            pc: 0,
        }
    }

    fn get_value(&self, source: &Source) -> i64 {
        match source {
            Source::Register(c) => self.registers[(*c as u8 - b'a') as usize],
            Source::Constant(value) => *value,
        }
    }

    fn set_register(&mut self, register: char, value: i64) {
        self.registers[(register as u8 - b'a') as usize] = value;
    }

    fn get_register(&mut self, register: char) -> i64 {
        self.registers[(register as u8 - b'a') as usize]
    }
}

#[derive(Debug)]
enum Instruction {
    Set { register: char, value: Source },
    Subtract { register: char, value: Source },
    Multiply { register: char, value: Source },
    JumpNZ { value_a: Source, value_b: Source },
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let register = parts[1].chars().next().unwrap();
        let value = parts[2].parse()?;
        Ok(match parts[0] {
            "set" => Instruction::Set { register, value },
            "sub" => Instruction::Subtract { register, value },
            "mul" => Instruction::Multiply { register, value },
            "jnz" => Instruction::JumpNZ {
                value_a: parts[1].parse()?,
                value_b: value,
            },
            _ => panic!(),
        })
    }
}

#[derive(Debug)]
enum Source {
    Register(char),
    Constant(i64),
}

impl FromStr for Source {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        Ok(match c {
            'a'..='h' => Source::Register(c),
            _ => Source::Constant(s.parse()?),
        })
    }
}
