use std::str::FromStr;

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::many1,
    IResult, Parser,
};

use crate::common::{Vector2, Vector3};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // glhf

    let input = input.parse::<Input>().unwrap();

    let object = input.map.position_after_moves(&input.moves);
    let part1 = object.score();

    // TODO: finish
    let cube_map = input.map.as_cube_map();
    let object2 = cube_map.position_after_moves(&input.moves);
    let part2 = object2.score();

    (part1, part2)
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn position_after_moves(&self, moves: &[Move]) -> Object {
        let mut object = Object {
            position: self.top_left_tile(),
            direction: Direction::Right,
        };

        for m in moves {
            match m {
                Move::Left => object.direction = object.direction.turn_left(),
                Move::Right => object.direction = object.direction.turn_right(),
                Move::Forward(count) => {
                    for _ in 0..*count {
                        let next_pos = self.get_next_space(object.position, object.direction);
                        if self.tile_at(next_pos) == Tile::Floor {
                            object.position = next_pos;
                        } else {
                            break;
                        }
                    }
                }
            }

            // for (y, row) in self.tiles.iter().enumerate() {
            //     let mut line = String::new();
            //     for (x, tile) in row.iter().enumerate() {
            //         if object.position == Vector2::new(x as i64, y as i64) {
            //             line.push('X');
            //             continue;
            //         }
            //         line.push(match tile {
            //             Tile::Empty => ' ',
            //             Tile::Floor => '.',
            //             Tile::Wall => '#',
            //         });
            //     }
            //     println!("{}", line);
            // }
            // println!();
        }

        object
    }

    fn get_next_space(&self, position: Vector2, direction: Direction) -> Vector2 {
        let mut next = position + direction.as_vector();
        if self.in_bounds(next) && self.tile_at(next) != Tile::Empty {
            return next;
        }

        match direction {
            Direction::Up => next.y = self.height() as i64 - 1,
            Direction::Down => next.y = 0,
            Direction::Left => next.x = self.width() as i64 - 1,
            Direction::Right => next.x = 0,
        }

        while self.tile_at(next) == Tile::Empty {
            next += direction.as_vector();
        }

        next
    }

    fn in_bounds(&self, position: Vector2) -> bool {
        position.y >= 0
            && position.y < self.height() as i64
            && position.x >= 0
            && position.x < self.width() as i64
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn tile_at(&self, position: Vector2) -> Tile {
        self.tile_at_i64(position.x, position.y)
    }

    fn tile_at_i64(&self, x: i64, y: i64) -> Tile {
        self.tile_at_usize(x as usize, y as usize)
    }

    fn tile_at_usize(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn top_left_tile(&self) -> Vector2 {
        let x = self.tiles[0]
            .iter()
            .position(|t| *t == Tile::Floor)
            .unwrap();
        Vector2::new(x as i64, 0)
    }

    fn as_cube_map(&self) -> CubeMap {
        let cube_size = self.cube_face_size();

        let first_chunk_index = (0..)
            .find(|i| self.tile_at_usize(i * cube_size, 0) != Tile::Empty)
            .unwrap();

        #[derive(Debug)]
        struct QueueItem {
            chunk_x: i64,
            chunk_y: i64,
            side: CubeSide,
        }

        let initial_item = QueueItem {
            chunk_x: first_chunk_index as i64,
            chunk_y: 0,
            side: CubeSide::new(Vector3::new(0, 0, 1), Vector3::new(0, -1, 1)),
        };

        type TmpChunkData = (Vec<Vec<Tile>>, CubeSide, i64, i64);
        let mut chunks: Vec<Option<TmpChunkData>> = vec![None; 6];

        let mut queue = vec![initial_item];
        while let Some(item) = queue.pop() {
            let item_position = Vector2::new(
                item.chunk_x * cube_size as i64,
                item.chunk_y * cube_size as i64,
            );
            if !self.in_bounds(item_position) || self.tile_at(item_position) == Tile::Empty {
                continue;
            }

            if let Some(entry) = &chunks[item.side.as_index()] {
                assert_eq!(entry.1, item.side);
                continue;
            }

            let chunk = self.copy_chunk(
                item.chunk_x as usize * cube_size,
                item.chunk_y as usize * cube_size,
                cube_size,
            );
            chunks[item.side.as_index()] = Some((chunk, item.side, item.chunk_x, item.chunk_y));

            let new_items = [
                QueueItem {
                    chunk_x: item.chunk_x,
                    chunk_y: item.chunk_y - 1,
                    side: item.side.rotate_up(),
                },
                QueueItem {
                    chunk_x: item.chunk_x,
                    chunk_y: item.chunk_y + 1,
                    side: item.side.rotate_down(),
                },
                QueueItem {
                    chunk_x: item.chunk_x + 1,
                    chunk_y: item.chunk_y,
                    side: item.side.rotate_right(),
                },
                QueueItem {
                    chunk_x: item.chunk_x - 1,
                    chunk_y: item.chunk_y,
                    side: item.side.rotate_left(),
                },
            ];
            queue.extend(new_items);
        }

        assert!(chunks.iter().all(Option::is_some));

        let chunks = chunks
            .into_iter()
            .map(|chunk| {
                let (tiles, side, chunk_x, chunk_y) = chunk.unwrap();
                CubeMapSide {
                    map: Map { tiles },
                    side,
                    chunk_x,
                    chunk_y,
                }
            })
            .collect::<Vec<_>>();

        CubeMap {
            sides: chunks.try_into().unwrap(),
            size: cube_size,
            _full_tiles: self.tiles.clone(),
        }
    }

    fn copy_chunk(&self, x_offset: usize, y_offset: usize, size: usize) -> Vec<Vec<Tile>> {
        self.tiles
            .iter()
            .skip(y_offset)
            .take(size)
            .map(|row| row.iter().skip(x_offset).take(size).cloned().collect())
            .collect()
    }

    fn cube_face_size(&self) -> usize {
        for size in (1..=100).rev() {
            if !self.width().is_multiple_of(size) || !self.height().is_multiple_of(size) {
                continue;
            }
            let chunk_row_count = self.width() / size;
            let chunk_column_count = self.height() / size;

            let mut found = true;
            'outer: for chunk_start_y in (0..chunk_column_count).map(|i| i * size) {
                for chunk_start_x in (0..chunk_row_count).map(|i| i * size) {
                    let top_left_tile = self.tile_at_usize(chunk_start_x, chunk_start_y);

                    if top_left_tile == Tile::Empty {
                        if !(chunk_start_y..(chunk_start_y + size)).all(|y| {
                            (chunk_start_x..(chunk_start_x + size))
                                .all(|x| self.tile_at_usize(x, y) == Tile::Empty)
                        }) {
                            found = false;
                            break 'outer;
                        }
                    } else if !(chunk_start_y..(chunk_start_y + size)).all(|y| {
                        (chunk_start_x..(chunk_start_x + size))
                            .all(|x| self.tile_at_usize(x, y) != Tile::Empty)
                    }) {
                        found = false;
                        break 'outer;
                    }
                }
            }
            if found {
                return size;
            }
        }
        unreachable!();
    }
}

