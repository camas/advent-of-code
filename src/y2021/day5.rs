use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let lines = input.lines().map(Line::from_string).collect::<Vec<_>>();

    let straight_lines = lines.iter().filter(|l| l.straight());
    let mut overlaps = HashMap::<Point, u64>::new();
    for line in straight_lines {
        for point in line.points() {
            *overlaps.entry(point).or_default() += 1;
        }
    }
    let part1 = overlaps.values().filter(|v| **v > 1).count();

    let mut overlaps = HashMap::<Point, u64>::new();
    for line in lines.iter() {
        for point in line.points() {
            *overlaps.entry(point).or_default() += 1;
        }
    }
    let part2 = overlaps.values().filter(|v| **v > 1).count();

    (part1, part2)
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_string(s: &str) -> Line {
        let mut parts = s.split(" -> ");
        let start = Point::from_string(parts.next().unwrap());
        let end = Point::from_string(parts.next().unwrap());
        Line { start, end }
    }

    fn straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn points(&self) -> Vec<Point> {
        if self.straight() {
            if self.start.x == self.end.x {
                (min(self.start.y, self.end.y)..=max(self.start.y, self.end.y))
                    .map(|y| Point { x: self.start.x, y })
                    .collect()
            } else {
                (min(self.start.x, self.end.x)..=max(self.start.x, self.end.x))
                    .map(|x| Point { x, y: self.start.y })
                    .collect()
            }
        } else {
            let dist = (self.end.x - self.start.x).abs();
            let x_increasing = if self.start.x < self.end.x { 1 } else { -1 };
            let y_increasing = if self.start.y < self.end.y { 1 } else { -1 };
            (0..=dist)
                .map(|i| Point {
                    x: self.start.x + i * x_increasing,
                    y: self.start.y + i * y_increasing,
                })
                .collect()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn from_string(s: &str) -> Point {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let data = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let (part1, part2) = solve(data);
        assert_eq!(part1.to_string(), "5");
        assert_eq!(part2.to_string(), "12");
    }
}
