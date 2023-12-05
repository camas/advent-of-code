use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, map},
    multi::{many0, separated_list1},
    sequence::tuple,
    IResult,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let almanac = parse_almanac(input).unwrap().1;

    let mut values = almanac.seeds.clone();
    for almanac_map in almanac.maps.iter() {
        values = values.into_iter().map(|v| almanac_map.apply(v)).collect();
    }
    let part1 = *values.iter().min().unwrap();

    let mut ranges = almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<_>>();
    for almanac_map in almanac.maps.iter() {
        ranges = ranges
            .into_iter()
            .flat_map(|range| almanac_map.apply_range(range))
            .collect();
    }
    let part2 = ranges.into_iter().map(|range| range.start).min().unwrap();

    (part1, part2)
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<AlmanacMap>,
}

struct AlmanacMap {
    _source_name: String,
    _destination_name: String,
    entries: Vec<AlmanacMapEntry>,
}

struct AlmanacMapEntry {
    source_start: i64,
    destination_start: i64,
    length: i64,
}

impl AlmanacMap {
    fn apply(&self, value: i64) -> i64 {
        self.entries
            .iter()
            .find_map(|entry| {
                if entry.range().contains(&value) {
                    Some(value + (entry.destination_start - entry.source_start))
                } else {
                    None
                }
            })
            .unwrap_or(value)
    }

    fn apply_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut remaining_ranges = vec![range];
        let mut results = Vec::new();
        for entry in self.entries.iter() {
            remaining_ranges = remaining_ranges
                .into_iter()
                .flat_map(|range| {
                    if let Some((overlap, remaining)) = intersect(range.clone(), entry.range()) {
                        let entry_diff = entry.destination_start - entry.source_start;
                        results.push((overlap.start + entry_diff)..(overlap.end + entry_diff));
                        remaining
                    } else {
                        vec![range]
                    }
                })
                .collect();
        }

        results.extend(remaining_ranges);
        results
    }
}

impl AlmanacMapEntry {
    fn range(&self) -> Range<i64> {
        self.source_start..(self.source_start + self.length)
    }
}

// Returns (overlap, remaining ranges)
fn intersect(input: Range<i64>, against: Range<i64>) -> Option<(Range<i64>, Vec<Range<i64>>)> {
    if input.end <= against.start || input.start >= against.end {
        return None;
    }

    let overlap = input.start.max(against.start)..input.end.min(against.end);

    let mut remaining = Vec::new();
    if input.start != overlap.start {
        remaining.push(input.start..overlap.start);
    }
    if input.end != overlap.end {
        remaining.push(overlap.end..input.end);
    }

    Some((overlap, remaining))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    all_consuming(map(
        tuple((
            parse_seeds,
            tag("\n\n"),
            separated_list1(tag("\n\n"), parse_map),
            many0(tag("\n")),
        )),
        |(seeds, _, maps, _)| Almanac { seeds, maps },
    ))(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    map(
        tuple((tag("seeds: "), separated_list1(tag(" "), digit1))),
        |(_, digit_list)| {
            digit_list
                .into_iter()
                .map(|digits: &str| digits.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        },
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, AlmanacMap> {
    map(
        tuple((
            alpha1::<&str, _>,
            tag("-to-"),
            alpha1,
            tag(" map:\n"),
            separated_list1(
                tag("\n"),
                tuple((digit1, tag(" "), digit1, tag(" "), digit1)),
            ),
        )),
        |(source_name, _, destination_name, _, entries)| AlmanacMap {
            _source_name: source_name.to_string(),
            _destination_name: destination_name.to_string(),
            entries: entries
                .into_iter()
                .map(
                    |(destination_start, _, source_start, _, length)| AlmanacMapEntry {
                        source_start: source_start.parse::<i64>().unwrap(),
                        destination_start: destination_start.parse::<i64>().unwrap(),
                        length: length.parse::<i64>().unwrap(),
                    },
                )
                .collect(),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "35");
        assert_eq!(part2.to_string(), "46");
    }
}
