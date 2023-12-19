use std::{
    collections::{BinaryHeap, HashSet},
    ops::RangeInclusive,
};

use crate::common::{Direction, Vector2};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let heat_losses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let map = Map { heat_losses };

    let part1 = search(&map, 1..=3);
    let part2 = search(&map, 4..=10);

    (part1, part2)
}

fn search(map: &Map, move_range: RangeInclusive<i64>) -> u64 {
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        position: Vector2::new(0, 0),
        direction: Direction::North,
        score: 0,
    });
    queue.push(State {
        position: Vector2::new(0, 0),
        direction: Direction::East,
        score: 0,
    });
    loop {
        let state = queue.pop().unwrap();
        if !seen.insert((state.position, state.direction)) {
            continue;
        }

        if state.position.x == map.width() as i64 - 1 && state.position.y == map.height() as i64 - 1
        {
            return state.score;
        }

        let next_direction = if state.direction == Direction::North {
            Direction::East
        } else {
            Direction::North
        };
        for side_direction in [state.direction.left(), state.direction.right()] {
            let mut score = state.score
                + (1..*move_range.start())
                    .map(|i| {
                        let position = state.position + i * side_direction.as_vector2();
                        if !map.in_bounds(position) {
                            0
                        } else {
                            map.heat_loss(position)
                        }
                    })
                    .sum::<u64>();
            for i in move_range.clone() {
                let new_position = state.position + i * side_direction.as_vector2();
                if !map.in_bounds(new_position) {
                    break;
                }
                score += map.heat_loss(new_position);
                queue.push(State {
                    position: new_position,
                    direction: next_direction,
                    score,
                });
            }
        }
    }
}

struct Map {
    heat_losses: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    position: Vector2,
    direction: Direction,
    score: u64,
}

impl Map {
    fn width(&self) -> usize {
        self.heat_losses[0].len()
    }

    fn height(&self) -> usize {
        self.heat_losses.len()
    }

    fn in_bounds(&self, position: Vector2) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.width() as i64
            && position.y < self.height() as i64
    }

    fn heat_loss(&self, position: Vector2) -> u64 {
        self.heat_losses[position.y as usize][position.x as usize] as u64
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "102");
        assert_eq!(part2.to_string(), "94");
    }
}
