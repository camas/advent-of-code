use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let map = parse_input(input);

    let mut part1 = 0;
    let mut part2 = 0;
    for (i, galaxy) in map.galaxies.iter().enumerate() {
        for other_galaxy in map.galaxies.iter().skip(i + 1) {
            let abs_difference = (galaxy.x - other_galaxy.x).unsigned_abs() as usize
                + (galaxy.y - other_galaxy.y).unsigned_abs() as usize;
            let empty_rows_between = map
                .empty_rows
                .iter()
                .filter(|row_y| {
                    (galaxy.y.min(other_galaxy.y)..=galaxy.y.max(other_galaxy.y))
                        .contains(&(**row_y as i64))
                })
                .count()
                + map
                    .empty_columns
                    .iter()
                    .filter(|row_x| {
                        (galaxy.x.min(other_galaxy.x)..=galaxy.x.max(other_galaxy.x))
                            .contains(&(**row_x as i64))
                    })
                    .count();
            part1 += abs_difference + empty_rows_between;
            part2 += abs_difference + empty_rows_between * 999999;
        }
    }

    (part1, part2)
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<Vector2>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

fn parse_input(input: &str) -> Map {
    let mut galaxies = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_columns = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row_empty = true;
        for (x, char) in line.chars().enumerate() {
            if y == 0 && empty_columns.len() < x + 1 {
                empty_columns.push(true);
            }
            if char == '#' {
                row_empty = false;
                empty_columns[x] = false;
                galaxies.push(Vector2::new(x as i64, y as i64));
            }
        }
        if row_empty {
            empty_rows.push(y);
        }
    }

    Map {
        galaxies,
        empty_rows,
        empty_columns: empty_columns
            .into_iter()
            .enumerate()
            .filter(|(_, empty)| *empty)
            .map(|(i, _)| i)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let (part1, _) = solve(input);

        assert_eq!(part1.to_string(), "374");
    }
}
