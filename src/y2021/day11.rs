use std::collections::HashSet;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut octopuses = Octopuses::from_str(input.trim());

    let mut part1 = None;
    let mut part2 = None;
    for i in 0.. {
        octopuses.step();
        if i == 99 {
            part1 = Some(octopuses.flash_count);
        }
        if octopuses.all_zero() {
            part2 = Some(i + 1);
            break;
        }
    }

    (part1.unwrap(), part2.unwrap())
}

struct Octopuses {
    values: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    flash_count: u64,
}

impl Octopuses {
    fn from_str(data: &str) -> Self {
        let values = data
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let width = values[0].len();
        let height = values.len();
        Self {
            values,
            width,
            height,
            flash_count: 0,
        }
    }

    fn step(&mut self) {
        let mut queue = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                self.values[y][x] += 1;
                if self.values[y][x] > 9 {
                    queue.push((x as i64, y as i64));
                }
            }
        }

        // Continue until all flashes handled
        let mut seen = HashSet::new();
        while let Some((x, y)) = queue.pop() {
            if !seen.insert((x, y)) {
                continue;
            }

            self.flash_count += 1;

            let adjacent = [
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ];
            for (adj_x, adj_y) in adjacent {
                if !self.in_bounds(adj_x, adj_y) {
                    continue;
                }
                self.values[adj_y as usize][adj_x as usize] += 1;
                if self.values[adj_y as usize][adj_x as usize] > 9 {
                    queue.push((adj_x, adj_y));
                }
            }
        }

        // Normalize
        for row in self.values.iter_mut() {
            for value in row.iter_mut() {
                if *value > 9 {
                    *value = 0;
                }
            }
        }
    }

    fn in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    fn all_zero(&self) -> bool {
        self.values.iter().all(|row| row.iter().all(|v| *v == 0))
    }
}

impl std::fmt::Debug for Octopuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.values.iter() {
            for value in row.iter() {
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let data = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        let (part1, part2) = solve(data);

        assert_eq!(part1.to_string(), 1656.to_string());
        assert_eq!(part2.to_string(), 195.to_string());
    }
}
