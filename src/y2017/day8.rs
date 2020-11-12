use std::collections::HashMap;

use crate::Exercise;

pub struct Day8;

impl Exercise for Day8 {
    fn part1(&self, input: &str) -> String {
        let instructions = input
            .lines()
            .map(|line| Instruction::from_str(line))
            .collect::<Vec<_>>();
        let mut registers = HashMap::new();
        for instruction in instructions {
            let r = *registers.entry(instruction.condition_register).or_insert(0);
            let val = instruction.condition_amount;
            let condition_matched = match instruction.condition {
                Condition::Equal => r == val,
                Condition::NotEqual => r != val,
                Condition::LessThan => r < val,
                Condition::LessOrEqual => r <= val,
                Condition::GreaterThan => r > val,
                Condition::GreaterOrEqual => r >= val,
            };
            if condition_matched {
                let amount = match instruction.action {
                    Action::Inc => instruction.action_amount,
                    Action::Dec => -instruction.action_amount,
                };
                *registers.entry(instruction.action_register).or_insert(0) += amount;
            }
        }
        registers.values().max().unwrap().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let instructions = input
            .lines()
            .map(|line| Instruction::from_str(line))
            .collect::<Vec<_>>();
        let mut registers = HashMap::new();
        let mut highest = i64::MIN;
        for instruction in instructions {
            let r = *registers.entry(instruction.condition_register).or_insert(0);
            let val = instruction.condition_amount;
            let condition_matched = match instruction.condition {
                Condition::Equal => r == val,
                Condition::NotEqual => r != val,
                Condition::LessThan => r < val,
                Condition::LessOrEqual => r <= val,
                Condition::GreaterThan => r > val,
                Condition::GreaterOrEqual => r >= val,
            };
            if condition_matched {
                let amount = match instruction.action {
                    Action::Inc => instruction.action_amount,
                    Action::Dec => -instruction.action_amount,
                };
                *registers.entry(instruction.action_register).or_insert(0) += amount;
                let new_value = *registers.get(instruction.action_register).unwrap();
                if new_value > highest {
                    highest = new_value;
                }
            }
        }
        highest.to_string()
    }
}

struct Instruction<'a> {
    action: Action,
    action_register: &'a str,
    action_amount: i64,
    condition: Condition,
    condition_register: &'a str,
    condition_amount: i64,
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        let action_register = parts[0];
        let action = match parts[1] {
            "inc" => Action::Inc,
            "dec" => Action::Dec,
            _ => panic!(),
        };
        let action_amount = parts[2].parse().unwrap();
        let condition_register = parts[4];
        let condition = match parts[5] {
            "<" => Condition::LessThan,
            "<=" => Condition::LessOrEqual,
            ">" => Condition::GreaterThan,
            ">=" => Condition::GreaterOrEqual,
            "==" => Condition::Equal,
            "!=" => Condition::NotEqual,
            _ => panic!(),
        };
        let condition_amount = parts[6].parse().unwrap();
        Self {
            action,
            action_register,
            action_amount,
            condition,
            condition_register,
            condition_amount,
        }
    }
}

enum Action {
    Inc,
    Dec,
}

enum Condition {
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    Equal,
    NotEqual,
}
