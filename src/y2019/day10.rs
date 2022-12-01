use std::collections::HashMap;

use num::Integer;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = map[0].len();
    let height = map.len();

    let mut best = 0;
    let mut best_directions = None;
    let mut best_coords = None;
    for y in 0..height {
        for x in 0..width {
            if !map[y][x] {
                continue;
            }
            let mut directions = HashMap::<_, Vec<i64>>::new();
            #[allow(clippy::needless_range_loop)] // looks nicer ¯\_(ツ)_/¯
            for other_y in 0..height {
                for other_x in 0..width {
                    if !map[other_y][other_x] || (other_y == y && other_x == x) {
                        continue;
                    }
                    let relative_x = other_x as i64 - x as i64;
                    let relative_y = other_y as i64 - y as i64;
                    let m = relative_x.abs().gcd(&relative_y.abs());
                    directions
                        .entry((relative_x / m, relative_y / m))
                        .or_default()
                        .push(m)
                }
            }
            if directions.len() > best {
                best = directions.len();
                best_directions = Some(directions);
                best_coords = Some((x, y));
            }
        }
    }
    let part1 = best;
    let directions = best_directions.unwrap();
    let coords = best_coords.unwrap();

    // Calculate angles remembering that -y is up
    let mut directions = directions
        .into_iter()
        .map(|(dir, mut offsets)| {
            let mut angle = (dir.0 as f64).atan2(-dir.1 as f64);
            if angle < 0.0 {
                angle += 2. * std::f64::consts::PI;
            }
            // Sort offsets so that closest ones are first
            offsets.sort_unstable();
            offsets.reverse();
            (angle, offsets, dir)
        })
        .collect::<Vec<_>>();
    // Sort by angle from 0 onwards
    directions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut to_remove = 200;
    let part2;
    'outer: loop {
        for (_, offsets, dir) in directions.iter_mut() {
            if !offsets.is_empty() {
                let m = offsets.pop().unwrap();
                to_remove -= 1;
                if to_remove == 0 {
                    part2 = (coords.0 as i64 + (dir.0 * m)) * 100 + (coords.1 as i64 + (dir.1 * m));
                    break 'outer;
                }
            }
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "210");
        assert_eq!(part2.to_string(), "802");
    }
}
