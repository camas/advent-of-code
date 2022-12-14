use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    str::FromStr,
};

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input_data = input.parse::<InputData>().unwrap();

    let part1 = moves_to_goal(
        &input_data.height_map,
        |_, position| position == input_data.start,
        input_data.goal,
    );

    let part2 = moves_to_goal(
        &input_data.height_map,
        |height, _| height == 0,
        input_data.goal,
    );

    (part1, part2)
}

/// Finds the shortest route from the goal to any position that matches the start condition
fn moves_to_goal<F>(height_map: &HeightMap, start: F, goal: Vector2) -> i64
where
    F: Fn(u8, Vector2) -> bool,
{
    let initial_state = State::new(goal, 0, height_map.height_at(goal));

    let mut best_at_position = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(initial_state);
    while let Some(state) = queue.pop() {
        if !height_map.in_bounds(state.position) {
            continue;
        }

        let curr_height = height_map.height_at(state.position);
        if state.last_height > curr_height + 1 {
            continue;
        }

        match best_at_position.entry(state.position) {
            Entry::Occupied(mut entry) => {
                if *entry.get() <= state.score() {
                    continue;
                }
                entry.insert(state.score());
            }
            Entry::Vacant(entry) => {
                entry.insert(state.score());
            }
        }

        if start(curr_height, state.position) {
            return state.score();
        }

        for direction in [
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
            Vector2::new(0, -1),
        ] {
            let new_state = State::new(state.position + direction, state.moves + 1, curr_height);
            queue.push(new_state);
        }
    }

    unreachable!();
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    position: Vector2,
    moves: i64,
    last_height: u8,
}

impl State {
    fn new(position: Vector2, moves: i64, last_height: u8) -> State {
        State {
            position,
            moves,
            last_height,
        }
    }

    fn score(&self) -> i64 {
        self.moves
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score()).reverse()
    }
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn width(&self) -> i64 {
        self.heights[0].len() as i64
    }

    fn height(&self) -> i64 {
        self.heights.len() as i64
    }

    fn in_bounds(&self, position: Vector2) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.width()
            && position.y < self.height()
    }

    fn height_at(&self, position: Vector2) -> u8 {
        self.heights[position.y as usize][position.x as usize]
    }
}

#[derive(Debug)]
struct InputData {
    height_map: HeightMap,
    start: Vector2,
    goal: Vector2,
}

impl FromStr for InputData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Vector2::default();
        let mut goal = Vector2::default();

        let heights = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Vector2::new(x as i64, y as i64);
                            0
                        }
                        'E' => {
                            goal = Vector2::new(x as i64, y as i64);
                            25
                        }
                        'a'..='z' => c as u8 - b'a',
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(InputData {
            height_map: HeightMap { heights },
            start,
            goal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 31.to_string());
        assert_eq!(result.1.to_string(), 29.to_string())
    }
}
