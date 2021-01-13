use std::{collections::HashSet, num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let moves = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.parse::<Move>().unwrap())
        .collect::<Vec<_>>();

    let mut pos = (0_i64, 0_i64);
    let mut dir = Direction::North;
    let mut visited = HashSet::new();
    let mut part2 = None;
    visited.insert(pos);
    for m in moves.iter() {
        let value = match m {
            Move::Left(v) => {
                dir = dir.left();
                v
            }
            Move::Right(v) => {
                dir = dir.right();
                v
            }
        };
        for _ in 0..*value {
            match dir {
                Direction::North => pos.0 -= 1,
                Direction::East => pos.1 += 1,
                Direction::South => pos.0 += 1,
                Direction::West => pos.1 -= 1,
            }
            if part2.is_none() && !visited.insert(pos) {
                part2 = Some(pos.0.abs() + pos.1.abs());
            }
        }
    }
    let part1 = pos.0.abs() + pos.1.abs();

    (part1, part2.unwrap())
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

enum Move {
    Left(i64),
    Right(i64),
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse()?;
        Ok(match s.chars().next().unwrap() {
            'L' => Move::Left(value),
            'R' => Move::Right(value),
            _ => unreachable!(),
        })
    }
}
