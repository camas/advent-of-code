pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let maps = parse_input(input);

    let part1 = maps
        .iter()
        .map(|map| {
            let (direction, offset) = map.find_reflection();
            match direction {
                LineDirection::Vertical => offset,
                LineDirection::Horizontal => offset * 100,
            }
        })
        .sum::<usize>();

    let part2 = maps
        .iter()
        .map(|map| {
            let (direction, offset) = map.find_smudged_reflection();
            match direction {
                LineDirection::Vertical => offset,
                LineDirection::Horizontal => offset * 100,
            }
        })
        .sum::<usize>();

    (part1, part2)
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rocks,
}

enum LineDirection {
    Vertical,
    Horizontal,
}

impl Map {
    fn find_reflection(&self) -> (LineDirection, usize) {
        let mut possible_vertical = (1..self.width()).collect::<Vec<_>>();
        for row in self.tiles.iter() {
            possible_vertical.retain(|&offset| {
                row.iter()
                    .take(offset)
                    .rev()
                    .zip(row.iter().skip(offset))
                    .all(|(a, b)| a == b)
            });

            if possible_vertical.is_empty() {
                break;
            }
        }

        debug_assert!(possible_vertical.len() <= 1);
        if possible_vertical.len() == 1 {
            return (LineDirection::Vertical, possible_vertical[0]);
        }

        let mut possible_horizontal = (1..self.height()).collect::<Vec<_>>();
        for column_index in 0..self.width() {
            possible_horizontal.retain(|&offset| {
                let left = self
                    .tiles
                    .iter()
                    .take(offset)
                    .map(|row| &row[column_index])
                    .rev();
                let right = self.tiles.iter().skip(offset).map(|row| &row[column_index]);

                left.zip(right).all(|(a, b)| a == b)
            });

            if possible_horizontal.len() <= 1 {
                break;
            }
        }

        debug_assert!(possible_horizontal.len() == 1);
        (LineDirection::Horizontal, possible_horizontal[0])
    }

    fn find_smudged_reflection(&self) -> (LineDirection, usize) {
        for vertical_index in 1..self.width() {
            let differences = self
                .tiles
                .iter()
                .map(|row| {
                    row.iter()
                        .take(vertical_index)
                        .rev()
                        .zip(row.iter().skip(vertical_index))
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>();

            if differences == 1 {
                return (LineDirection::Vertical, vertical_index);
            }
        }

        for horizontal_index in 1..self.height() {
            let differences = (0..self.width())
                .map(|column_index| {
                    let left = self
                        .tiles
                        .iter()
                        .take(horizontal_index)
                        .map(|row| &row[column_index])
                        .rev();
                    let right = self
                        .tiles
                        .iter()
                        .skip(horizontal_index)
                        .map(|row| &row[column_index]);

                    left.zip(right).filter(|(a, b)| a != b).count()
                })
                .sum::<usize>();

            if differences == 1 {
                return (LineDirection::Horizontal, horizontal_index);
            }
        }

        unreachable!()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }
}

fn parse_input(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .map(|map| {
            let tiles = map
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| match char {
                            '.' => Tile::Ash,
                            '#' => Tile::Rocks,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect();
            Map { tiles }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "405");
        assert_eq!(part2.to_string(), "400");
    }
}
