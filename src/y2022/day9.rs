use std::{collections::HashSet, str::FromStr};

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let motions = input
        .trim()
        .lines()
        .map(|l| l.parse::<Motion>().unwrap())
        .collect::<Vec<_>>();

    let mut map = Map::new(2);
    for motion in motions.iter() {
        map.do_motion(motion);
    }
    let part1 = map.seen.len();

    let mut map = Map::new(10);
    for motion in motions.iter() {
        map.do_motion(motion);
    }
    let part2 = map.seen.len();

    (part1, part2)
}

#[derive(Debug)]
struct Map {
    rope: Vec<Vector2>,
    seen: HashSet<Vector2>,
}

impl Map {
    fn new(rope_length: usize) -> Self {
        let rope = (0..rope_length).map(|_| Vector2::default()).collect();
        let mut seen = HashSet::new();
        seen.insert(Vector2::default());
        Self { rope, seen }
    }

    fn do_motion(&mut self, motion: &Motion) {
        for _ in 0..motion.steps {
            self.do_step(motion.direction);
        }
    }

    fn do_step(&mut self, direction: Direction) {
        self.rope[0] += direction.as_vec();

        for i in 1..self.rope.len() {
            let parent = *self.rope.get(i - 1).unwrap();
            let child = *self.rope.get(i).unwrap();

            let diff = parent - child;
            let child_move = match (diff.x, diff.y) {
                (2, 0) => Vector2::new(1, 0),
                (-2, 0) => Vector2::new(-1, 0),
                (0, 2) => Vector2::new(0, 1),
                (0, -2) => Vector2::new(0, -1),
                (1, 2) | (2, 2) | (2, 1) => Vector2::new(1, 1),
                (-1, 2) | (-2, 2) | (-2, 1) => Vector2::new(-1, 1),
                (1, -2) | (2, -2) | (2, -1) => Vector2::new(1, -1),
                (-1, -2) | (-2, -2) | (-2, -1) => Vector2::new(-1, -1),
                (-1..=1, -1..=1) => continue,
                _ => unreachable!("Invalid diff {:?}", diff),
            };
            *self.rope.get_mut(i).unwrap() += child_move;
        }

        self.seen.insert(*self.rope.last().unwrap());
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_vec(&self) -> Vector2 {
        match self {
            Direction::Up => Vector2::new(0, 1),
            Direction::Down => Vector2::new(0, -1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(' ').unwrap();

        let direction = match parts.0 {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let steps = parts.1.parse::<usize>().unwrap();

        Ok(Motion { direction, steps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 13.to_string());
        assert_eq!(result.1.to_string(), 1.to_string());
    }
}
