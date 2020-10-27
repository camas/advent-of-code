use std::str::FromStr;

use crate::Exercise;

pub struct Day1 {}

impl Exercise for Day1 {
    fn part1(&self, input: &str) -> String {
        let directions: Vec<Direction> = input
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let floor = directions.iter().fold(0, |acc, next| match next {
            Direction::Up => acc + 1,
            Direction::Down => acc - 1,
        });
        floor.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let directions: Vec<Direction> = input
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let mut floor = 0;
        for (i, dir) in directions.iter().enumerate() {
            floor += match dir {
                Direction::Up => 1,
                Direction::Down => -1,
            };
            if floor == -1 {
                return (i + 1).to_string();
            }
        }
        unreachable!()
    }
}

enum Direction {
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Direction::Up),
            ")" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}
