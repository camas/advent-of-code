use winnow::{
    ascii::dec_int,
    combinator::separated,
    error::{ContextError, ParserError, StrContext},
    Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let mut part1 = 0_i64;

    for id in input.ingredient_ids {
        if input.id_ranges.iter().any(|range| range.contains(id)) {
            part1 += 1;
        }
    }

    let mut ranges = input.id_ranges;
    loop {
        let Some((i, j)) =
            ranges[0..(ranges.len() - 1)]
                .iter()
                .enumerate()
                .find_map(|(i, range_i)| {
                    ranges
                        .iter()
                        .enumerate()
                        .skip(i + 1)
                        .find(|(j, range_j)| range_i.overlap(range_j))
                        .map(|(j, _)| (i, j))
                })
        else {
            break;
        };
        let range_j = ranges.swap_remove(j);
        let range_i = ranges.swap_remove(i);
        // println!("{range_i:?} {range_j:?}");
        ranges.push(range_i.merge(&range_j));
    }

    // for range in ranges.iter() {
    //     println!("{range:?}");
    // }

    let part2 = ranges.iter().map(|range| range.len()).sum::<i64>();

    (part1.to_string(), part2.to_string())
}

struct Input {
    id_ranges: Vec<Range>,
    ingredient_ids: Vec<i64>,
}

impl Input {
    fn parse(input: &str) -> Self {
        (
            separated(1.., Range::parse::<ContextError<_>>, '\n'),
            "\n\n",
            separated(1.., dec_int::<_, i64, ContextError<StrContext>>, '\n'),
        )
            .parse(input.trim_ascii_end())
            .map(|(id_ranges, _, ingredient_ids)| Self {
                id_ranges,
                ingredient_ids,
            })
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn parse<'a, E: ParserError<&'a str>>(input: &mut &'a str) -> winnow::Result<Self, E> {
        (dec_int, '-', dec_int)
            .parse_next(input)
            .map(|(start, _, end)| Self { start, end })
    }

    fn contains(&self, value: i64) -> bool {
        (self.start..=self.end).contains(&value)
    }

    fn overlap(&self, other: &Range) -> bool {
        (other.start <= self.start && other.end >= self.start)
            || (other.end >= self.end && other.start <= self.end)
            || (other.start >= self.start && other.end <= self.end)
            || (self.start >= other.start && self.end <= other.end)
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (result1, result2) = solve(input);
        assert_eq!(result1.to_string(), "3");
        assert_eq!(result2.to_string(), "14");
    }
}
