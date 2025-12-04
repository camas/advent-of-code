use std::collections::HashSet;

use winnow::{
    combinator::{repeat, separated},
    error::ContextError,
    token::one_of,
    Parser,
};

const ADJACENT_DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let mut part1 = 0_i64;

    let width = input.grid[0].len() as i64;
    let height = input.grid.len() as i64;

    let mut count_grid = Vec::new();
    let mut to_remove = Vec::new();

    for (y, row) in input.grid.iter().enumerate() {
        let mut count_row = Vec::new();
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Empty {
                count_row.push(0);
                continue;
            }

            let adjacent = ADJACENT_DIRECTIONS
                .iter()
                .map(|(dir_x, dir_y)| (dir_x + x as i64, dir_y + y as i64))
                .filter(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
                .filter(|&(x, y)| input.grid[y as usize][x as usize] == Tile::Paper)
                .count();

            if adjacent < 4 {
                part1 += 1;
                to_remove.push((x as i64, y as i64));
            }

            count_row.push(adjacent);
        }
        count_grid.push(count_row);
    }

    let mut removed = HashSet::new();
    while let Some((remove_x, remove_y)) = to_remove.pop() {
        if !removed.insert((remove_x, remove_y)) {
            continue;
        }

        for (adjacent_x, adjacent_y) in ADJACENT_DIRECTIONS
            .iter()
            .map(|(dir_x, dir_y)| (dir_x + remove_x, dir_y + remove_y))
            .filter(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
        {
            let count = count_grid[adjacent_y as usize][adjacent_x as usize];
            if count > 0 {
                count_grid[adjacent_y as usize][adjacent_x as usize] -= 1;
            }
            if count == 4 {
                to_remove.push((adjacent_x, adjacent_y));
            }
        }
    }

    (part1.to_string(), removed.len().to_string())
}

struct Input {
    grid: Vec<Vec<Tile>>,
}

impl Input {
    fn parse(input: &str) -> Self {
        separated(
            1..,
            repeat::<_, _, Vec<Tile>, _, _>(
                1..,
                one_of::<_, _, ContextError<&str>>(['.', '@']).map(|c: char| match c {
                    '.' => Tile::Empty,
                    '@' => Tile::Paper,
                    _ => unreachable!(),
                }),
            ),
            '\n',
        )
        .parse(input.trim_ascii_end())
        .map(|grid| Self { grid })
        .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Paper,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let (result1, result2) = solve(input);
        assert_eq!(result1.to_string(), "13");
        assert_eq!(result2.to_string(), "43");
    }
}
