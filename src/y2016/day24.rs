use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input as 2d tile array
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '0'..='9' => Tile::Point(c),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find all numbered locations
    let location_chars = map
        .iter()
        .flat_map(|row| {
            row.iter().filter_map(|t| match t {
                Tile::Point(c) => Some(*c),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    // Calculate the distances between each location
    let mut distances = HashMap::new();
    for &c in location_chars.iter() {
        let c_distances = breadth_first_search(c, &map);
        distances.insert(c, c_distances);
    }

    // Travelling salesman problem
    // Bruteforce all permutations starting from `0`
    let other_chars = location_chars.iter().filter(|&&c| c != '0');
    let part1 = other_chars
        .permutations(location_chars.len() - 1)
        .map(|perm| {
            let first_dist = distances[&'0'][perm[0]];
            let other_dists = perm
                .windows(2)
                .map(|chars| &distances[chars[0]][chars[1]])
                .sum::<u64>();
            first_dist + other_dists
        })
        .min()
        .unwrap();

    // Same as above but going back to start
    let other_chars = location_chars.iter().filter(|&&c| c != '0');
    let part2 = other_chars
        .permutations(location_chars.len() - 1)
        .map(|perm| {
            let first_dist = distances[&'0'][perm[0]];
            let other_dists = perm
                .windows(2)
                .map(|chars| &distances[chars[0]][chars[1]])
                .sum::<u64>();
            let last_dist = distances[&'0'][perm.last().unwrap()];
            first_dist + other_dists + last_dist
        })
        .min()
        .unwrap();

    (part1, part2)
}

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Point(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn breadth_first_search(start_char: char, map: &[Vec<Tile>]) -> HashMap<char, u64> {
    // Find the position of the start char
    let initial_position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| match tile {
                Tile::Point(c) if c == &start_char => Some(Point { x, y }),
                _ => None,
            })
        })
        .unwrap();
    // Create the starting state
    let initial_state = State {
        moves: 0,
        position: initial_position,
    };
    // Search initialisation
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_front(initial_state);
    let mut seen = HashSet::new();
    // Pull from the front of the queue, i.e the state with the lowest moves, until all states done
    while let Some(state) = queue.pop_front() {
        // Add new states for each direction
        macro_rules! insert_new {
            ($new_position:expr) => {{
                let new_state = State {
                    moves: state.moves + 1,
                    position: $new_position,
                };
                queue.push_back(new_state);
            }};
        }
        macro_rules! add_new {
            ($x:expr, $y:expr) => {
                let new_position = Point { x: $x, y: $y };
                if seen.insert(new_position.clone()) {
                    let tile = &map[new_position.y][new_position.x];
                    match tile {
                        Tile::Empty => insert_new!(new_position),
                        Tile::Point(c) if c != &start_char => {
                            distances.insert(*c, state.moves + 1);
                            insert_new!(new_position);
                        }
                        _ => (),
                    }
                }
            };
        }

        if state.position.x > 0 {
            add_new!(state.position.x - 1, state.position.y);
        }
        if state.position.x < map[0].len() - 1 {
            add_new!(state.position.x + 1, state.position.y);
        }
        if state.position.y > 0 {
            add_new!(state.position.x, state.position.y - 1);
        }
        if state.position.y < map.len() - 1 {
            add_new!(state.position.x, state.position.y + 1);
        }
    }
    distances
}

#[derive(Debug)]
struct State {
    moves: u64,
    position: Point,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let results = solve(input);
        assert_eq!(results.0.to_string(), "14");
    }
}
