use std::collections::{HashMap, HashSet};

use num::Signed;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let wires = input
        .lines()
        .map(|l| l.split(',').map(Move::from_str).collect::<Vec<_>>());

    // Convert to list of points
    let points = wires
        .map(|wire| {
            let mut points = Vec::new();
            let mut x = 0;
            let mut y = 0;
            for m in wire {
                let dist = m.dist();
                for _ in 0..dist {
                    match m {
                        Move::Up(_) => y -= 1,
                        Move::Down(_) => y += 1,
                        Move::Left(_) => x -= 1,
                        Move::Right(_) => x += 1,
                    }
                    points.push((x, y));
                }
            }
            points
        })
        .collect::<Vec<_>>();

    let wire2_hashed = points[1].iter().collect::<HashSet<_>>();
    let overlaps = points[0]
        .iter()
        .filter(|p| wire2_hashed.contains(p))
        .collect::<Vec<_>>();

    let closest = overlaps
        .iter()
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap();
    let part1 = closest;

    let wire_distances = points
        .iter()
        .map(|wire| {
            let mut distances = HashMap::new();
            for (i, p) in wire.iter().enumerate() {
                distances.insert(p, i);
            }
            distances
        })
        .collect::<Vec<_>>();
    let part2 = overlaps
        .iter()
        .map(|p| wire_distances[0][p] + wire_distances[1][p])
        .min()
        .unwrap()
        // Add 2 as we aren't counting the first step for both
        + 2;

    (part1, part2)
}

enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl Move {
    fn from_str(s: &str) -> Self {
        let dist = s[1..].parse().unwrap();
        match s.chars().next().unwrap() {
            'U' => Move::Up(dist),
            'D' => Move::Down(dist),
            'L' => Move::Left(dist),
            'R' => Move::Right(dist),
            _ => unreachable!(),
        }
    }

    fn dist(&self) -> i64 {
        match self {
            Move::Up(a) | Move::Down(a) | Move::Left(a) | Move::Right(a) => *a,
        }
    }
}
