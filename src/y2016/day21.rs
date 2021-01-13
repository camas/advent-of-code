use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    // Part 1: Scramble
    let mut data = "abcdefgh".chars().collect::<Vec<_>>();
    let data_len = data.len();
    for instr in instructions.iter() {
        match instr {
            Instruction::SwapPos { x, y } => {
                data.swap(*x, *y);
            }
            Instruction::SwapChar { x, y } => {
                let x_pos = data.iter().enumerate().find(|(_, c)| *c == x).unwrap().0;
                let y_pos = data.iter().enumerate().find(|(_, c)| *c == y).unwrap().0;
                data.swap(x_pos, y_pos);
            }
            Instruction::RotateLeft { amount } => {
                data.rotate_left(*amount);
            }
            Instruction::RotateRight { amount } => {
                data.rotate_right(*amount);
            }
            Instruction::RotateOnPosition { x } => {
                let x_pos = data.iter().enumerate().find(|(_, c)| *c == x).unwrap().0;
                let rot_amount = 1 + x_pos + if x_pos >= 4 { 1 } else { 0 };
                data.rotate_right(rot_amount % data_len);
            }
            Instruction::Reverse { x, y } => {
                data[*x..(*y + 1)].reverse();
            }
            Instruction::Move { x, y } => {
                let c = data.remove(*x);
                data.insert(*y, c);
            }
        }
    }
    let part1 = data.into_iter().collect::<String>();

    // Part 2: Unsramble
    let mut data = "fbgdceah".chars().collect::<Vec<_>>();
    let data_len = data.len();
    assert_eq!(data_len, 8);
    for instr in instructions.iter().rev() {
        match instr {
            Instruction::SwapPos { x, y } => {
                data.swap(*x, *y);
            }
            Instruction::SwapChar { x, y } => {
                let x_pos = data.iter().enumerate().find(|(_, c)| *c == x).unwrap().0;
                let y_pos = data.iter().enumerate().find(|(_, c)| *c == y).unwrap().0;
                data.swap(x_pos, y_pos);
            }
            Instruction::RotateLeft { amount } => {
                data.rotate_right(*amount);
            }
            Instruction::RotateRight { amount } => {
                data.rotate_left(*amount);
            }
            Instruction::RotateOnPosition { x } => {
                // original 0 1 2 3 4 5 6 7
                // new      1 3 5 7 2 4 6 0
                // rotation 1 2 3 4 6 7 0 1
                let x_pos = data.iter().enumerate().find(|(_, c)| *c == x).unwrap().0;
                let rot_amount = match x_pos {
                    1 => 1,
                    3 => 2,
                    5 => 3,
                    7 => 4,
                    2 => 6,
                    4 => 7,
                    6 => 0,
                    0 => 1,
                    _ => unreachable!(),
                };
                data.rotate_left(rot_amount);
            }
            Instruction::Reverse { x, y } => {
                data[*x..(*y + 1)].reverse();
            }
            Instruction::Move { x, y } => {
                let c = data.remove(*y);
                data.insert(*x, c);
            }
        }
    }
    let part2 = data.into_iter().collect::<String>();

    (part1, part2)
}

#[derive(Debug)]
enum Instruction {
    SwapPos { x: usize, y: usize },
    SwapChar { x: char, y: char },
    RotateLeft { amount: usize },
    RotateRight { amount: usize },
    RotateOnPosition { x: char },
    Reverse { x: usize, y: usize },
    Move { x: usize, y: usize },
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        Ok(match (parts[0], parts[1]) {
            ("swap", "position") => {
                let x = parts[2].parse()?;
                let y = parts[5].parse()?;
                Instruction::SwapPos { x, y }
            }
            ("swap", "letter") => {
                let x = parts[2].chars().next().unwrap();
                let y = parts[5].chars().next().unwrap();
                Instruction::SwapChar { x, y }
            }
            ("rotate", "left") => {
                let amount = parts[2].parse()?;
                Instruction::RotateLeft { amount }
            }
            ("rotate", "right") => {
                let amount = parts[2].parse()?;
                Instruction::RotateRight { amount }
            }
            ("rotate", "based") => {
                let x = parts[6].chars().next().unwrap();
                Instruction::RotateOnPosition { x }
            }
            ("reverse", _) => {
                let x = parts[2].parse()?;
                let y = parts[4].parse()?;
                Instruction::Reverse { x, y }
            }
            ("move", _) => {
                let x = parts[2].parse()?;
                let y = parts[5].parse()?;
                Instruction::Move { x, y }
            }
            _ => unreachable!(),
        })
    }
}