#[derive(Debug)]
struct CubeMap {
    sides: [CubeMapSide; 6],
    size: usize,
    _full_tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
struct CubeMapSide {
    map: Map,
    side: CubeSide,
    chunk_x: i64,
    chunk_y: i64,
}

impl CubeMap {
    fn position_after_moves(&self, moves: &[Move]) -> Object {
        struct Position {
            side_index: usize,
            position: Vector2,
            direction: Direction,
        }

        let initial_side_index = 4; // TODO: no hardcode
        let mut position = Position {
            side_index: initial_side_index,
            position: self.sides[initial_side_index].map.top_left_tile(),
            direction: Direction::Right,
        };

        for m in moves {
            match m {
                Move::Left => position.direction = position.direction.turn_left(),
                Move::Right => position.direction = position.direction.turn_right(),
                Move::Forward(count) => {
                    for _ in 0..*count {
                        let next_pos = position.position + position.direction.as_vector();
                        let side = &self.sides[position.side_index];
                        if !side.map.in_bounds(next_pos) {
                            let next_side = match position.direction {
                                Direction::Up => side.side.rotate_up(),
                                Direction::Down => {
                                    side.side.rotate_down().rotate_side_90().rotate_side_90()
                                }
                                Direction::Left => side
                                    .side
                                    .rotate_left()
                                    .rotate_side_90()
                                    .rotate_side_90()
                                    .rotate_side_90(),
                                Direction::Right => side.side.rotate_right().rotate_side_90(),
                            };
                            let next_offset = match position.direction {
                                Direction::Up => next_pos.x,
                                Direction::Down => self.size as i64 - next_pos.x - 1,
                                Direction::Left => self.size as i64 - next_pos.y - 1,
                                Direction::Right => next_pos.y,
                            };
                            let expected_next_side = &self.sides[next_side.as_index()];
                            let (next_direction, next_position) = if next_side
                                == expected_next_side.side
                            {
                                (
                                    Direction::Up,
                                    Vector2::new(next_offset, self.size as i64 - 1),
                                )
                            } else if next_side.rotate_side_90() == expected_next_side.side {
                                (
                                    Direction::Left,
                                    Vector2::new(
                                        self.size as i64 - 1,
                                        self.size as i64 - 1 - next_offset,
                                    ),
                                )
                            } else if next_side.rotate_side_90().rotate_side_90()
                                == expected_next_side.side
                            {
                                (
                                    Direction::Down,
                                    Vector2::new(self.size as i64 - 1 - next_offset, 0),
                                )
                            } else if next_side.rotate_side_90().rotate_side_90().rotate_side_90()
                                == expected_next_side.side
                            {
                                (Direction::Right, Vector2::new(0, next_offset))
                            } else {
                                unreachable!();
                            };
                            if expected_next_side.map.tile_at(next_position) != Tile::Floor {
                                break;
                            }
                            position.side_index = expected_next_side.side.as_index();
                            position.position = next_position;
                            position.direction = next_direction;
                        } else if side.map.tile_at(next_pos) == Tile::Floor {
                            position.position = next_pos;
                        } else {
                            break;
                        }
                    }
                }
            }

            // let side = &self.sides[position.side_index];
            // let tile_position = Vector2::new(
            //     side.chunk_x * self.size as i64 + position.position.x,
            //     side.chunk_y * self.size as i64 + position.position.y,
            // );
            // for (y, row) in self.full_tiles.iter().enumerate() {
            //     let mut line = String::new();
            //     for (x, tile) in row.iter().enumerate() {
            //         if tile_position == Vector2::new(x as i64, y as i64) {
            //             line.push('X');
            //             continue;
            //         }
            //         line.push(match tile {
            //             Tile::Empty => ' ',
            //             Tile::Floor => '.',
            //             Tile::Wall => '#',
            //         });
            //     }
            //     println!("{}", line);
            // }
            // println!();
        }

        let side = &self.sides[position.side_index];
        Object {
            position: Vector2::new(
                side.chunk_x * self.size as i64 + position.position.x,
                side.chunk_y * self.size as i64 + position.position.y,
            ),
            direction: position.direction,
        }
    }
}

/// Modelled as two corners of a square of a cube
/// Could be simplified to the 4 corners of the 6 squares
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CubeSide {
    side: Vector3,
    bottom_edge: Vector3,
}

impl CubeSide {
    fn new(side: Vector3, bottom_edge: Vector3) -> CubeSide {
        assert!(matches!(
            (side.x, side.y, side.z),
            (1 | -1, 0, 0) | (0, 1 | -1, 0) | (0, 0, 1 | -1)
        ));
        assert!(matches!(
            (bottom_edge.x, bottom_edge.y, bottom_edge.z),
            (1 | -1, 1 | -1, 0) | (1 | -1, 0, 1 | -1) | (0, 1 | -1, 1 | -1)
        ));

        CubeSide { side, bottom_edge }
    }

