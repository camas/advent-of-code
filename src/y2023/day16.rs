use std::{collections::HashSet, str::FromStr};

use crate::common::{Direction, Vector2};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let map = input.parse::<Map>().unwrap();

    let part1 = track_beam(&map, Vector2::new(0, 0), Direction::East);

    let part2 = (0..map.tiles[0].len())
        .map(|x| track_beam(&map, Vector2::new(x as i64, 0), Direction::South))
        .chain((0..map.tiles[0].len()).map(|x| {
            track_beam(
                &map,
                Vector2::new(x as i64, map.tiles.len() as i64 - 1),
                Direction::North,
            )
        }))
        .chain(
            (0..map.tiles.len())
                .map(|y| track_beam(&map, Vector2::new(0, y as i64), Direction::East)),
        )
        .chain((0..map.tiles.len()).map(|y| {
            track_beam(
                &map,
                Vector2::new(y as i64, map.tiles[0].len() as i64 - 1),
                Direction::West,
            )
        }))
        .max()
        .unwrap();

    (part1, part2)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    position: Vector2,
    direction: Direction,
}

fn track_beam(map: &Map, starting_position: Vector2, starting_direction: Direction) -> usize {
    let mut seen = HashSet::new();
    let mut queue = Vec::new();
    queue.push(State {
        position: starting_position,
        direction: starting_direction,
    });
    while let Some(state) = queue.pop() {
        if !map.in_bounds(state.position) {
            continue;
        }
        if !seen.insert(state.clone()) {
            continue;
        }

        let tile = map.tiles[state.position.y as usize][state.position.x as usize];
        match (tile, state.direction) {
            (Tile::Empty, _)
            | (Tile::HorizontalSplitter, Direction::East | Direction::West)
            | (Tile::VerticalSplitter, Direction::North | Direction::South) => queue.push(State {
                position: state.position + state.direction.as_vector2(),
                direction: state.direction,
            }),
            (Tile::HorizontalSplitter, Direction::North | Direction::South) => {
                queue.push(State {
                    position: state.position + Direction::East.as_vector2(),
                    direction: Direction::East,
                });
                queue.push(State {
                    position: state.position + Direction::West.as_vector2(),
                    direction: Direction::West,
                });
            }
            (Tile::VerticalSplitter, Direction::East | Direction::West) => {
                queue.push(State {
                    position: state.position + Direction::North.as_vector2(),
                    direction: Direction::North,
                });
                queue.push(State {
                    position: state.position + Direction::South.as_vector2(),
                    direction: Direction::South,
                });
            }
            (Tile::NorthEastMirror, _) => {
                let new_direction = match state.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                queue.push(State {
                    position: state.position + new_direction.as_vector2(),
                    direction: new_direction,
                });
            }
            (Tile::NorthWestMirror, _) => {
                let new_direction = match state.direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::North,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                };
                queue.push(State {
                    position: state.position + new_direction.as_vector2(),
                    direction: new_direction,
                });
            }
        }
    }

    seen.into_iter()
        .map(|state| state.position)
        .collect::<HashSet<_>>()
        .len()
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    NorthEastMirror,
    NorthWestMirror,
}

impl Map {
    fn in_bounds(&self, position: Vector2) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.tiles[0].len() as i64
            && position.y < self.tiles.len() as i64
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => Tile::Empty,
                        '-' => Tile::HorizontalSplitter,
                        '|' => Tile::VerticalSplitter,
                        '/' => Tile::NorthEastMirror,
                        '\\' => Tile::NorthWestMirror,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Map { tiles })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "46");
        assert_eq!(part2.to_string(), "51");
    }
}
