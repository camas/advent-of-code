use std::{collections::HashMap, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let passes = input
        .lines()
        .map(|line| line.parse::<Pass>().unwrap())
        .collect::<Vec<_>>();

    let part1 = passes.iter().map(|pass| pass.seat_id()).max().unwrap();

    let mut row_counts = HashMap::new();
    for pass in passes.iter() {
        *row_counts.entry(pass.get_row()).or_insert(0) += 1;
    }
    let my_row = *row_counts.iter().find(|(_, v)| v == &&7).unwrap().0;
    let my_column = (0..8)
        .find(|i| {
            !passes
                .iter()
                .any(|pass| pass.get_row() == my_row && pass.get_column() == *i)
        })
        .unwrap();

    (part1, (my_row * 8) + my_column)
}

struct Pass {
    row: Vec<Dir>,
    column: Vec<Dir>,
}

impl Pass {
    fn get_row(&self) -> u64 {
        bin_search(127, &self.row)
    }

    fn get_column(&self) -> u64 {
        bin_search(7, &self.column)
    }

    fn seat_id(&self) -> u64 {
        (self.get_row() * 8) + self.get_column()
    }
}

fn bin_search(higher: u64, dirs: &[Dir]) -> u64 {
    let mut higher = higher + 1;
    let mut lower = 0;
    for dir in dirs {
        let middle = (higher + lower) / 2;
        match dir {
            Dir::Higher => {
                lower = middle;
            }
            Dir::Lower => {
                higher = middle;
            }
        }
    }
    assert_eq!(lower, higher - 1);
    lower
}

impl FromStr for Pass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dirs = s.chars().map(|c| match c {
            'F' | 'L' => Dir::Lower,
            'B' | 'R' => Dir::Higher,
            _ => unreachable!(),
        });
        let row = dirs.by_ref().take(7).collect();
        let column = dirs.collect();
        Ok(Self { row, column })
    }
}

#[derive(Debug)]
enum Dir {
    Higher,
    Lower,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
        let pass = "FBFBBFFRLR".parse::<Pass>().unwrap();
        assert_eq!(pass.get_row(), 44);
        assert_eq!(pass.get_column(), 5);
    }
}
