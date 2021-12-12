pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut octopuses = Octopuses::from_str(input);

    let mut part1 = None;
    let mut part2 = None;
    for i in 0.. {
        octopuses.step();
        if i == 101 {
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

        // Add all indexes to queue (only 100 so not too bad)
        for y in 0..self.height {
            for x in 0..self.width {
                queue.push((x as i8, y as i8));
            }
        }

        // Continue until all flashes handled
        while !queue.is_empty() {
            let (x, y) = queue.pop().unwrap();
            if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
                continue;
            }
            self.values[y as usize][x as usize] += 1;
            if self.values[y as usize][x as usize] == 10 {
                self.flash_count += 1;
                queue.push((x - 1, y));
                queue.push((x + 1, y));
                queue.push((x, y - 1));
                queue.push((x, y + 1));
                queue.push((x - 1, y - 1));
                queue.push((x + 1, y - 1));
                queue.push((x - 1, y + 1));
                queue.push((x + 1, y + 1));
            }
        }

        // Normalize
        for row in self.values.iter_mut() {
            for value in row.iter_mut() {
                *value = if *value <= 9 { *value } else { 0 };
            }
        }
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
        let data = r"5483143223
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
        assert_eq!(part1.to_string(), "1656");
    }
}
