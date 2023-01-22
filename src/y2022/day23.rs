use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let elf_positions = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .map(|(x, y)| Vector2::new(x as i64, y as i64))
        .collect::<Vec<_>>();

    let mut problem = Problem {
        elf_positions,
        round: 0,
    };
    for _ in 0..10 {
        problem.do_round();
        // println!("{}", problem);
    }
    let part1 = problem.score();

    let mut part2 = 10 + 1;
    while !problem.do_round() {
        part2 += 1;
    }

    (part1, part2)
}

struct Problem {
    elf_positions: Vec<Vector2>,
    round: i64,
}

impl Problem {
    fn do_round(&mut self) -> bool {
        let positions = self.elf_positions.iter().cloned().collect::<HashSet<_>>();

        let possible_moves = self
            .elf_positions
            .iter()
            .map(|position| {
                let n_taken = positions.contains(&(*position + Direction::North.as_vector()));
                let e_taken = positions.contains(&(*position + Direction::East.as_vector()));
                let s_taken = positions.contains(&(*position + Direction::South.as_vector()));
                let w_taken = positions.contains(&(*position + Direction::West.as_vector()));
                let ne_taken = positions.contains(&(*position + Direction::NorthEast.as_vector()));
                let nw_taken = positions.contains(&(*position + Direction::NorthWest.as_vector()));
                let se_taken = positions.contains(&(*position + Direction::SouthEast.as_vector()));
                let sw_taken = positions.contains(&(*position + Direction::SouthWest.as_vector()));

                let can_move = n_taken
                    | e_taken
                    | s_taken
                    | w_taken
                    | ne_taken
                    | nw_taken
                    | se_taken
                    | sw_taken;
                let move_north = !(n_taken | ne_taken | nw_taken);
                let move_south = !(s_taken | se_taken | sw_taken);
                let move_west = !(nw_taken | w_taken | sw_taken);
                let move_east = !(ne_taken | e_taken | se_taken);

                [
                    can_move & move_north,
                    can_move & move_south,
                    can_move & move_west,
                    can_move & move_east,
                ]
            })
            .collect::<Vec<_>>();

        let moves = [
            Direction::North.as_vector(),
            Direction::South.as_vector(),
            Direction::West.as_vector(),
            Direction::East.as_vector(),
        ];
        let proposed_positions = self
            .elf_positions
            .iter()
            .zip(possible_moves.iter())
            .map(|(pos, possible)| {
                for i in self.round..(self.round + 4) {
                    let i = i % 4;
                    if possible[i as usize] {
                        return *pos + moves[i as usize];
                    }
                }
                *pos
            })
            .collect::<Vec<_>>();

        let new_positions = self
            .elf_positions
            .iter()
            .enumerate()
            .map(|(i, original_position)| {
                let proposed = proposed_positions[i];
                if proposed_positions
                    .iter()
                    .filter(|p| **p == proposed)
                    .count()
                    < 2
                {
                    proposed
                } else {
                    *original_position
                }
            })
            .collect();

        let changed = self.elf_positions == new_positions;
        self.elf_positions = new_positions;
        self.round += 1;

        changed
    }

    fn score(&self) -> i64 {
        let min_x = self.elf_positions.iter().map(|pos| pos.x).min().unwrap();
        let min_y = self.elf_positions.iter().map(|pos| pos.y).min().unwrap();
        let max_x = self.elf_positions.iter().map(|pos| pos.x).max().unwrap();
        let max_y = self.elf_positions.iter().map(|pos| pos.y).max().unwrap();

        ((max_x - min_x).abs() + 1) * ((max_y - min_y).abs() + 1) - self.elf_positions.len() as i64
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.elf_positions.iter().map(|pos| pos.x).min().unwrap();
        let min_y = self.elf_positions.iter().map(|pos| pos.y).min().unwrap();
        let max_x = self.elf_positions.iter().map(|pos| pos.x).max().unwrap();
        let max_y = self.elf_positions.iter().map(|pos| pos.y).max().unwrap();

        f.write_fmt(format_args!("Round: {}\n", self.round))?;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elf_positions.contains(&Vector2::new(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn as_vector(&self) -> Vector2 {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
            Direction::NorthEast => Vector2::new(1, -1),
            Direction::NorthWest => Vector2::new(-1, -1),
            Direction::SouthEast => Vector2::new(1, 1),
            Direction::SouthWest => Vector2::new(-1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 110.to_string());
        assert_eq!(result.1.to_string(), 20.to_string());
    }
}
