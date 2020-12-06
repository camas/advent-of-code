use std::{convert::TryInto, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lights = input.parse::<Lights>().unwrap();
    for _ in 0..100 {
        lights.step();
    }
    let part1 = lights
        .values
        .iter()
        .map(|row| row.iter().filter(|&&value| value).count())
        .sum::<usize>();

    let mut lights = input.parse::<Lights>().unwrap();
    lights.values[0][0] = true;
    lights.values[0][99] = true;
    lights.values[99][0] = true;
    lights.values[99][99] = true;
    for _ in 0..100 {
        lights.step_stuck();
    }
    let part2 = lights
        .values
        .iter()
        .map(|row| row.iter().filter(|&&value| value).count())
        .sum::<usize>();

    (part1, part2)
}

struct Lights {
    pub values: [[bool; 100]; 100],
}

impl Lights {
    pub fn step(&mut self) {
        let mut new_values = [[false; 100]; 100];
        for (y, row) in self.values.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                let neighbours = self.count_neighbours(x, y);
                new_values[y][x] = if value {
                    neighbours == 2 || neighbours == 3
                } else {
                    neighbours == 3
                };
            }
        }
        self.values = new_values;
    }

    pub fn step_stuck(&mut self) {
        let mut new_values = [[false; 100]; 100];
        for (y, row) in self.values.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                new_values[y][x] = if (x == 0 || x == 99) && (y == 0 || y == 99) {
                    true
                } else {
                    let neighbours = self.count_neighbours(x, y);
                    if value {
                        neighbours == 2 || neighbours == 3
                    } else {
                        neighbours == 3
                    }
                }
            }
        }
        self.values = new_values;
    }

    #[allow(clippy::let_and_return)]
    pub fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        let xmax = x == 99;
        let xmin = x == 0;
        let ymax = y == 99;
        let ymin = y == 0;
        let value = if !xmax && self.values[y][x + 1] { 1 } else { 0 }
            + if !xmin && self.values[y][x - 1] { 1 } else { 0 }
            + if !ymax && self.values[y + 1][x] { 1 } else { 0 }
            + if !ymin && self.values[y - 1][x] { 1 } else { 0 }
            + if !xmax && !ymax && self.values[y + 1][x + 1] {
                1
            } else {
                0
            }
            + if !xmax && !ymin && self.values[y - 1][x + 1] {
                1
            } else {
                0
            }
            + if !xmin && !ymax && self.values[y + 1][x - 1] {
                1
            } else {
                0
            }
            + if !xmin && !ymin && self.values[y - 1][x - 1] {
                1
            } else {
                0
            };
        value
    }
}

impl FromStr for Lights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        Ok(Self { values })
    }
}
