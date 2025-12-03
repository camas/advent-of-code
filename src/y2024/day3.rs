use winnow::{
    combinator::{alt, repeat},
    token::any,
    Parser,
};

use crate::common::parse_u32;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = parse_instructions(input);
    // println!("{:?}", instructions);

    let part1 = instructions
        .iter()
        .filter_map(|instruction: &Instruction| match instruction {
            Instruction::Mul { left, right } => Some((left, right)),
            _ => None,
        })
        .map(|(left, right)| left * right)
        .sum::<u32>();

    let mut part2 = 0;
    let mut enabled = true;
    for instruction in instructions {
        match instruction {
            Instruction::Mul { left, right } => {
                if enabled {
                    part2 += left * right;
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    (part1, part2)
}

#[derive(Debug)]
enum Instruction {
    Mul { left: u32, right: u32 },
    Do,
    Dont,
}

impl Instruction {
    fn parse_mul(input: &mut &str) -> winnow::Result<Self> {
        ("mul(", parse_u32, ",", parse_u32, ")")
            .map(|(_, left, _, right, _)| Self::Mul { left, right })
            .parse_next(input)
    }

    fn parse_do(input: &mut &str) -> winnow::Result<Self> {
        "do()".map(|_| Instruction::Do).parse_next(input)
    }

    fn parse_dont(input: &mut &str) -> winnow::Result<Self> {
        "don't()".map(|_| Instruction::Dont).parse_next(input)
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    repeat(
        0..,
        alt((
            Instruction::parse_mul.map(Some),
            Instruction::parse_do.map(Some),
            Instruction::parse_dont.map(Some),
            any.map(|_| None),
        )),
    )
    .map(|instructions: Vec<Option<Instruction>>| instructions.into_iter().flatten().collect())
    .parse(input)
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "161");
        assert_eq!(part2.to_string(), "48");
    }
}
