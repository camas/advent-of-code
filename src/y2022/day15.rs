use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    solve_inner(input, 2_000_000)
}

fn solve_inner(input: &str, row: i64) -> (impl ToString, impl ToString) {
    let sensor_beacon_pairs = input
        .trim()
        .lines()
        .map(|l| l.parse::<SensorBeaconPair>().unwrap())
        .collect::<Vec<_>>();
    let beacons = sensor_beacon_pairs
        .iter()
        .map(|p| p.beacon)
        .collect::<HashSet<_>>();

    let sensor_ranges = sensor_beacon_pairs
        .iter()
        .map(|p| SensorRange {
            position: p.sensor,
            size: p.sensor.manhattan_distance(p.beacon),
        })
        .collect::<Vec<_>>();

    let sensor_y_ranges = sensor_ranges
        .iter()
        .filter_map(|s| s.range_at_y(row))
        .sorted()
        .collect::<Vec<_>>();

    let beacons_on_y = beacons.iter().filter(|beacon| beacon.y == row).count();

    let part1 = Range::merge_vec(sensor_y_ranges)
        .iter()
        .map(|r| r.size())
        .sum::<i64>()
        - beacons_on_y as i64;

    let mut perimeter_points = sensor_ranges
        .iter()
        .flat_map(|sr| sr.perimeter().into_iter());
    let distress_location = perimeter_points
        .find(|p| {
            p.x >= 0
                && p.x <= row * 2
                && p.y >= 0
                && p.y <= row * 2
                && !sensor_ranges
                    .iter()
                    .any(|sr| sr.position.manhattan_distance(*p) <= sr.size)
        })
        .unwrap();

    let part2 = distress_location.x * 4000000 + distress_location.y;

    (part1, part2)
}

/// Inclusive range
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn size(&self) -> i64 {
        self.end - self.start + 1
    }

    fn overlap(&self, other: &Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start > end {
            return None;
        }

        Some(Range { start, end })
    }

    fn merge_vec(mut ranges: Vec<Range>) -> Vec<Range> {
        let mut i = 0;
        while i < ranges.len() {
            let curr = &ranges[i];
            match ranges
                .iter()
                .enumerate()
                .skip(i + 1)
                .find_map(|(j, r)| curr.merge(r).map(|v| (j, v)))
            {
                Some((other_i, merged)) => {
                    ranges[i] = merged;
                    ranges.swap_remove(other_i);
                    i = 0;
                }
                None => i += 1,
            }
        }
        ranges
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        self.overlap(other).map(|overlap| Range {
            start: self.start.min(other.start).min(overlap.start),
            end: self.end.max(other.end).max(overlap.end),
        })
    }
}

#[derive(Debug)]
struct SensorRange {
    position: Vector2,
    size: i64,
}

impl SensorRange {
    fn range_at_y(&self, y: i64) -> Option<Range> {
        let y_diff = (self.position.y - y).abs();
        if y_diff > self.size {
            return None;
        }

        let width_at_y = self.size - y_diff;
        let min_x = self.position.x - width_at_y;
        let max_x = self.position.x + width_at_y;

        Some(Range {
            start: min_x,
            end: max_x,
        })
    }

    fn perimeter(&self) -> Vec<Vector2> {
        let perimeter_size = self.size + 1;
        let mut perimeter_points = Vec::new();
        for i in 0..=perimeter_size {
            perimeter_points.push(Vector2::new(
                self.position.x + i,
                self.position.y + perimeter_size - i,
            ));
            perimeter_points.push(Vector2::new(
                self.position.x - i,
                self.position.y + perimeter_size - i,
            ));
            perimeter_points.push(Vector2::new(
                self.position.x + i,
                self.position.y - perimeter_size + i,
            ));
            perimeter_points.push(Vector2::new(
                self.position.x - i,
                self.position.y - perimeter_size + i,
            ));
        }
        perimeter_points
    }
}

#[derive(Debug)]
struct SensorBeaconPair {
    sensor: Vector2,
    beacon: Vector2,
}

impl FromStr for SensorBeaconPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_number(s: &str) -> i64 {
            s.trim_matches(|c: char| !matches!(c, '0'..='9' | '-'))
                .parse()
                .unwrap()
        }

        let parts = s.split(' ').collect::<Vec<_>>();
        let sensor_x = parse_number(parts[2]);
        let sensor_y = parse_number(parts[3]);
        let beacon_x = parse_number(parts[8]);
        let beacon_y = parse_number(parts[9]);

        let sensor = Vector2::new(sensor_x, sensor_y);
        let beacon = Vector2::new(beacon_x, beacon_y);

        Ok(SensorBeaconPair { sensor, beacon })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

        let result = solve_inner(input, 10);

        assert_eq!(result.0.to_string(), 26.to_string());
        assert_eq!(result.1.to_string(), 56000011.to_string());
    }
}
