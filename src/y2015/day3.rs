use std::collections::HashMap;

use crate::Exercise;

pub struct Day3 {}

impl Exercise for Day3 {
    fn part1(&self, input: &str) -> String {
        let directions = input.chars().map(Direction::from_char).collect::<Vec<_>>();
        let mut x = 0;
        let mut y = 0;
        let mut visited = HashMap::new();
        visited.insert((x, y), 0);
        for dir in directions {
            match dir {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }
            *visited.entry((x, y)).or_insert(0) += 1;
        }

        visited
            .values()
            .filter(|&&value| value >= 1)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let directions = input.chars().map(Direction::from_char).collect::<Vec<_>>();
        let mut x = 0;
        let mut y = 0;
        let mut robot_x = 0;
        let mut robot_y = 0;
        let mut visited = HashMap::new();
        let mut santa_move = true;
        visited.insert((x, y), 0);
        for dir in directions {
            if santa_move {
                match dir {
                    Direction::Up => y += 1,
                    Direction::Down => y -= 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
                *visited.entry((x, y)).or_insert(0) += 1;
            } else {
                match dir {
                    Direction::Up => robot_y += 1,
                    Direction::Down => robot_y -= 1,
                    Direction::Left => robot_x -= 1,
                    Direction::Right => robot_x += 1,
                }
                *visited.entry((robot_x, robot_y)).or_insert(0) += 1;
            }
            santa_move = !santa_move;
        }

        visited
            .values()
            .filter(|&&value| value >= 1)
            .count()
            .to_string()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!(),
        }
    }
}