    fn as_index(&self) -> usize {
        match (self.side.x, self.side.y, self.side.z) {
            (1, 0, 0) => 0,
            (-1, 0, 0) => 1,
            (0, 1, 0) => 2,
            (0, -1, 0) => 3,
            (0, 0, 1) => 4,
            (0, 0, -1) => 5,
            _ => unreachable!(),
        }
    }

    fn rotate_up(&self) -> CubeSide {
        match (
            self.side.x,
            self.side.y,
            self.side.z,
            self.bottom_edge.x,
            self.bottom_edge.y,
            self.bottom_edge.z,
        ) {
            (0, 1, 0, 0, 1, -1)
            | (0, -1, 0, 0, -1, 1)
            | (0, 0, 1, 0, 1, 1)
            | (0, 0, -1, 0, -1, -1) => self.rotate_90_around_x(),
            (0, 1, 0, 0, 1, 1)
            | (0, -1, 0, 0, -1, -1)
            | (0, 0, 1, 0, -1, 1)
            | (0, 0, -1, 0, 1, -1) => self.rotate_270_around_x(),
            (0, 0, 1, -1, 0, 1)
            | (0, 0, -1, 1, 0, -1)
            | (1, 0, 0, 1, 0, 1)
            | (-1, 0, 0, -1, 0, -1) => self.rotate_90_around_y(),
            (0, 0, 1, 1, 0, 1)
            | (0, 0, -1, -1, 0, -1)
            | (1, 0, 0, 1, 0, -1)
            | (-1, 0, 0, -1, 0, 1) => self.rotate_270_around_y(),
            (1, 0, 0, 1, -1, 0)
            | (-1, 0, 0, -1, 1, 0)
            | (0, 1, 0, 1, 1, 0)
            | (0, -1, 0, -1, -1, 0) => self.rotate_90_around_z(),
            (1, 0, 0, 1, 1, 0)
            | (-1, 0, 0, -1, -1, 0)
            | (0, 1, 0, -1, 1, 0)
            | (0, -1, 0, 1, -1, 0) => self.rotate_270_around_z(),
            _ => unreachable!(),
        }
    }

    fn rotate_down(&self) -> CubeSide {
        self.rotate_side_90()
            .rotate_side_90()
            .rotate_up()
            .rotate_side_90()
            .rotate_side_90()
    }

