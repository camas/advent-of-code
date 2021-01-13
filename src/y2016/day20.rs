use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut ranges = input
        .lines()
        .map(|l| l.parse::<Range>().unwrap())
        .collect::<Vec<_>>();

    // Combine ranges
    ranges.sort_unstable_by_key(|r| r.lower);
    let mut ranges = ranges.into_iter();
    let mut curr = ranges.next().unwrap();
    let mut combined = Vec::new();
    for range in ranges {
        if curr.upper == u32::MAX || range.lower <= curr.upper + 1 {
            curr.upper = curr.upper.max(range.upper);
        } else {
            combined.push(curr);
            curr = range;
        }
    }
    combined.push(curr);

    let part1 = combined[0].upper + 1;
    let part2 = combined
        .windows(2)
        .map(|w| w[1].lower - w[0].upper - 1)
        .sum::<u32>();

    (part1, part2)
}

#[derive(Debug)]
struct Range {
    lower: u32,
    upper: u32,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let lower = parts.next().unwrap().parse()?;
        let upper = parts.next().unwrap().parse()?;
        Ok(Range { lower, upper })
    }
}
