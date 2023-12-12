use std::collections::HashSet;

use crate::common::{Direction, Vector2};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let map = Map::new(parse_input(input));

    // Follow the pipe
    let mut position = map.start;
    let mut direction = match map.tile(map.start) {
        Tile::NorthEast | Tile::NorthWest | Tile::Vertical => Direction::North,
        Tile::SouthEast | Tile::SouthWest => Direction::South,
        Tile::Horizontal => Direction::East,
        _ => unreachable!(),
    };
    let mut count = 0;
    let mut pipe_tiles = HashSet::new();
    loop {
        position += direction.as_vector2();
        pipe_tiles.insert(position);
        count += 1;
        if position == map.start {
            break;
        }

        direction = map.tile(position).next_direction(direction.invert());
    }
    let part1 = count / 2;

    // Find an outside pipe tile by searching from the top
    let start_position = (0..=map.start.y)
        .map(|y| Vector2::new(map.start.x, y))
        .find(|test_position| pipe_tiles.contains(test_position))
        .unwrap();

    // Go around the pipe again, if on a pipe and heading east any empty tiles below
    // must be inside the loop
    let mut position = start_position;
    let mut direction = match map.tile(position) {
        Tile::SouthEast | Tile::Horizontal => Direction::East,
        Tile::SouthWest => Direction::South,
        _ => unreachable!(),
    };
    let mut inner_count = 0;
    loop {
        if matches!(
            (direction, map.tile(position)),
            (Direction::East, Tile::Horizontal | Tile::NorthEast)
                | (Direction::North, Tile::NorthWest)
        ) {
            inner_count += (1..)
                .take_while(|y_offset| !pipe_tiles.contains(&position.add_y(*y_offset)))
                .count();
        }

        position += direction.as_vector2();
        if position == start_position {
            break;
        }
        direction = map.tile(position).next_direction(direction.invert());
    }
    let part2 = inner_count;

    (part1, part2)
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Vector2,
}

impl Map {
    fn new(mut tiles: Vec<Vec<Tile>>) -> Map {
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find(|(_, tile)| **tile == Tile::Start)
                    .map(|(x, _)| Vector2::new(x as i64, y as i64))
            })
            .unwrap();

        let start_connections = Direction::iter()
            .map(|direction| {
                let position = start + direction.as_vector2();
                position.y >= 0
                    && position.x >= 0
                    && tiles[position.y as usize][position.x as usize]
                        .joins_from(direction.invert())
            })
            .collect::<Vec<_>>();
        let start_tile = match (
            start_connections[0],
            start_connections[1],
            start_connections[2],
            start_connections[3],
        ) {
            (true, true, false, false) => Tile::NorthEast,
            (true, false, true, false) => Tile::Vertical,
            (true, false, false, true) => Tile::NorthWest,
            (false, true, true, false) => Tile::SouthEast,
            (false, true, false, true) => Tile::Horizontal,
            (false, false, true, true) => Tile::SouthWest,
            _ => unreachable!(),
        };

        tiles[start.y as usize][start.x as usize] = start_tile;

        Map { tiles, start }
    }

    fn tile(&self, position: Vector2) -> Tile {
        self.tiles[position.y as usize][position.x as usize]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Tile {
    fn joins_from(&self, direction: Direction) -> bool {
        matches!(
            (direction, self),
            (
                Direction::North,
                Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest,
            ) | (
                Direction::South,
                Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest,
            ) | (
                Direction::East,
                Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthEast,
            ) | (
                Direction::West,
                Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthWest,
            )
        )
    }

    fn next_direction(&self, from: Direction) -> Direction {
        match (self, from) {
            (Tile::Vertical, Direction::South) => Direction::North,
            (Tile::Vertical, Direction::North) => Direction::South,
            (Tile::Horizontal, Direction::East) => Direction::West,
            (Tile::Horizontal, Direction::West) => Direction::East,
            (Tile::NorthEast, Direction::North) => Direction::East,
            (Tile::NorthEast, Direction::East) => Direction::North,
            (Tile::NorthWest, Direction::North) => Direction::West,
            (Tile::NorthWest, Direction::West) => Direction::North,
            (Tile::SouthEast, Direction::South) => Direction::East,
            (Tile::SouthEast, Direction::East) => Direction::South,
            (Tile::SouthWest, Direction::South) => Direction::West,
            (Tile::SouthWest, Direction::West) => Direction::South,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    'S' => Tile::Start,
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    'F' => Tile::SouthEast,
                    '7' => Tile::SouthWest,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let (part1, _) = solve(input);

        assert_eq!(part1.to_string(), "8");

        let input = "FF7F7F7F7F7F7F7F---7
L|LJS|||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

        let (_, part2) = solve(input);

        assert_eq!(part2.to_string(), "10");
    }
}
