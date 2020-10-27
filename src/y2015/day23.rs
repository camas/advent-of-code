use std::{num::ParseIntError, str::FromStr};

use crate::Exercise;

pub struct Day23;

impl Exercise for Day23 {
    fn part1(&self, input: &str) -> String {
        let mut computer: Computer = input.parse().unwrap();
        while computer.step() {}
        computer.state.b.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut computer: Computer = input.parse().unwrap();
        computer.state.a = 1;
        while computer.step() {}
        computer.state.b.to_string()
    }
}

struct Computer {
    instructions: Vec<Instruction>,
    state: ComputerState,
}

impl Computer {
    pub fn step(&mut self) -> bool {
        if self.state.instruction_pointer < 0
            || self.state.instruction_pointer >= self.instructions.len() as i32
        {
            return false;
        }
        self.state
            .step(&self.instructions[self.state.instruction_pointer as usize])
    }
}

#[derive(Debug)]
struct ComputerState {
    a: u32,
    b: u32,
    instruction_pointer: i32,
}

impl ComputerState {
    /// Returns false if pointer doesn't point to an instruction
    pub fn step(&mut self, instruction: &Instruction) -> bool {
        // println!("{:?}", instruction);
        match instruction {
            Instruction::Half(r) => *self.get_register(r) /= 2,
            Instruction::Triple(r) => *self.get_register(r) *= 3,
            Instruction::Increment(r) => *self.get_register(r) += 1,
            Instruction::Jump(offset) => self.instruction_pointer += offset,
            Instruction::JumpIfEven(r, offset) => {
                if *self.get_register(r) % 2 == 0 {
                    self.instruction_pointer += offset;
                } else {
                    self.instruction_pointer += 1;
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                if *self.get_register(r) == 1 {
                    self.instruction_pointer += offset;
                } else {
                    self.instruction_pointer += 1;
                }
            }
        }
        match instruction {
            Instruction::Half(_) | Instruction::Triple(_) | Instruction::Increment(_) => {
                self.instruction_pointer += 1
            }
            _ => (),
        }
        // println!("{:?}", self);
        true
    }

    pub fn get_register(&mut self, register: &Register) -> &mut u32 {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

impl FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self {
            instructions,
            state: ComputerState {
                a: 0,
                b: 0,
                instruction_pointer: 0,
            },
        })
    }
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = &s[..3];
        let values = s[4..].split(", ").collect::<Vec<_>>();
        Ok(match name {
            "hlf" => Instruction::Half(values[0].parse()?),
            "tpl" => Instruction::Triple(values[0].parse()?),
            "inc" => Instruction::Increment(values[0].parse()?),
            "jmp" => Instruction::Jump(values[0].parse()?),
            "jie" => Instruction::JumpIfEven(values[0].parse()?, values[1].parse()?),
            "jio" => Instruction::JumpIfOne(values[0].parse()?, values[1].parse()?),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        })
    }
}
