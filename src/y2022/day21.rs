use std::{collections::HashMap, str::FromStr};

use num::{One, Rational64, Zero};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut monkeys = Monkeys::new(
        input
            .trim()
            .lines()
            .map(|l| l.parse::<Monkey>().unwrap())
            .collect::<Vec<_>>(),
    );

    let part1 = monkeys.calculate_root();
    let part2 = monkeys.calculate_needed_input();

    (part1, part2)
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    monkey_indexes: HashMap<String, usize>,
}

impl Monkeys {
    fn new(monkeys: Vec<Monkey>) -> Monkeys {
        let monkey_indexes = monkeys
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name.clone(), i))
            .collect();
        Self {
            monkeys,
            monkey_indexes,
        }
    }

    fn calculate_root(&self) -> i64 {
        let root_index = self.monkey_indexes["root"];
        let mut values = vec![None; self.monkeys.len()];
        while values[root_index].is_none() {
            let mut updates = Vec::new();
            for (i, value) in values.iter().enumerate() {
                if value.is_some() {
                    continue;
                }
                let monkey = &self.monkeys[i];
                match &monkey.job {
                    Job::Number(v) => {
                        updates.push((i, *v));
                    }
                    Job::Expression(lhs, operation, rhs) => {
                        let lhs = values[self.monkey_indexes[lhs]];
                        let rhs = values[self.monkey_indexes[rhs]];
                        if lhs.is_none() || rhs.is_none() {
                            continue;
                        }
                        let lhs = lhs.unwrap();
                        let rhs = rhs.unwrap();
                        let v = match operation {
                            Operation::Multiply => lhs * rhs,
                            Operation::Divide => lhs / rhs,
                            Operation::Add => lhs + rhs,
                            Operation::Subtract => lhs - rhs,
                        };
                        updates.push((i, v));
                    }
                }
            }
            for (i, v) in updates {
                values[i] = Some(v);
            }
        }
        values[root_index].unwrap()
    }

    fn calculate_needed_input(&mut self) -> i64 {
        let (lhs, rhs) = match &self.monkeys[self.monkey_indexes["root"]].job {
            Job::Expression(lhs, _, rhs) => (lhs.to_string(), rhs.to_string()),
            Job::Number(_) => unreachable!(),
        };

        let lhs_expression = self.get_expression(&lhs);
        let rhs_expression = self.get_expression(&rhs);

        let lhs_equation = lhs_expression.to_equation();
        let rhs_equation = rhs_expression.to_equation();

        let result = if rhs_equation.a == Rational64::zero() {
            (rhs_equation.b - lhs_equation.b) / lhs_equation.a
        } else {
            (lhs_equation.b - rhs_equation.b) / rhs_equation.a
        };

        *result.round().numer()
    }

    fn get_expression(&self, name: &str) -> Expression {
        if name == "humn" {
            return Expression::Variable;
        }

        let monkey = &self.monkeys[self.monkey_indexes[name]];
        match &monkey.job {
            Job::Number(v) => Expression::Number(*v),
            Job::Expression(lhs, op, rhs) => Expression::Operation {
                lhs: Box::new(self.get_expression(lhs)),
                operation: *op,
                rhs: Box::new(self.get_expression(rhs)),
            },
        }
    }
}

/// Of the form ax + b
#[derive(Debug)]
struct Equation {
    a: Rational64,
    b: Rational64,
}

impl Equation {
    fn multiply(&self, other: &Equation) -> Equation {
        assert_eq!(self.a * other.a, Rational64::zero());
        Equation {
            a: (self.a * other.b) + (other.a * self.b),
            b: self.b * other.b,
        }
    }

    fn divide(&self, other: &Equation) -> Equation {
        assert_eq!(other.a, Rational64::zero());
        Equation {
            a: self.a / other.b,
            b: self.b / other.b,
        }
    }

    fn add(&self, other: &Equation) -> Equation {
        Equation {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }

    fn subtract(&self, other: &Equation) -> Equation {
        Equation {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

#[derive(Debug)]
enum Expression {
    Number(i64),
    Operation {
        lhs: Box<Expression>,
        operation: Operation,
        rhs: Box<Expression>,
    },
    Variable,
}

impl Expression {
    fn to_equation(&self) -> Equation {
        match self {
            Expression::Number(v) => Equation {
                a: Rational64::zero(),
                b: Rational64::new(*v, 1),
            },
            Expression::Operation {
                lhs,
                operation,
                rhs,
            } => {
                let lhs = lhs.to_equation();
                let rhs = rhs.to_equation();

                match operation {
                    Operation::Multiply => lhs.multiply(&rhs),
                    Operation::Divide => lhs.divide(&rhs),
                    Operation::Add => lhs.add(&rhs),
                    Operation::Subtract => lhs.subtract(&rhs),
                }
            }
            Expression::Variable => Equation {
                a: Rational64::one(),
                b: Rational64::zero(),
            },
        }
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

#[derive(Debug)]
enum Job {
    Number(i64),
    Expression(String, Operation, String),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply,
    Divide,
    Add,
    Subtract,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, s) = s.split_once(": ").unwrap();
        let job_parts = s.split(' ').collect::<Vec<_>>();
        let job = if job_parts.len() == 1 {
            Job::Number(job_parts[0].parse().unwrap())
        } else {
            let operation = match job_parts[1] {
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                _ => unreachable!(),
            };
            Job::Expression(
                job_parts[0].to_string(),
                operation,
                job_parts[2].to_string(),
            )
        };

        Ok(Monkey {
            name: name.to_string(),
            job,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 152.to_string());
        assert_eq!(result.1.to_string(), 301.to_string());
    }
}
