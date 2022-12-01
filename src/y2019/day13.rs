use std::{cmp::Ordering, collections::HashMap};

use super::intcode::{Handler, Machine};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut machine = Machine::from_str(input);

    let mut handler = Part1Handler { output: Vec::new() };
    machine.clone().run(Some(&mut handler));
    let output = handler.output;

    let mut screen = HashMap::new();
    for chunk in output.chunks(3) {
        let x = chunk[0];
        let y = chunk[1];
        let v = chunk[2];
        screen.insert((x, y), Tile::from_i64(v));
    }
    let part1 = screen.values().filter(|t| **t == Tile::Block).count();

    machine.memory[0] = 2;
    let mut handler = Part2Handler::default();
    machine.run(Some(&mut handler));
    let part2 = handler.score;

    (part1, part2)
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_i64(v: i64) -> Tile {
        match v {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unreachable!(),
        }
    }
}

struct Part1Handler {
    output: Vec<i64>,
}

impl Handler for Part1Handler {
    fn input(&mut self, _: &Machine) -> i64 {
        unreachable!()
    }

    fn output(&mut self, _: &Machine, value: i64) {
        self.output.push(value);
    }
}

#[derive(Default)]
struct Part2Handler {
    output: Vec<i64>,
    screen: HashMap<(i64, i64), Tile>,
    score: i64,
}

impl Part2Handler {
    fn find_tile(&self, tile: Tile) -> (i64, i64) {
        self.screen
            .iter()
            .filter(|(_, t)| **t == tile)
            .map(|(&(x, y), _)| (x, y))
            .next()
            .unwrap()
    }
}

impl Handler for Part2Handler {
    fn input(&mut self, _: &Machine) -> i64 {
        let (ball_x, _) = self.find_tile(Tile::Ball);
        let (paddle_x, _) = self.find_tile(Tile::Paddle);
        match paddle_x.cmp(&ball_x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    fn output(&mut self, _: &Machine, value: i64) {
        self.output.push(value);
        if self.output.len() != 3 {
            return;
        }

        let x = self.output[0];
        let y = self.output[1];
        let v = self.output[2];
        self.output.clear();

        if x == -1 && y == 0 {
            self.score = v;
            return;
        }

        self.screen.insert((x, y), Tile::from_i64(v));
    }
}
