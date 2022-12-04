use std::ops::RangeInclusive;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let range_pairs = input
        .trim()
        .lines()
        .map(|l| {
            let mut iter = l.split(',').map(|p| {
                let (a, b) = p.split_once('-').unwrap();
                a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
            });
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect::<Vec<_>>();

    let part1 = range_pairs
        .iter()
        .filter(|(a, b)| a.full_contains(b) || b.full_contains(a))
        .count();

    let part2 = range_pairs.iter().filter(|(a, b)| a.overlaps(b)).count();

    (part1, part2)
}

trait RangeExt {
    fn full_contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeExt for RangeInclusive<u64> {
    fn overlaps(&self, other: &Self) -> bool {
        !(self.end() < other.start() || self.start() > other.end())
    }

    fn full_contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 2.to_string());
        assert_eq!(result.1.to_string(), 4.to_string());
    }
}
