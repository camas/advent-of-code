use std::{cmp::Ordering, fmt::Display};

use num::traits::Euclid;
use winnow::{
    ascii::dec_int,
    combinator::separated,
    error::{ContextError, ParserError},
    token::one_of,
    Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let mut position = 50;
    let mut part1_count = 0;
    let mut part2_count = 0;
    for rotation in input.rotations.iter() {
        let (div, rem) = (position
            + match rotation {
                Rotation::Left(count) => -count,
                Rotation::Right(count) => *count,
            })
        .div_rem_euclid(&100);

        if rem == 0 {
            part1_count += 1;
        }
        part2_count += match div.cmp(&0) {
            Ordering::Equal => {
                if rem == 0 {
                    1
                } else {
                    0
                }
            }
            Ordering::Greater => div,
            Ordering::Less => {
                if position == 0 && rem != 0 {
                    div.abs() - 1
                } else if position != 0 && rem == 0 {
                    div.abs() + 1
                } else {
                    div.abs()
                }
            }
        };

        position = rem;

        // println!("{rotation} {position} {div} {rem} {part2_count}");
    }

    (part1_count.to_string(), part2_count.to_string())
}

struct Input {
    rotations: Vec<Rotation>,
}

impl Input {
    fn parse(input: &str) -> Self {
        separated(1.., Rotation::parse::<ContextError<&str>>, '\n')
            .parse(input.trim_end())
            .map(|rotations| Self { rotations })
            .unwrap()
    }
}

enum Rotation {
    Left(i64),
    Right(i64),
}

impl Rotation {
    fn parse<'a, E: ParserError<&'a str>>(input: &mut &'a str) -> winnow::Result<Self, E> {
        (one_of(['L', 'R']), dec_int)
            .parse_next(input)
            .map(|(direction, count)| match direction {
                'L' => Rotation::Left(count),
                'R' => Rotation::Right(count),
                _ => unreachable!(),
            })
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Left(count) => write!(f, "L{count}"),
            Rotation::Right(count) => write!(f, "R{count}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let (result1, result2) = solve(input);
        assert_eq!(result1.to_string(), "3");
        assert_eq!(result2.to_string(), "6");

        assert_eq!(solve("R1000").1.to_string(), "10");
        assert_eq!(solve("R950").1.to_string(), "10");
        assert_eq!(solve("R50\nL1").1.to_string(), "1");
        assert_eq!(solve("L50\nR1").1.to_string(), "1");
    }
}