    fn rotate_right(&self) -> CubeSide {
        self.rotate_side_90()
            .rotate_up()
            .rotate_side_90()
            .rotate_side_90()
            .rotate_side_90()
    }

    fn rotate_left(&self) -> CubeSide {
        self.rotate_side_90()
            .rotate_side_90()
            .rotate_side_90()
            .rotate_up()
            .rotate_side_90()
    }

    fn rotate_side_90(&self) -> CubeSide {
        match (self.side.x, self.side.y, self.side.z) {
            (1, 0, 0) => self.rotate_270_around_x(),
            (-1, 0, 0) => self.rotate_90_around_x(),
            (0, 1, 0) => self.rotate_270_around_y(),
            (0, -1, 0) => self.rotate_90_around_y(),
            (0, 0, 1) => self.rotate_270_around_z(),
            (0, 0, -1) => self.rotate_90_around_z(),
            _ => unreachable!(),
        }
    }

    fn rotate_90_around_x(&self) -> CubeSide {
        CubeSide::new(
            self.side.rotate_90_around_x(),
            self.bottom_edge.rotate_90_around_x(),
        )
    }

    fn rotate_270_around_x(&self) -> CubeSide {
        CubeSide::new(
            self.side.rotate_270_around_x(),
            self.bottom_edge.rotate_270_around_x(),
        )
    }

    fn rotate_90_around_y(&self) -> CubeSide {
        CubeSide::new(
            self.side.rotate_90_around_y(),
            self.bottom_edge.rotate_90_around_y(),
        )
    }

    fn rotate_270_around_y(&self) -> CubeSide {
        CubeSide::new(
            self.side.rotate_270_around_y(),
            self.bottom_edge.rotate_270_around_y(),
        )
    }

    fn rotate_90_around_z(&self) -> CubeSide {
        CubeSide::new(
            self.side.rotate_90_around_z(),
            self.bottom_edge.rotate_90_around_z(),
        )
    }

    fn rotate_270_around_z(&self) -> CubeSide {
        CubeSide::new(
            self.side.rotate_270_around_z(),
            self.bottom_edge.rotate_270_around_z(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Floor,
    Wall,
}

#[derive(Debug)]
struct Object {
    position: Vector2,
    direction: Direction,
}

impl Object {
    fn score(&self) -> i64 {
        (self.position.y + 1) * 1000
            + (self.position.x + 1) * 4
            + match self.direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn as_vector(&self) -> Vector2 {
        match self {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Forward(i64),
}

#[derive(Debug)]
struct Input {
    map: Map,
    moves: Vec<Move>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("\n\n").unwrap();

        let mut tiles = a
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        ' ' => Tile::Empty,
                        '.' => Tile::Floor,
                        '#' => Tile::Wall,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = tiles.iter().map(|row| row.len()).max().unwrap();
        tiles
            .iter_mut()
            .for_each(|row| row.resize_with(width, || Tile::Empty));

        let moves = many1(parse_move).parse(b.trim()).unwrap().1;

        Ok(Input {
            map: Map { tiles },
            moves,
        })
    }
}

fn parse_move(s: &str) -> IResult<&str, Move> {
    alt((
        map(char('L'), |_| Move::Left),
        map(char('R'), |_| Move::Right),
        map(digit1, |s: &str| Move::Forward(s.parse().unwrap())),
    ))
    .parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 6032.to_string());
        assert_eq!(result.1.to_string(), 5031.to_string());
    }

    #[test]
    fn rotations_go_back_to_original() {
        let starts = [
            CubeSide::new(Vector3::new(0, 0, 1), Vector3::new(0, -1, 1)),
            CubeSide::new(Vector3::new(0, -1, 0), Vector3::new(1, -1, 0)),
            CubeSide::new(Vector3::new(0, 0, -1), Vector3::new(-1, 0, -1)),
        ];
        for start in starts {
            let after_4_up = start.rotate_up().rotate_up().rotate_up().rotate_up();
            let after_4_down = start
                .rotate_down()
                .rotate_down()
                .rotate_down()
                .rotate_down();
            let after_4_left = start
                .rotate_left()
                .rotate_left()
                .rotate_left()
                .rotate_left();
            let after_4_right = start
                .rotate_right()
                .rotate_right()
                .rotate_right()
                .rotate_right();
            let after_right_up_left = start
                .rotate_right()
                .rotate_up()
                .rotate_left()
                .rotate_side_90();

            assert_eq!(start, after_4_up, "Up failed");
            assert_eq!(start, after_4_down, "Down failed");
            assert_eq!(start, after_4_left, "Left failed");
            assert_eq!(start, after_4_right, "Right failed");
            assert_eq!(start, after_right_up_left);
        }
    }
}
