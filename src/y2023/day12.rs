use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let records = input
        .lines()
        .map(|line| line.parse::<Record>().unwrap())
        .collect::<Vec<_>>();

    let part1 = records.iter().map(Record::count_arrangements).sum::<u64>();

    let part2 = records
        .into_iter()
        .map(|record| {
            let mut new_springs = Vec::new();
            new_springs.extend(&record.springs);
            for _ in 0..4 {
                new_springs.push(Spring::Unknown);
                new_springs.extend(&record.springs);
            }

            let mut new_group_sizes = Vec::new();
            for _ in 0..5 {
                new_group_sizes.extend(&record.group_sizes);
            }

            Record {
                springs: new_springs,
                group_sizes: new_group_sizes,
            }
            .count_arrangements()
        })
        .sum::<u64>();

    (part1, part2)
}

struct Record {
    springs: Vec<Spring>,
    group_sizes: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Record {
    fn count_arrangements(&self) -> u64 {
        let mut starts_and_counts = HashMap::new();
        starts_and_counts.insert(0, 1);

        let mut remaining_size = self.group_sizes.iter().sum::<usize>() + self.group_sizes.len();

        for group_size in self.group_sizes.iter() {
            remaining_size -= group_size + 1;

            // println!("{:?}", starts_and_counts);
            // println!("{:?}", remaining_size);

            let mut new_starts_and_counts = HashMap::new();
            for (start, count) in starts_and_counts.into_iter() {
                for i in (start..=(self.springs.len() - remaining_size - group_size))
                    .take_while_inclusive(|i| self.springs[*i] != Spring::Damaged)
                {
                    if !self.fits_group(i, *group_size) {
                        continue;
                    }

                    let next_start = i + group_size + 1;

                    *new_starts_and_counts.entry(next_start).or_insert(0) += count;
                }
            }
            starts_and_counts = new_starts_and_counts;
        }

        starts_and_counts
            .into_iter()
            .filter(|(start, _)| {
                let start = start - 1;
                start >= self.springs.len()
                    || self.springs[start..]
                        .iter()
                        .all(|spring| matches!(spring, Spring::Operational | Spring::Unknown))
            })
            .map(|(_, count)| count)
            .sum()
    }

    fn fits_group(&self, start_index: usize, group_size: usize) -> bool {
        start_index + group_size <= self.springs.len()
            && (start_index == 0
                || matches!(
                    self.springs[start_index - 1],
                    Spring::Operational | Spring::Unknown
                ))
            && self.springs[start_index..(start_index + group_size)]
                .iter()
                .all(|spring| matches!(spring, Spring::Damaged | Spring::Unknown))
            && (start_index + group_size == self.springs.len()
                || matches!(
                    self.springs[start_index + group_size],
                    Spring::Operational | Spring::Unknown
                ))
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, group_sizes) = s.split_once(' ').unwrap();
        Ok(Record {
            springs: springs
                .chars()
                .map(|char| match char {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect(),
            group_sizes: group_sizes
                .split(',')
                .map(|size| size.parse().unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "21");
        assert_eq!(part2.to_string(), "525152");
    }
}
