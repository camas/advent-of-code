use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let tiles = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut lines = s.lines();
            let id = lines.next().unwrap()[5..9].parse::<u64>().unwrap();
            let data = lines
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect();
            Tile::new(id, data)
        })
        .collect::<Vec<_>>();

    assert_eq!(tiles[0].data().len(), tiles[0].data()[0].len());
    let tile_size = tiles[0].data().len();

    // Connect tiles
    let mut unused = tiles;
    let mut map = HashMap::new();
    map.insert((0, 0), unused.pop().unwrap());
    let mut queue = vec![(0, 0)];
    while !queue.is_empty() {
        let tile_pos = queue.pop().unwrap();
        // Could do this without cloning
        let tile = map.get(&tile_pos).unwrap().clone();
        const DIRS: [(Side, (i32, i32)); 4] = [
            (Side::Top, (-1, 0)),
            (Side::Right, (0, 1)),
            (Side::Bottom, (1, 0)),
            (Side::Left, (0, -1)),
        ];
        for (dir, offset) in DIRS.iter() {
            let new_pos = (tile_pos.0 + offset.0, tile_pos.1 + offset.1);
            if map.contains_key(&new_pos) {
                continue;
            }
            let side = tile.side(*dir);
            // Find matching tile
            let match_result = unused.iter().enumerate().find_map(|(i, other)| {
                Side::all().find_map(|s| {
                    if sides_match(side, other.side(s)) {
                        Some((i, s))
                    } else {
                        None
                    }
                })
            });
            if match_result.is_none() {
                continue;
            }
            let (matching_index, matching_side) = match_result.unwrap();
            let mut matching_tile = unused.swap_remove(matching_index);

            // Rotate matching tile
            matching_tile.orientation = match (*dir, matching_side) {
                (Side::Top, other) | (Side::TopFlipped, other) => {
                    other.as_orientation().rot90().rot90()
                }
                (Side::Right, other) | (Side::RightFlipped, other) => {
                    other.as_orientation().rot90()
                }
                (Side::Bottom, other) | (Side::BottomFlipped, other) => other.as_orientation(),
                (Side::Left, other) | (Side::LeftFlipped, other) => {
                    other.as_orientation().rot90().rot90().rot90()
                }
            };
            // Insert into map
            map.insert(new_pos, matching_tile);
            // Add new pos to check queue
            queue.push(new_pos);
        }
    }
    assert!(unused.is_empty());
    let min_x = *map.keys().map(|(_, x)| x).min().unwrap();
    let max_x = *map.keys().map(|(_, x)| x).max().unwrap();
    let min_y = *map.keys().map(|(y, _)| y).min().unwrap();
    let max_y = *map.keys().map(|(y, _)| y).max().unwrap();

    let part1 = [
        (min_y, min_x),
        (min_y, max_x),
        (max_y, min_x),
        (max_y, max_x),
    ]
    .iter()
    .map(|pos| map.get(pos).unwrap().id)
    .product::<u64>();

    let mut search_data = (min_y..=max_y)
        .flat_map(|y| {
            let tile_row_data = (min_x..=max_x)
                .map(|x| map.get(&(y, x)).unwrap().data())
                .collect::<Vec<_>>();
            (1..(tile_size - 1)).map(move |i| {
                tile_row_data
                    .iter()
                    .flat_map(|tile| {
                        tile[i].iter().skip(1).take(tile_size - 2).map(|b| {
                            if *b {
                                SearchType::Full
                            } else {
                                SearchType::Empty
                            }
                        })
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    let monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    ' ' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let to_check = &[
        rot90(&monster),
        rot90(&rot90(&monster)),
        rot90(&rot90(&rot90(&monster))),
        flip(&monster),
        rot90(&flip(&monster)),
        rot90(&rot90(&flip(&monster))),
        rot90(&rot90(&rot90(&flip(&monster)))),
        monster,
    ];
    for monster_data in to_check.iter() {
        for start_y in 0..=(search_data.len() - monster_data.len()) {
            for start_x in 0..=(search_data[0].len() - monster_data[0].len()) {
                let is_match = monster_data
                    .iter()
                    .enumerate()
                    .all(|(monster_y, monster_row)| {
                        monster_row.iter().enumerate().all(|(monster_x, b)| {
                            let y = start_y + monster_y;
                            let x = start_x + monster_x;
                            !b || search_data[y][x] != SearchType::Empty
                        })
                    });
                if is_match {
                    monster_data
                        .iter()
                        .enumerate()
                        .for_each(|(monster_y, monster_row)| {
                            monster_row.iter().enumerate().for_each(|(monster_x, b)| {
                                if *b {
                                    let y = start_y + monster_y;
                                    let x = start_x + monster_x;
                                    search_data[y][x] = SearchType::SeaMonster;
                                }
                            });
                        });
                }
            }
        }
    }

    let part2 = search_data
        .iter()
        .map(|row| row.iter().filter(|v| **v == SearchType::Full).count())
        .sum::<usize>();

    (part1, part2)
}

fn sides_match(side_a: &[bool], side_b: &[bool]) -> bool {
    assert_eq!(side_a.len(), side_b.len());
    side_a.iter().zip(side_b.iter().rev()).all(|(a, b)| a == b)
}

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    data: Vec<Vec<bool>>,
    sides: Vec<Vec<bool>>,
    orientation: Orientation,
}

impl Tile {
    fn new(id: u64, data: Vec<Vec<bool>>) -> Self {
        // All sides are read clockwise
        let sides = vec![
            data[0].clone(),
            data.iter().map(|row| *row.last().unwrap()).collect(),
            data.last().unwrap().iter().rev().cloned().collect(),
            data.iter().map(|row| row[0]).rev().collect(),
            // Flipped
            data[0].iter().rev().cloned().collect(),
            data.iter().map(|row| row[0]).collect(),
            data.last().unwrap().clone(),
            data.iter().map(|row| *row.last().unwrap()).rev().collect(),
        ];
        Self {
            id,
            data,
            sides,
            orientation: Orientation::Normal,
        }
    }

    fn side(&self, side: Side) -> &Vec<bool> {
        let is_flipped = self.orientation.is_flipped() ^ side.is_flipped();
        let offset = (side as usize + self.orientation as usize) % Orientation::Flipped as usize;
        let index = if !is_flipped {
            offset
        } else {
            Orientation::Flipped as usize + offset
        };
        &self.sides[index]
    }

    fn data(&self) -> Vec<Vec<bool>> {
        match self.orientation {
            Orientation::Normal => self.data.clone(),
            Orientation::Rot90 => rot90(&self.data),
            Orientation::Rot180 => rot90(&rot90(&self.data)),
            Orientation::Rot270 => rot90(&rot90(&rot90(&self.data))),
            Orientation::Flipped => flip(&self.data),
            Orientation::FlipRot90 => rot90(&flip(&self.data)),
            Orientation::FlipRot180 => rot90(&rot90(&flip(&self.data))),
            Orientation::FlipRot270 => rot90(&rot90(&rot90(&flip(&self.data)))),
        }
    }
}

fn rot90(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let width = data[0].len();
    let height = data.len();
    (0..width)
        .map(|i| (0..height).map(|j| data[j][width - 1 - i]).collect())
        .collect()
}

fn flip(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
    data.iter()
        .map(|row| row.iter().rev().cloned().collect::<Vec<_>>())
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum SearchType {
    Empty,
    Full,
    SeaMonster,
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
    TopFlipped,
    RightFlipped,
    BottomFlipped,
    LeftFlipped,
}

impl Side {
    fn all() -> impl Iterator<Item = Side> {
        [
            Side::Top,
            Side::Right,
            Side::Bottom,
            Side::Left,
            Side::TopFlipped,
            Side::RightFlipped,
            Side::BottomFlipped,
            Side::LeftFlipped,
        ]
        .iter()
        .copied()
    }

    fn as_orientation(&self) -> Orientation {
        match self {
            Self::Top => Orientation::Normal,
            Self::Right => Orientation::Rot90,
            Self::Bottom => Orientation::Rot180,
            Self::Left => Orientation::Rot270,
            Self::TopFlipped => Orientation::Flipped,
            Self::RightFlipped => Orientation::FlipRot90,
            Self::BottomFlipped => Orientation::FlipRot180,
            Self::LeftFlipped => Orientation::FlipRot270,
        }
    }

    fn is_flipped(&self) -> bool {
        *self as usize >= Self::TopFlipped as usize
    }
}

/// Rotated anti-clockwise
#[derive(Debug, Clone, Copy)]
enum Orientation {
    Normal,
    Rot90,
    Rot180,
    Rot270,
    Flipped,
    FlipRot90,
    FlipRot180,
    FlipRot270,
}

impl Orientation {
    fn rot90(&self) -> Orientation {
        match self {
            Orientation::Normal => Orientation::Rot90,
            Orientation::Rot90 => Orientation::Rot180,
            Orientation::Rot180 => Orientation::Rot270,
            Orientation::Rot270 => Orientation::Normal,
            Orientation::Flipped => Orientation::FlipRot90,
            Orientation::FlipRot90 => Orientation::FlipRot180,
            Orientation::FlipRot180 => Orientation::FlipRot270,
            Orientation::FlipRot270 => Orientation::Flipped,
        }
    }

    fn is_flipped(&self) -> bool {
        *self as usize >= Orientation::Flipped as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sides() {
        let mut tile = Tile::new(
            0,
            vec![
                vec![true, false, true, true],
                vec![false, false, false, false],
                vec![false, false, false, true],
                vec![true, true, true, false],
            ],
        );
        assert_eq!(tile.side(Side::Top), &vec![true, false, true, true]);
        assert_eq!(tile.side(Side::Right), &vec![true, false, true, false]);
        assert_eq!(tile.side(Side::Bottom), &vec![false, true, true, true]);
        assert_eq!(tile.side(Side::Left), &vec![true, false, false, true]);
        assert_eq!(tile.side(Side::TopFlipped), &vec![true, true, false, true]);
        assert_eq!(
            tile.side(Side::RightFlipped),
            &vec![true, false, false, true]
        );
        assert_eq!(
            tile.side(Side::BottomFlipped),
            &vec![true, true, true, false]
        );
        assert_eq!(
            tile.side(Side::LeftFlipped),
            &vec![false, true, false, true]
        );

        tile.orientation = Orientation::Rot270;
        assert_eq!(
            tile.data(),
            vec![
                vec![true, false, false, true],
                vec![true, false, false, false],
                vec![true, false, false, true],
                vec![false, true, false, true],
            ]
        );
        assert_eq!(tile.side(Side::Top), &vec![true, false, false, true]);
        assert_eq!(tile.side(Side::Right), &vec![true, false, true, true]);
        assert_eq!(tile.side(Side::Bottom), &vec![true, false, true, false]);
        assert_eq!(tile.side(Side::Left), &vec![false, true, true, true]);

        tile.orientation = Orientation::FlipRot90;
        assert_eq!(
            tile.data(),
            vec![
                vec![true, false, false, true],
                vec![false, false, false, true],
                vec![true, false, false, true],
                vec![true, false, true, false],
            ]
        );
        assert_eq!(tile.side(Side::Top), &vec![true, false, false, true]);
        assert_eq!(tile.side(Side::Right), &vec![true, true, true, false]);
        assert_eq!(tile.side(Side::Bottom), &vec![false, true, false, true]);
        assert_eq!(tile.side(Side::Left), &vec![true, true, false, true]);

        tile.orientation = Orientation::FlipRot270;
        assert_eq!(
            tile.data(),
            vec![
                vec![false, true, false, true],
                vec![true, false, false, true],
                vec![true, false, false, false],
                vec![true, false, false, true],
            ]
        );
        assert_eq!(tile.side(Side::Top), &vec![false, true, false, true]);
        assert_eq!(tile.side(Side::Right), &vec![true, true, false, true]);
        assert_eq!(tile.side(Side::Bottom), &vec![true, false, false, true]);
        assert_eq!(tile.side(Side::Left), &vec![true, true, true, false]);
    }
}
