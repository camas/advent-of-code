use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let mut state = ShipState {
        x: 0,
        y: 0,
        dir: 90,
        waypoint_x: 0,
        waypoint_y: 0,
    };
    for instruction in instructions.iter() {
        state.step(instruction);
    }
    let part1 = state.x.abs() + state.y.abs();

    let mut state = ShipState {
        x: 0,
        y: 0,
        dir: 90,
        waypoint_x: 10,
        waypoint_y: 1,
    };
    for instruction in instructions.iter() {
        state.step2(instruction);
    }
    let part2 = state.x.abs() + state.y.abs();

    (part1, part2)
}

struct ShipState {
    x: i64,
    y: i64,
    dir: i64,
    waypoint_x: i64,
    waypoint_y: i64,
}

impl ShipState {
    fn step(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => self.y += value,
            Instruction::South(value) => self.y -= value,
            Instruction::East(value) => self.x += value,
            Instruction::West(value) => self.x -= value,
            Instruction::Left(value) => self.dir = (self.dir - value).rem_euclid(360),
            Instruction::Right(value) => self.dir = (self.dir + value) % 360,
            Instruction::Forward(value) => match self.dir {
                0 => self.y += value,
                90 => self.x += value,
                180 => self.y -= value,
                270 => self.x -= value,
                _ => unreachable!(),
            },
        }
    }

    fn step2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => self.waypoint_y += value,
            Instruction::South(value) => self.waypoint_y -= value,
            Instruction::East(value) => self.waypoint_x += value,
            Instruction::West(value) => self.waypoint_x -= value,
            Instruction::Left(value) if *value == 90 => {
                let temp = self.waypoint_x;
                self.waypoint_x = -self.waypoint_y;
                self.waypoint_y = temp;
            }
            Instruction::Left(value) if *value == 180 => {
                self.waypoint_x = -self.waypoint_x;
                self.waypoint_y = -self.waypoint_y;
            }
            Instruction::Left(value) if *value == 270 => {
                let temp = self.waypoint_x;
                self.waypoint_x = self.waypoint_y;
                self.waypoint_y = -temp;
            }
            Instruction::Right(value) if *value == 90 => {
                let temp = self.waypoint_x;
                self.waypoint_x = self.waypoint_y;
                self.waypoint_y = -temp;
            }
            Instruction::Right(value) if *value == 180 => {
                self.waypoint_x = -self.waypoint_x;
                self.waypoint_y = -self.waypoint_y;
            }
            Instruction::Right(value) if *value == 270 => {
                let temp = self.waypoint_x;
                self.waypoint_x = -self.waypoint_y;
                self.waypoint_y = temp;
            }

            Instruction::Forward(value) => {
                self.x += value * self.waypoint_x;
                self.y += value * self.waypoint_y;
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    North(i64),
    East(i64),
    South(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse()?;
        Ok(match s.chars().next().unwrap() {
            'N' => Instruction::North(value),
            'E' => Instruction::East(value),
            'S' => Instruction::South(value),
            'W' => Instruction::West(value),
            'L' => Instruction::Left(value),
            'R' => Instruction::Right(value),
            'F' => Instruction::Forward(value),
            _ => unreachable!(),
        })
    }
}
