use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

use num::integer::lcm;

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = input.trim().parse::<Input>().unwrap();

    let part1 = pathfind(&input, false);
    let part2 = pathfind(&input, true);

    (part1, part2)
}

fn pathfind(input: &Input, part2: bool) -> i64 {
    let height = input.tiles.len() as i64;
    let width = input.tiles[0].len() as i64;

    let cycle_length = lcm(width - 2, height - 2);
    let mut blizzards_at_time = Vec::with_capacity(cycle_length as usize);
    let mut blizzards = input.blizzards.clone();
    for _ in 0..cycle_length {
        blizzards_at_time.push(blizzards.iter().map(|b| b.position).collect::<HashSet<_>>());
        for blizzard in blizzards.iter_mut() {
            let mut new_pos = blizzard.position + blizzard.direction.as_vector();
            if new_pos.x <= 0 {
                new_pos.x = width - 2;
            } else if new_pos.x >= width - 1 {
                new_pos.x = 1;
            }
            if new_pos.y <= 0 {
                new_pos.y = height - 2;
            } else if new_pos.y >= height - 1 {
                new_pos.y = 1;
            }
            blizzard.position = new_pos
        }
    }

    let initial_x = input.tiles[0]
        .iter()
        .position(|t| *t == Tile::None)
        .unwrap() as i64;
    let start_pos = Vector2::new(initial_x, 0);
    let end_x = input
        .tiles
        .last()
        .unwrap()
        .iter()
        .position(|t| *t == Tile::None)
        .unwrap() as i64;
    let end_pos = Vector2::new(end_x, height - 1);

    let initial_state = State::new(start_pos, 0, Progress::ToExit, start_pos, end_pos);
    let mut queue = BinaryHeap::new();
    queue.push(initial_state);
    let mut seen = HashSet::new();

    while let Some(mut state) = queue.pop() {
        if state.player.x < 0
            || state.player.y < 0
            || state.player.x >= width
            || state.player.y >= height
        {
            continue;
        }
        if input.tiles[state.player.y as usize][state.player.x as usize] == Tile::Wall {
            continue;
        }
        if !seen.insert((state.time, state.player, state.progress)) {
            continue;
        }
        if blizzards_at_time[(state.time % cycle_length) as usize].contains(&state.player) {
            continue;
        }

        if state.player == end_pos {
            if !part2 {
                return state.time;
            }
            if state.progress == Progress::ToExitAgain {
                return state.time;
            }
            if state.progress == Progress::ToExit {
                state.progress = Progress::BackForFood;
            }
        } else if state.player == start_pos && state.progress == Progress::BackForFood {
            state.progress = Progress::ToExitAgain;
        }

        queue.extend(state.moves(start_pos, end_pos));
    }

    unreachable!();
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    player: Vector2,
    time: i64,
    progress: Progress,
    h: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Progress {
    ToExit,
    BackForFood,
    ToExitAgain,
}

impl State {
    fn new(
        player: Vector2,
        time: i64,
        progress: Progress,
        start_position: Vector2,
        end_position: Vector2,
    ) -> State {
        let h = time
            + match progress {
                Progress::ToExit => {
                    player.manhattan_distance(end_position) * 2
                        + player.manhattan_distance(start_position)
                }
                Progress::BackForFood => {
                    player.manhattan_distance(end_position)
                        + player.manhattan_distance(start_position)
                }
                Progress::ToExitAgain => player.manhattan_distance(end_position),
            };
        State {
            player,
            time,
            progress,
            h,
        }
    }

    fn moves(&self, start_position: Vector2, end_position: Vector2) -> Vec<State> {
        vec![
            State::new(
                self.player,
                self.time + 1,
                self.progress,
                start_position,
                end_position,
            ),
            State::new(
                self.player + Direction::Up.as_vector(),
                self.time + 1,
                self.progress,
                start_position,
                end_position,
            ),
            State::new(
                self.player + Direction::Down.as_vector(),
                self.time + 1,
                self.progress,
                start_position,
                end_position,
            ),
            State::new(
                self.player + Direction::Left.as_vector(),
                self.time + 1,
                self.progress,
                start_position,
                end_position,
            ),
            State::new(
                self.player + Direction::Right.as_vector(),
                self.time + 1,
                self.progress,
                start_position,
                end_position,
            ),
        ]
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.h.partial_cmp(&other.h).map(Ordering::reverse)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Input {
    tiles: Vec<Vec<Tile>>,
    blizzards: Vec<Blizzard>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blizzards = Vec::new();

        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::None,
                        '#' => Tile::Wall,
                        '^' | '>' | 'v' | '<' => {
                            let direction = match c {
                                '^' => Direction::Up,
                                '>' => Direction::Right,
                                'v' => Direction::Down,
                                '<' => Direction::Left,
                                _ => unreachable!(),
                            };
                            blizzards.push(Blizzard {
                                position: Vector2::new(x as i64, y as i64),
                                direction,
                            });
                            Tile::None
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Input { tiles, blizzards })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Blizzard {
    position: Vector2,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_vector(&self) -> Vector2 {
        match self {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    None,
    Wall,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 18.to_string());
        assert_eq!(result.1.to_string(), 54.to_string());
    }
}
