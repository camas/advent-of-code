use itertools::Itertools;
use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::separated,
    PResult, Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = parse_input(input);

    let left_sorted = input
        .pairs
        .iter()
        .map(|(left, _)| *left)
        .sorted()
        .collect::<Vec<_>>();

    let right_sorted = input
        .pairs
        .iter()
        .map(|(_, right)| *right)
        .sorted()
        .collect::<Vec<_>>();

    let part1 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(i, j)| i.abs_diff(*j))
        .sum::<u32>();

    let mut last = 0;
    let mut last_calc = 0;
    let mut right_index = 0;
    let mut part2 = 0;
    for left_value in left_sorted {
        if left_value == last {
            part2 += last_calc;
            continue;
        }

        while right_index < right_sorted.len() && left_value > right_sorted[right_index] {
            right_index += 1;
        }
        if right_index >= right_sorted.len() {
            break;
        }

        let mut right_count = 0;
        loop {
            if left_value != right_sorted[right_index] {
                break;
            }
            right_count += 1;
            right_index += 1;
            if right_index >= right_sorted.len() {
                break;
            }
        }

        last = left_value;
        let similarity_score = left_value * right_count;
        last_calc = similarity_score;
        part2 += similarity_score;

        if right_index >= right_sorted.len() {
            break;
        }
    }

    (part1, part2)
}

struct Input {
    pairs: Vec<(u32, u32)>,
}

fn parse_input(input: &str) -> Input {
    (separated(1.., parse_pair, "\n"), multispace0)
        .map(|(pairs, _)| Input { pairs })
        .parse(input)
        .unwrap()
}

fn parse_pair(input: &mut &str) -> PResult<(u32, u32)> {
    (parse_number, multispace1, parse_number)
        .map(|(a, _, b)| (a, b))
        .parse_next(input)
}

fn parse_number(input: &mut &str) -> PResult<u32> {
    digit1
        .map(|digits: &str| digits.parse::<u32>().unwrap())
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "11");
        assert_eq!(part2.to_string(), "31");
    }
}
