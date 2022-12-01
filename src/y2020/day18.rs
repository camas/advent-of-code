use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let expressions = input
        .lines()
        .map(|l| l.parse::<Expression>().unwrap())
        .collect::<Vec<_>>();

    let part1 = expressions.iter().map(|e| e.evaluate()).sum::<u64>();
    let part2 = expressions.iter().map(|e| e.evaluate2()).sum::<u64>();

    (part1, part2)
}

#[derive(Debug)]
enum Expression {
    Number(u64),
    Complex {
        children: Vec<Expression>,
        operators: Vec<Operator>,
    },
}

impl Expression {
    fn from_tokens<I: Iterator<Item = Token>>(tokens: &mut I) -> Self {
        let next = tokens.next().unwrap();
        let first_child = match next {
            Token::Number(value) => Expression::Number(value),
            Token::LeftBracket => Self::from_tokens(tokens),
            _ => unreachable!(),
        };
        let mut children = vec![first_child];
        let mut operators = Vec::new();
        loop {
            match tokens.next() {
                None => break,
                Some(Token::RightBracket) => break,
                Some(Token::Operator(op)) => operators.push(op),
                _ => unreachable!(),
            }
            let child = match tokens.next().unwrap() {
                Token::Number(value) => Expression::Number(value),
                Token::LeftBracket => Self::from_tokens(tokens),
                _ => unreachable!(),
            };
            children.push(child);
        }
        if children.len() == 1 {
            return children.pop().unwrap();
        }
        Expression::Complex {
            children,
            operators,
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Expression::Number(v) => *v,
            Expression::Complex {
                children,
                operators,
            } => {
                let mut children = children.iter();
                let mut curr = children.next().unwrap().evaluate();
                for (operator, child) in operators.iter().zip(children) {
                    let child_value = child.evaluate();
                    match operator {
                        Operator::Add => curr += child_value,
                        Operator::Multiply => curr *= child_value,
                    }
                }
                curr
            }
        }
    }

    fn evaluate2(&self) -> u64 {
        match self {
            Expression::Number(v) => *v,
            Expression::Complex {
                children,
                operators,
            } => {
                // Two passes. First addition, then multiplication
                let nums = children.iter().map(|c| c.evaluate2()).collect::<Vec<_>>();
                let mut to_mult = Vec::new();
                let mut curr = nums[0];
                for (n, op) in nums[1..].iter().zip(operators.iter()) {
                    match op {
                        Operator::Add => curr += *n,
                        Operator::Multiply => {
                            to_mult.push(curr);
                            curr = *n;
                        }
                    }
                }
                to_mult.push(curr);
                to_mult.iter().product()
            }
        }
    }
}

impl FromStr for Expression {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Tokenize
        let mut tokens = s.chars().filter(|c| *c != ' ').map(|c| match c {
            '(' => Token::LeftBracket,
            ')' => Token::RightBracket,
            '+' => Token::Operator(Operator::Add),
            '*' => Token::Operator(Operator::Multiply),
            n if n.is_ascii_digit() => Token::Number(n.to_digit(10).unwrap() as u64),
            _ => unreachable!(),
        });
        Ok(Self::from_tokens(&mut tokens))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Number(u64),
    Operator(Operator),
    LeftBracket,
    RightBracket,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let expr = "2 * 3 + (4 * 5)".parse::<Expression>().unwrap();
        assert_eq!(expr.evaluate(), 26);
        assert_eq!(expr.evaluate2(), 46);
    }
}
