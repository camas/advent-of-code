use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap());

    let mut values: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut moves = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Give { bot, value } => {
                values.entry(bot).or_default().push(value);
            }
            Instruction::Move { bot, .. } => {
                moves.insert(bot, instruction);
            }
        }
    }

    let mut output = HashMap::new();
    let mut part1 = 0;
    while !values.is_empty() {
        // Find bot with two values
        let bot_id = *values.iter().find(|(_, v)| v.len() == 2).unwrap().0;
        let bot_values = values.remove(&bot_id).unwrap();

        if (bot_values[0] == 17 && bot_values[1] == 61)
            || (bot_values[0] == 61 && bot_values[1] == 17)
        {
            part1 = bot_id;
        }

        let bot_move = &moves[&bot_id];
        match bot_move {
            Instruction::Move { low, high, .. } => {
                let (low_value, high_value) = if bot_values[0] < bot_values[1] {
                    (bot_values[0], bot_values[1])
                } else {
                    (bot_values[1], bot_values[0])
                };
                match low {
                    Destination::Bot(id) => values.entry(*id).or_default().push(low_value),
                    Destination::Output(id) => {
                        output.insert(*id, low_value);
                    }
                }
                match high {
                    Destination::Bot(id) => values.entry(*id).or_default().push(high_value),
                    Destination::Output(id) => {
                        output.insert(*id, high_value);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    let part2 = output[&0] * output[&1] * output[&2];

    (part1, part2)
}

#[derive(Debug)]
enum Instruction {
    Give {
        bot: usize,
        value: u64,
    },
    Move {
        bot: usize,
        high: Destination,
        low: Destination,
    },
}

#[derive(Debug)]
enum Destination {
    Bot(usize),
    Output(usize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        if s.starts_with("value") {
            let value = parts[1].parse()?;
            let bot = parts[5].parse()?;
            Ok(Instruction::Give { bot, value })
        } else {
            let bot = parts[1].parse()?;
            let low_value = parts[6].parse()?;
            let low = match parts[5] {
                "bot" => Destination::Bot(low_value),
                "output" => Destination::Output(low_value),
                _ => unreachable!(),
            };
            let high_value = parts[11].parse()?;
            let high = match parts[10] {
                "bot" => Destination::Bot(high_value),
                "output" => Destination::Output(high_value),
                _ => unreachable!(),
            };
            Ok(Instruction::Move { bot, low, high })
        }
    }
}
