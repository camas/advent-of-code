use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap());

    let mut screen = [[false; 50]; 6];
    for instruction in instructions {
        match instruction {
            Instruction::Rect { width, height } => {
                screen[..height]
                    .iter_mut()
                    .for_each(|row| row[..width].iter_mut().for_each(|v| *v = true));
            }
            Instruction::RotateRow { row, amount } => {
                screen[row].rotate_right(amount);
            }
            Instruction::RotateColumn { column, amount } => {
                let mut values = screen.iter().map(|row| row[column]).collect::<Vec<_>>();
                values.rotate_right(amount);
                values
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, v)| screen[i][column] = v);
            }
        }
    }

    let part1 = screen
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum::<usize>();

    let part2 = (0..screen[0].len())
        .step_by(5)
        .map(|x_offset| {
            let code = screen
                .iter()
                .flat_map(|row| row[x_offset..(x_offset + 5)].iter())
                .enumerate()
                .fold(0, |acc, (i, b)| acc + ((*b as u32) << i));
            match code {
                0 => ' ',
                504405039 => 'E',
                211068198 => 'O',
                311928102 => 'A',
                307471655 => 'R',
                479626534 => 'G',
                34841895 => 'P',
                311737641 => 'H',
                138553905 => 'Y',
                other => {
                    println!("Unknown code: {}", other);
                    screen.iter().for_each(|row| {
                        println!(
                            "{}",
                            row[x_offset..(x_offset + 5)]
                                .iter()
                                .map(|b| if *b { '#' } else { '.' })
                                .collect::<String>()
                        );
                    });
                    panic!();
                }
            }
        })
        .collect::<String>();

    (part1, part2)
}

#[derive(Debug)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, amount: usize },
    RotateColumn { column: usize, amount: usize },
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("rect") {
            let mut parts = s[5..].split('x');
            let width = parts.next().unwrap().parse()?;
            let height = parts.next().unwrap().parse()?;
            Ok(Instruction::Rect { width, height })
        } else if s.starts_with("rotate row") {
            let mut parts = s[13..].split(" by ");
            let row = parts.next().unwrap().parse()?;
            let amount = parts.next().unwrap().parse()?;
            Ok(Instruction::RotateRow { row, amount })
        } else if s.starts_with("rotate column") {
            let mut parts = s[16..].split(" by ");
            let column = parts.next().unwrap().parse()?;
            let amount = parts.next().unwrap().parse()?;
            Ok(Instruction::RotateColumn { column, amount })
        } else {
            unreachable!()
        }
    }
}
