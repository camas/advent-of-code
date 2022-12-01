use std::{collections::HashMap, iter, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut transforms = HashMap::new();
    for line in input.lines() {
        let mut patterns = line.split(" => ").map(|x| x.parse::<Pattern>().unwrap());
        let from = patterns.next().unwrap();
        let to = patterns.next().unwrap();
        for pattern in iter::once(from.flipped()).chain(iter::once(from)) {
            let rotated_90 = pattern.rotated_clockwise();
            let rotated_180 = rotated_90.rotated_clockwise();
            let rotated_270 = rotated_180.rotated_clockwise();
            transforms.insert(pattern, to.clone());
            transforms.insert(rotated_90, to.clone());
            transforms.insert(rotated_180, to.clone());
            transforms.insert(rotated_270, to.clone());
        }
    }

    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    for _ in 0..5 {
        grid = step(grid, &transforms);
    }

    let part1 = grid
        .iter()
        .map(|row| {
            row.iter()
                .fold(0, |acc, &curr| if curr { acc + 1 } else { acc })
        })
        .sum::<u32>();

    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    for _ in 0..18 {
        grid = step(grid, &transforms);
    }

    let part2 = grid
        .iter()
        .map(|row| {
            row.iter()
                .fold(0, |acc, &curr| if curr { acc + 1 } else { acc })
        })
        .sum::<u32>();

    (part1, part2)
}

fn step(grid: Vec<Vec<bool>>, transforms: &HashMap<Pattern, Pattern>) -> Vec<Vec<bool>> {
    if grid.len() % 2 == 0 {
        let blocks = (0..grid.len())
            .step_by(2)
            .map(|y| {
                (0..grid.len())
                    .step_by(2)
                    .map(|x| {
                        let block = vec![
                            vec![grid[y][x], grid[y][x + 1]],
                            vec![grid[y + 1][x], grid[y + 1][x + 1]],
                        ];
                        transforms.get(&Pattern { data: block }).unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut new_grid = Vec::new();
        for row in blocks {
            let inner_row_0 = row
                .iter()
                .flat_map(|r| r.data[0].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_0);
            let inner_row_1 = row
                .iter()
                .flat_map(|r| r.data[1].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_1);
            let inner_row_2 = row
                .iter()
                .flat_map(|r| r.data[2].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_2);
        }
        new_grid
    } else {
        let blocks = (0..grid.len())
            .step_by(3)
            .map(|y| {
                (0..grid.len())
                    .step_by(3)
                    .map(|x| {
                        let block = vec![
                            vec![grid[y][x], grid[y][x + 1], grid[y][x + 2]],
                            vec![grid[y + 1][x], grid[y + 1][x + 1], grid[y + 1][x + 2]],
                            vec![grid[y + 2][x], grid[y + 2][x + 1], grid[y + 2][x + 2]],
                        ];
                        transforms.get(&Pattern { data: block }).unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut new_grid = Vec::new();
        for row in blocks {
            let inner_row_0 = row
                .iter()
                .flat_map(|r| r.data[0].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_0);
            let inner_row_1 = row
                .iter()
                .flat_map(|r| r.data[1].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_1);
            let inner_row_2 = row
                .iter()
                .flat_map(|r| r.data[2].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_2);
            let inner_row_3 = row
                .iter()
                .flat_map(|r| r.data[3].clone())
                .collect::<Vec<_>>();
            new_grid.push(inner_row_3);
        }
        new_grid
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pattern {
    data: Vec<Vec<bool>>,
}

impl Pattern {
    fn flipped(&self) -> Pattern {
        let mut data = self.data.clone();
        for row in data.iter_mut() {
            row.reverse();
        }
        Pattern { data }
    }

    fn rotated_clockwise(&self) -> Pattern {
        let data = (0..self.data.len())
            .map(|y| {
                (0..self.data.len())
                    .map(|x| self.data[self.data.len() - x - 1][y])
                    .collect()
            })
            .collect();
        Pattern { data }
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern {
            data: s
                .split('/')
                .map(|section| {
                    section
                        .chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let mut transforms = HashMap::new();
        for line in input.lines() {
            let mut patterns = line.split(" => ").map(|x| x.parse::<Pattern>().unwrap());
            let from = patterns.next().unwrap();
            let to = patterns.next().unwrap();
            for pattern in iter::once(from.flipped()).chain(iter::once(from)) {
                let rotated_90 = pattern.rotated_clockwise();
                let rotated_180 = rotated_90.rotated_clockwise();
                let rotated_270 = rotated_180.rotated_clockwise();
                transforms.insert(pattern, to.clone());
                transforms.insert(rotated_90, to.clone());
                transforms.insert(rotated_180, to.clone());
                transforms.insert(rotated_270, to.clone());
            }
        }
        let grid = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];
        println!("{:?}", grid);
        let step_1 = step(grid, &transforms);
        println!("{:?}", step_1);
        let step_2 = step(step_1, &transforms);
        println!("{:?}", step_2);
    }

    #[test]
    fn test_rotation() {
        let pattern = Pattern {
            data: vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![false, false, false],
            ],
        };
        let rotated = pattern.rotated_clockwise();
        assert_eq!(
            rotated.data,
            vec![
                vec![false, false, true],
                vec![false, true, false],
                vec![false, false, true]
            ]
        );
    }

    #[test]
    fn test_flip() {
        let pattern = Pattern {
            data: vec![
                vec![true, false, true],
                vec![true, true, false],
                vec![false, false, false],
            ],
        };
        let flipped = pattern.flipped();
        assert_eq!(
            flipped.data,
            vec![
                vec![true, false, true],
                vec![false, true, true],
                vec![false, false, false]
            ]
        );
    }
}
