use winnow::{ascii::multispace0, combinator::separated, Parser};

use crate::common::parse_u32;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let part1 = input
        .reports
        .iter()
        .filter(|report| report.is_safe1())
        .count();

    let part2 = input
        .reports
        .iter()
        .filter(|report| report.is_safe2())
        .count();

    (part1, part2)
}

struct Input {
    reports: Vec<Report>,
}

impl Input {
    fn parse(input: &str) -> Input {
        (separated(1.., Report::parse, "\n"), multispace0)
            .map(|(reports, _)| Input { reports })
            .parse(input)
            .unwrap()
    }
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn parse(input: &mut &str) -> winnow::Result<Report> {
        separated(1.., parse_u32, " ")
            .map(|levels| Report { levels })
            .parse_next(input)
    }

    fn is_safe1(&self) -> bool {
        is_safe_naive(&self.levels)
    }

    fn is_safe2(&self) -> bool {
        if self.levels[0] == self.levels[1] {
            return is_safe_naive(&self.levels[1..]);
        }

        if is_safe_naive(&self.levels) {
            return true;
        }

        let increasing = self.levels[0] < self.levels[1];
        let first_wrong_position = self
            .levels
            .windows(2)
            .position(|pair| {
                !(1..=3).contains(&pair[0].abs_diff(pair[1]))
                    || (increasing && pair[0] > pair[1])
                    || (!increasing && pair[0] < pair[1])
            })
            .unwrap();

        let levels_with_first_removed = copy_without_index(&self.levels, first_wrong_position);
        if is_safe_naive(&levels_with_first_removed) {
            return true;
        }

        let levels_with_second_removed = copy_without_index(&self.levels, first_wrong_position + 1);
        if is_safe_naive(&levels_with_second_removed) {
            return true;
        }

        if first_wrong_position != 0 {
            let levels_with_prev_removed =
                copy_without_index(&self.levels, first_wrong_position - 1);
            if is_safe_naive(&levels_with_prev_removed) {
                return true;
            }
        }

        false
    }
}

fn is_safe_naive(levels: &[u32]) -> bool {
    // Not optimal using this everywhere instead of doing it in one loop
    // Ho-hum
    (levels.windows(2).all(|pair| pair[0] >= pair[1])
        || levels.windows(2).all(|pair| pair[0] <= pair[1]))
        && levels
            .windows(2)
            .all(|pair| (1..=3).contains(&pair[0].abs_diff(pair[1])))
}

fn copy_without_index(values: &[u32], index: usize) -> Vec<u32> {
    values
        .iter()
        .take(index)
        .chain(values.iter().skip(index + 1))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "2");
        assert_eq!(part2.to_string(), "4");
    }
}
