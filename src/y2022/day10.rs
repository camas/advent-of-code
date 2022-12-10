use std::{num::ParseIntError, str::FromStr};

use crate::common::parse_letters;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let mut computer = Computer::new(instructions);
    let mut part1 = 0;
    let mut part2 = Vec::with_capacity(240);
    for i in 0..240 {
        let cycle = i + 1;
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            part1 += cycle * computer.x;
        }

        let sprite_pos = computer.x;
        let pixel_pos = i % 40;
        if ((sprite_pos - 1)..=(sprite_pos + 1)).contains(&pixel_pos) {
            part2.push(true);
        } else {
            part2.push(false);
        }

        computer.cycle();
    }

    let part2 = parse_letters(&part2.chunks(40).map(|c| c.to_vec()).collect::<Vec<_>>());

    (part1, part2)
}

#[derive(Debug)]
struct Computer {
    x: i64,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    current_instr_cycle: u64,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Computer {
            x: 1,
            instructions,
            instruction_pointer: 0,
            current_instr_cycle: 0,
        }
    }

    fn cycle(&mut self) {
        self.current_instr_cycle += 1;

        let instr = self.instructions[self.instruction_pointer];
        if self.current_instr_cycle != instr.run_time() {
            return;
        }

        match instr {
            Instruction::NoOp => (),
            Instruction::AddX(value) => self.x = self.x.checked_add(value).unwrap(),
        }
        self.instruction_pointer += 1;
        self.current_instr_cycle = 0;
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i64),
}

impl Instruction {
    fn run_time(&self) -> u64 {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(match parts.next().unwrap() {
            "noop" => Instruction::NoOp,
            "addx" => Instruction::AddX(parts.next().unwrap().parse()?),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 13140.to_string());
    }
}
