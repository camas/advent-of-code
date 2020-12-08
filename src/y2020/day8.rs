use std::{collections::HashSet, iter, num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    // Part 1: Run until an instruction is about to be executed for a second time
    let mut machine = Machine::new(&instructions);
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(machine.instruction_pointer) {
            break;
        }
        machine.step();
    }
    let part1 = machine.accumulator;

    // Part 2: Change one instruction from jump -> nop or nop -> jump.
    // Correct program will try and execute instruction one past end
    let mut part2 = 0;
    'main: for i in 0..instructions.len() {
        let new_instruction = if let Instruction::Jump(value) = instructions[i] {
            Instruction::NoOperation(value)
        } else if let Instruction::NoOperation(value) = instructions[i] {
            Instruction::Jump(value)
        } else {
            continue;
        };

        let patched_instructions = instructions[0..i]
            .iter()
            .chain(iter::once(&new_instruction))
            .chain(instructions[(i + 1)..].iter())
            .cloned()
            .collect::<Vec<_>>();
        let mut machine = Machine::new(&patched_instructions);
        let mut visited = HashSet::new();
        loop {
            if machine.instruction_pointer == machine.instructions.len() {
                part2 = machine.accumulator;
                break 'main;
            }
            if !visited.insert(machine.instruction_pointer) {
                break;
            }
            machine.step();
        }
    }

    (part1, part2)
}

#[derive(Debug)]
struct Machine<'a> {
    instruction_pointer: usize,
    accumulator: i64,
    instructions: &'a [Instruction],
}

impl<'a> Machine<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instruction_pointer: 0,
            accumulator: 0,
            instructions,
        }
    }

    pub fn step(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::Accumulate(value) => {
                self.accumulator += value;
                self.instruction_pointer += 1;
            }
            Instruction::Jump(value) => {
                self.instruction_pointer = self.instruction_pointer.wrapping_add(value as usize);
            }
            Instruction::NoOperation(_) => {
                self.instruction_pointer += 1;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Accumulate(i64),
    Jump(i64),
    NoOperation(i64),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let name = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i64>()?;
        Ok(match name {
            "acc" => Instruction::Accumulate(value),
            "jmp" => Instruction::Jump(value),
            "nop" => Instruction::NoOperation(value),
            _ => unreachable!(),
        })
    }
}
