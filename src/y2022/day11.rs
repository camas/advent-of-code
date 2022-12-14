use std::str::FromStr;

use itertools::Itertools;
use num::Integer;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let monkeys = input
        .split("\n\n")
        .map(|m| m.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();

    let mut state = State::new(monkeys.clone());
    for _ in 0..20 {
        state.do_round(true);
    }
    let part1 = state.result();

    let mut state = State::new(monkeys);
    for _ in 0..10_000 {
        state.do_round(false);
    }
    let part2 = state.result();

    (part1, part2)
}

#[derive(Debug)]
struct State {
    monkeys: Vec<Monkey>,
    inspection_counts: Vec<u64>,
    lcm: i64,
}

impl State {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let monkeys_len = monkeys.len();
        let lcm = monkeys
            .iter()
            .map(|m| m.test_value)
            .fold(1, |a, b| a.lcm(&b));
        State {
            monkeys,
            inspection_counts: vec![0; monkeys_len],
            lcm,
        }
    }

    fn do_round(&mut self, part1: bool) {
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get_mut(i).unwrap();

            self.inspection_counts[i] += monkey.items.len() as u64;

            let mut items = Vec::new();
            std::mem::swap(&mut items, &mut monkey.items);

            let to_send = items
                .into_iter()
                .map(|item| {
                    let Item { mut worry_level } = item;

                    match monkey.operation_type {
                        Operation::Add => worry_level += monkey.operation_value.get(worry_level),
                        Operation::Multiply => {
                            worry_level *= monkey.operation_value.get(worry_level)
                        }
                    }

                    if part1 {
                        worry_level /= 3;
                    } else {
                        worry_level %= self.lcm;
                    }

                    let target = if worry_level % monkey.test_value == 0 {
                        monkey.test_true_target
                    } else {
                        monkey.test_false_target
                    };

                    (target, Item { worry_level })
                })
                .collect::<Vec<_>>();

            for (target, item) in to_send {
                self.monkeys[target].items.push(item);
            }
        }
    }

    fn result(&self) -> u64 {
        self.inspection_counts
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product::<u64>()
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation_type: Operation,
    operation_value: OperationValue,
    test_value: i64,
    test_true_target: usize,
    test_false_target: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let items = lines[1]
            .trim_start_matches("  Starting items: ")
            .split(", ")
            .map(|p| Item {
                worry_level: p.parse().unwrap(),
            })
            .collect::<Vec<_>>();

        let operation_line_parts = lines[2]
            .trim_start_matches("  Operation: new = old ")
            .split_once(' ')
            .unwrap();
        let operation_type = operation_line_parts.0.parse().unwrap();
        let operation_value = operation_line_parts.1.parse().unwrap();

        let test_value = lines[3]
            .trim_start_matches("  Test: divisible by ")
            .parse()
            .unwrap();
        let test_true_target = lines[4]
            .trim_start_matches("    If true: throw to monkey ")
            .parse()
            .unwrap();
        let test_false_target = lines[5]
            .trim_start_matches("    If false: throw to monkey ")
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            operation_type,
            operation_value,
            test_value,
            test_true_target,
            test_false_target,
        })
    }
}

#[derive(Debug, Clone)]
struct Item {
    worry_level: i64,
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Add,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
enum OperationValue {
    Integer(i64),
    Old,
}

impl OperationValue {
    fn get(&self, old: i64) -> i64 {
        match self {
            OperationValue::Integer(v) => *v,
            OperationValue::Old => old,
        }
    }
}

impl FromStr for OperationValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => OperationValue::Old,
            v => OperationValue::Integer(v.parse().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 10605.to_string());
        assert_eq!(result.1.to_string(), 2713310158_u64.to_string());
    }
}
