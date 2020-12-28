// Recommended reading https://www.redblobgames.com/grids/hexagons/

use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let routes = input
        .lines()
        .map(|l| {
            let mut dirs = Vec::new();
            let mut chars = l.chars();
            while let Some(c) = chars.next() {
                dirs.push(match c {
                    'e' => Direction::East,
                    'w' => Direction::West,
                    's' => match chars.next() {
                        Some('e') => Direction::SouthEast,
                        Some('w') => Direction::SouthWest,
                        _ => unreachable!(),
                    },
                    'n' => match chars.next() {
                        Some('e') => Direction::NorthEast,
                        Some('w') => Direction::NorthWest,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                });
            }
            dirs
        })
        .collect::<Vec<_>>();

    let mut flipped = HashSet::new();
    for route in &routes {
        // Axial coordinates
        let mut q = route
            .iter()
            .filter(|d| matches!(d, Direction::East))
            .count() as i64
            - route
                .iter()
                .filter(|d| matches!(d, Direction::West))
                .count() as i64;
        let mut r = route
            .iter()
            .filter(|d| matches!(d, Direction::SouthEast))
            .count() as i64
            - route
                .iter()
                .filter(|d| matches!(d, Direction::NorthWest))
                .count() as i64;
        let tmp = route
            .iter()
            .filter(|d| matches!(d, Direction::NorthEast))
            .count() as i64
            - route
                .iter()
                .filter(|d| matches!(d, Direction::SouthWest))
                .count() as i64;
        q += tmp;
        r -= tmp;
        if !flipped.insert((q, r)) {
            flipped.remove(&(q, r));
        }
    }
    let part1 = flipped.len();

    // If a tile is in state it is black
    let mut state = flipped;
    for _ in 0..100 {
        let mut count = HashMap::new();
        state.iter().for_each(|(q, r)| {
            count.entry((*q, *r)).or_default();
            const OFFSETS: [(i64, i64); 6] = [(1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1)];
            for (offset_q, offset_r) in OFFSETS.iter() {
                let q = q + offset_q;
                let r = r + offset_r;
                *count.entry((q, r)).or_insert(0) += 1;
            }
        });
        state = count
            .into_iter()
            .filter_map(|((q, r), count)| {
                let was_black = state.contains(&(q, r));
                if (was_black && !(count == 0 || count > 2)) || (!was_black && count == 2) {
                    Some((q, r))
                } else {
                    None
                }
            })
            .collect();
    }
    let part2 = state.len();

    (part1, part2)
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthEast,
    NorthWest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let result = solve(
            "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
        );
        assert_eq!(result.0.to_string(), "10");
        assert_eq!(result.1.to_string(), "2208");
    }
}
