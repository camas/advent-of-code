use std::collections::HashSet;

use crate::common::parse_letters;

use super::intcode::{Handler, Machine};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut machine = Machine::from_str(input);

    let mut handler = DayHandler::new(false);
    machine.clone().run(Some(&mut handler));
    let part1 = handler.painted.len();

    let mut handler = DayHandler::new(true);
    machine.run(Some(&mut handler));
    let part2 = parse_letters(&handler.dots());

    (part1, part2)
}

struct DayHandler {
    state: HandlerState,
    position: (i64, i64),
    direction: Direction,
    white_tiles: HashSet<(i64, i64)>,
    painted: HashSet<(i64, i64)>,
}

impl DayHandler {
    fn new(initial_white: bool) -> Self {
        let mut white_tiles = HashSet::new();
        if initial_white {
            white_tiles.insert((0, 0));
        }
        Self {
            state: HandlerState::SendInput,
            position: (0, 0),
            direction: Direction::Up,
            white_tiles,
            painted: HashSet::new(),
        }
    }

    fn dots(&self) -> Vec<Vec<bool>> {
        let x_min = self.white_tiles.iter().map(|(x, _)| *x).min().unwrap();
        let x_max = self.white_tiles.iter().map(|(x, _)| *x).max().unwrap();
        let y_min = self.white_tiles.iter().map(|(_, y)| *y).min().unwrap();
        let y_max = self.white_tiles.iter().map(|(_, y)| *y).max().unwrap();
        (y_min..=y_max)
            .rev()
            .map(|y| {
                (x_min..=x_max)
                    .map(|x| self.white_tiles.contains(&(x, y)))
                    .collect()
            })
            .collect()
    }
}

impl Handler for DayHandler {
    fn input(&mut self, _: &Machine) -> i64 {
        assert!(matches!(self.state, HandlerState::SendInput));
        self.state = HandlerState::ReceiveColor;
        i64::from(self.white_tiles.contains(&self.position))
    }

    fn output(&mut self, _: &Machine, value: i64) {
        match self.state {
            HandlerState::ReceiveColor => {
                self.state = HandlerState::ReceiveTurn;
                self.painted.insert(self.position);
                match value {
                    0 => {
                        self.white_tiles.remove(&self.position);
                    }
                    1 => {
                        self.white_tiles.insert(self.position);
                    }
                    _ => unreachable!(),
                }
            }
            HandlerState::ReceiveTurn => {
                self.state = HandlerState::SendInput;
                match value {
                    0 => self.direction = self.direction.left(),
                    1 => self.direction = self.direction.right(),
                    _ => unreachable!(),
                }
                match self.direction {
                    Direction::Up => self.position.1 += 1,
                    Direction::Down => self.position.1 -= 1,
                    Direction::Left => self.position.0 -= 1,
                    Direction::Right => self.position.0 += 1,
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum HandlerState {
    SendInput,
    ReceiveColor,
    ReceiveTurn,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
