use std::collections::{HashMap, HashSet};

use crate::Exercise;

pub struct Day25;

impl Exercise for Day25 {
    fn part1(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();
        let starting_state = lines[0].chars().nth(15).unwrap();
        let target_steps = lines[1]
            .split(' ')
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let instructions = lines[2..]
            .chunks(10)
            .map(|sub_lines| {
                let state_name = sub_lines[1].chars().nth(9).unwrap();
                let false_write = match sub_lines[3].chars().nth(22).unwrap() {
                    '0' => false,
                    '1' => true,
                    _ => panic!(),
                };
                let false_dir = match sub_lines[4]
                    .split(' ')
                    .nth(10)
                    .unwrap()
                    .trim_end_matches('.')
                {
                    "left" => Direction::Left,
                    "right" => Direction::Right,
                    _ => panic!(),
                };
                let false_next_state = sub_lines[5].chars().nth(26).unwrap();
                let true_write = match sub_lines[7].chars().nth(22).unwrap() {
                    '0' => false,
                    '1' => true,
                    _ => panic!(),
                };
                let true_dir = match sub_lines[8]
                    .split(' ')
                    .nth(10)
                    .unwrap()
                    .trim_end_matches('.')
                {
                    "left" => Direction::Left,
                    "right" => Direction::Right,
                    _ => panic!(),
                };
                let true_next_state = sub_lines[9].chars().nth(26).unwrap();
                let instr = Instruction {
                    false_write,
                    false_dir,
                    false_next_state,
                    true_write,
                    true_dir,
                    true_next_state,
                };
                (state_name, instr)
            })
            .collect::<HashMap<_, _>>();

        let mut tape = HashSet::new();
        let mut state = &instructions[&starting_state];
        let mut position = 0_i64;
        for _ in 0..target_steps {
            let cur_value = tape.contains(&position);
            if cur_value {
                if state.true_write {
                    tape.insert(position);
                } else {
                    tape.remove(&position);
                }
                position += match state.true_dir {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                state = &instructions[&state.true_next_state];
            } else {
                if state.false_write {
                    tape.insert(position);
                } else {
                    tape.remove(&position);
                }
                position += match state.false_dir {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                state = &instructions[&state.false_next_state];
            }
        }

        tape.len().to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Finished!".to_string()
    }
}

#[derive(Debug)]
struct Instruction {
    false_write: bool,
    false_dir: Direction,
    false_next_state: char,
    true_write: bool,
    true_dir: Direction,
    true_next_state: char,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
