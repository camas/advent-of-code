use std::str::FromStr;

use crate::common::{Direction, Vector2};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|line| line.parse::<DigInstruction>().unwrap())
        .collect::<Vec<_>>();

    let basic_instructions = instructions
        .iter()
        .map(|instruction| (instruction.direction, instruction.length))
        .collect::<Vec<_>>();
    let part1 = find_area(&basic_instructions);

    let hex_instructions = instructions
        .iter()
        .map(|instruction| (instruction.hex_direction, instruction.hex_length))
        .collect::<Vec<_>>();
    let part2 = find_area(&hex_instructions);

    (part1, part2)
}

struct DigInstruction {
    direction: Direction,
    length: i64,
    hex_direction: Direction,
    hex_length: i64,
}

fn find_area(instructions: &[(Direction, i64)]) -> i64 {
    let mut count = 0;

    let mut position = Vector2::new(0, 0);
    let mut east_lines = Vec::new();
    let mut west_lines = Vec::new();
    let mut prev_direction = Direction::North;
    for (direction, length) in instructions {
        count += *length;
        let next_position = position + (*length * direction.as_vector2());
        if *direction == Direction::East {
            let mut start = position.x;
            if prev_direction == Direction::North {
                start += 1;
            }
            if next_position.x > start {
                east_lines.push((position.y, start..=next_position.x));
            }
        } else if *direction == Direction::West {
            west_lines.push((position.y, next_position.x..=position.x));
        } else if *direction == Direction::South && prev_direction == Direction::East {
            let (_, last_line) = east_lines.last_mut().unwrap();
            let new_end = last_line.end() - 1;
            if new_end < *last_line.start() {
                east_lines.pop();
            } else {
                *last_line = *last_line.start()..=new_end;
            }
        }
        position = next_position;
        prev_direction = *direction;
    }

    west_lines.sort_by_key(|(y, _)| *y);

    let mut to_check = east_lines;
    while let Some((y, range)) = to_check.pop() {
        let (other_y, other_range) = west_lines
            .iter()
            .find(|(other_y, other_range)| {
                *other_y > y
                    && !(range.start() > other_range.end() || range.end() < other_range.start())
            })
            .unwrap();

        let width = range.end().min(other_range.end()) - range.start().max(other_range.start()) + 1;
        let height = other_y - y - 1;
        count += width * height;

        if other_range.start() > range.start() {
            to_check.push((y, *range.start()..=(other_range.start() - 1)));
        }

        if other_range.end() < range.end() {
            to_check.push((y, (other_range.end() + 1)..=*range.end()))
        }
    }

    count
}

impl FromStr for DigInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = match parts.next().unwrap() {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => unreachable!(),
        };
        let length = parts.next().unwrap().parse::<i64>().unwrap();
        let color_part = parts.next().unwrap();
        let hex_direction = match &color_part[(color_part.len() - 2)..(color_part.len() - 1)] {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => unreachable!(),
        };
        let hex_length = i64::from_str_radix(&color_part[2..(color_part.len() - 2)], 16).unwrap();

        Ok(DigInstruction {
            direction,
            length,
            hex_direction,
            hex_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "62");
        assert_eq!(part2.to_string(), "952408144115");
    }
}
