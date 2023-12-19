use std::{
    collections::HashMap,
    fmt::{Display, Write},
    str::FromStr,
};

use crate::common::Direction;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut map = input.parse::<Map>().unwrap();

    let mut part1_map = map.clone();
    part1_map.tilt(Direction::North);
    let part1 = part1_map.load();

    const CYCLE_DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut seen = HashMap::new();
    for i in 0.. {
        if let Some(prev_i) = seen.insert(map.tiles.clone(), i) {
            let remaining = (1000000000 - i) % (i - prev_i);
            for _ in 0..remaining {
                for direction in CYCLE_DIRECTIONS.iter() {
                    map.tilt(*direction);
                }
            }
            break;
        }

        for direction in CYCLE_DIRECTIONS.iter() {
            map.tilt(*direction);
        }
    }
    let part2 = map.load();

    (part1, part2)
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Rock,
    Boulder,
}

impl Map {
    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.tilt_north(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
            Direction::West => self.tilt_west(),
        }
    }

    fn tilt_north(&mut self) {
        for column_x in 0..self.width() {
            let mut last_free_y = None;
            for row_y in 0..self.height() {
                let tile = self.tiles[row_y][column_x];
                match tile {
                    Tile::Empty => {
                        if last_free_y.is_none() {
                            last_free_y = Some(row_y);
                        }
                    }
                    Tile::Rock => {
                        last_free_y = None;
                    }
                    Tile::Boulder => {
                        if let Some(free_y) = last_free_y.take() {
                            self.tiles[free_y][column_x] = Tile::Boulder;
                            self.tiles[row_y][column_x] = Tile::Empty;
                            last_free_y = Some(free_y + 1);
                        }
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for column_x in 0..self.width() {
            let mut last_free_y = None;
            for row_y in (0..self.height()).rev() {
                let tile = self.tiles[row_y][column_x];
                match tile {
                    Tile::Empty => {
                        if last_free_y.is_none() {
                            last_free_y = Some(row_y);
                        }
                    }
                    Tile::Rock => {
                        last_free_y = None;
                    }
                    Tile::Boulder => {
                        if let Some(free_y) = last_free_y.take() {
                            self.tiles[free_y][column_x] = Tile::Boulder;
                            self.tiles[row_y][column_x] = Tile::Empty;
                            last_free_y = Some(free_y - 1);
                        }
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row_y in 0..self.height() {
            let mut last_free_x = None;
            for column_x in 0..self.width() {
                let tile = self.tiles[row_y][column_x];
                match tile {
                    Tile::Empty => {
                        if last_free_x.is_none() {
                            last_free_x = Some(column_x);
                        }
                    }
                    Tile::Rock => {
                        last_free_x = None;
                    }
                    Tile::Boulder => {
                        if let Some(free_x) = last_free_x.take() {
                            self.tiles[row_y][free_x] = Tile::Boulder;
                            self.tiles[row_y][column_x] = Tile::Empty;
                            last_free_x = Some(free_x + 1);
                        }
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row_y in 0..self.height() {
            let mut last_free_x = None;
            for column_x in (0..self.width()).rev() {
                let tile = self.tiles[row_y][column_x];
                match tile {
                    Tile::Empty => {
                        if last_free_x.is_none() {
                            last_free_x = Some(column_x);
                        }
                    }
                    Tile::Rock => {
                        last_free_x = None;
                    }
                    Tile::Boulder => {
                        if let Some(free_x) = last_free_x.take() {
                            self.tiles[row_y][free_x] = Tile::Boulder;
                            self.tiles[row_y][column_x] = Tile::Empty;
                            last_free_x = Some(free_x - 1);
                        }
                    }
                }
            }
        }
    }

    fn load(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                (self.height() - y) * row.iter().filter(|tile| tile == &&Tile::Boulder).count()
            })
            .sum::<usize>()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            let row_string = row
                .iter()
                .map(|tile| match tile {
                    Tile::Empty => '.',
                    Tile::Rock => '#',
                    Tile::Boulder => 'O',
                })
                .collect::<String>();
            f.write_str(&row_string)?;
            f.write_char('\n')?;
        }

        Ok(())
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
                        '#' => Tile::Rock,
                        'O' => Tile::Boulder,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Map { tiles })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "136");
        assert_eq!(part2.to_string(), "64");
    }
}
