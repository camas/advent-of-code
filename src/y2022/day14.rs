use std::{collections::HashSet, str::FromStr};

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut cave = input.trim().parse::<Cave>().unwrap();

    let part1 = cave.calculate_sand();

    let floor_y = cave.max_y + 2;
    let floor_line = Line {
        start: Vector2::new(i64::MIN, floor_y),
        end: Vector2::new(i64::MAX, floor_y),
    };
    cave.lines.push(floor_line);
    cave.max_y += 3;
    let part2 = cave.calculate_sand();

    (part1, part2)
}

#[derive(Debug)]
struct Cave {
    max_y: i64,
    lines: Vec<Line>,
}

/// Start is always lower in one axis than end
#[derive(Debug)]
struct Line {
    start: Vector2,
    end: Vector2,
}

impl Cave {
    fn calculate_sand(&self) -> usize {
        let mut rested_sand = HashSet::new();

        let initial_pos = Vector2::new(500, 0);
        let mut sand_path = vec![initial_pos];
        while let Some(pos) = sand_path.last() {
            if !self.in_bounds(*pos) {
                break;
            }

            if self.in_lines(*pos) || rested_sand.contains(pos) {
                sand_path.pop();
                continue;
            }

            let next_pos = [(0, 1), (-1, 1), (1, 1)]
                .into_iter()
                .map(|(x, y)| *pos + Vector2::new(x, y))
                .find(|next_pos| !self.in_lines(*next_pos) && !rested_sand.contains(next_pos));
            match next_pos {
                Some(next_pos) => {
                    sand_path.push(next_pos);
                }
                None => {
                    rested_sand.insert(*pos);
                    sand_path.pop();
                }
            }
        }

        rested_sand.len()
    }

    fn in_bounds(&self, point: Vector2) -> bool {
        point.y <= self.max_y
    }

    fn in_lines(&self, point: Vector2) -> bool {
        self.lines.iter().any(|line| line.contains(point))
    }
}

impl Line {
    fn contains(&self, point: Vector2) -> bool {
        if self.start.x == self.end.x {
            point.x == self.start.x && point.y >= self.start.y && point.y <= self.end.y
        } else {
            point.y == self.start.y && point.x >= self.start.x && point.x <= self.end.x
        }
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .flat_map(|l| {
                let coords = l
                    .split(" -> ")
                    .map(|coords| {
                        let (a, b) = coords.split_once(',').unwrap();
                        Vector2::new(a.parse().unwrap(), b.parse().unwrap())
                    })
                    .collect::<Vec<_>>();
                coords
                    .windows(2)
                    .map(|window| {
                        let (start, end) = if window[0].x == window[1].x {
                            if window[0].y < window[1].y {
                                (window[0], window[1])
                            } else {
                                (window[1], window[0])
                            }
                        } else if window[0].y == window[1].y {
                            if window[0].x < window[1].x {
                                (window[0], window[1])
                            } else {
                                (window[1], window[0])
                            }
                        } else {
                            unreachable!()
                        };
                        Line { start, end }
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<Vec<_>>();

        let max_y = lines.iter().map(|l| l.start.y.max(l.end.y)).max().unwrap();

        Ok(Cave { max_y, lines })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 24.to_string());
        assert_eq!(result.1.to_string(), 93.to_string());
    }
}
