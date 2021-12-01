use std::{collections::HashSet, hash::Hash};

const TOP_FLOOR: usize = 3;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input
    let floors = input
        .lines()
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<_>>();
            if parts[4] == "nothing" {
                Vec::new()
            } else {
                let mut parts = parts[4..].iter();
                let mut items = Vec::new();
                while let Some(part) = parts.next() {
                    if *part == "and" {
                        parts.next().unwrap();
                    }
                    let name = parts.next().unwrap();
                    let item = match parts
                        .next()
                        .unwrap()
                        .trim_end_matches(|c| c == '.' || c == ',')
                    {
                        "generator" => RawObject::Generator(name.to_string()),
                        "microchip" => {
                            RawObject::Microchip(name.trim_end_matches("-compatible").to_string())
                        }
                        _ => unreachable!(),
                    };
                    items.push(item);
                }
                items
            }
        })
        .collect::<Vec<_>>();

    // Convert to index pairs
    let chip_indexes = floors.iter().enumerate().flat_map(|(i, floor)| {
        floor.iter().filter_map(move |obj| match obj {
            RawObject::Microchip(name) => Some((i, name)),
            _ => None,
        })
    });
    let mut positions = chip_indexes
        .map(|(i, name)| {
            let other_index = floors
                .iter()
                .enumerate()
                .find(|(_, floor)| {
                    floor.iter().any(|o| match o {
                        RawObject::Generator(gen_name) => gen_name == name,
                        _ => false,
                    })
                })
                .unwrap()
                .0;
            Pair {
                microchip: i,
                generator: other_index,
            }
        })
        .collect::<Vec<_>>();

    // A* algorithm with h being total distance of all objects from floor 4
    let part1 = moves_needed(&positions);
    positions.push(Pair {
        microchip: 0,
        generator: 0,
    });
    positions.push(Pair {
        microchip: 0,
        generator: 0,
    });
    let part2 = moves_needed(&positions);

    (part1, part2)
}

fn moves_needed(positions: &[Pair]) -> usize {
    let initial_state = State::new(0, 0, positions.to_vec());
    let mut queue = vec![initial_state];
    let mut seen = HashSet::new();
    loop {
        // Get state with smallest moves + heuristic
        let state = queue.pop().unwrap();

        // Check for end state
        if state.is_complete() {
            return state.moves;
        }

        // Add all possible moves from current state to queue
        macro_rules! insert_new {
            ($state:expr) => {
                let new = $state;
                // Ignore if state invalid or already seen
                if new.is_valid() && seen.insert((new.elevator, new.positions.clone())) {
                    let pos = queue
                        .binary_search_by(|s| {
                            (s.moves + s.heuristic)
                                .cmp(&(new.moves + new.heuristic))
                                .reverse()
                        })
                        .unwrap_or_else(|e| e);
                    queue.insert(pos, new);
                }
            };
        }
        // Check if elevator above lowest occupied floor
        let lowest_floor = state
            .positions
            .iter()
            .map(|p| p.microchip.min(p.generator))
            .min()
            .unwrap();
        let should_go_down = state.elevator > lowest_floor;
        // Single items
        for i in 0..(state.positions.len() * 2) {
            let value = match i % 2 {
                0 => state.positions[i / 2].microchip,
                1 => state.positions[i / 2].generator,
                _ => unreachable!(),
            };
            if value != state.elevator {
                continue;
            }
            // Up
            if state.elevator < TOP_FLOOR {
                let mut new_positions = state.positions.clone();
                match i % 2 {
                    0 => new_positions[i / 2].microchip += 1,
                    1 => new_positions[i / 2].generator += 1,
                    _ => unreachable!(),
                }
                new_positions.sort_unstable();
                insert_new!(State::new(
                    state.moves + 1,
                    state.elevator + 1,
                    new_positions
                ));
            }
            // Down
            if should_go_down && state.elevator > 0 {
                let mut new_positions = state.positions.clone();
                match i % 2 {
                    0 => new_positions[i / 2].microchip -= 1,
                    1 => new_positions[i / 2].generator -= 1,
                    _ => unreachable!(),
                }
                new_positions.sort_unstable();
                insert_new!(State::new(
                    state.moves + 1,
                    state.elevator - 1,
                    new_positions
                ));
            }
        }
        // Two items
        for (i, j) in (0..(state.positions.len() * 2))
            .flat_map(|i| ((i + 1)..(state.positions.len() * 2)).map(move |j| (i, j)))
            .filter(|(i, j)| {
                let i_value = match i % 2 {
                    0 => state.positions[i / 2].microchip,
                    1 => state.positions[i / 2].generator,
                    _ => unreachable!(),
                };
                let j_value = match j % 2 {
                    0 => state.positions[j / 2].microchip,
                    1 => state.positions[j / 2].generator,
                    _ => unreachable!(),
                };
                i_value == state.elevator && j_value == state.elevator
            })
        {
            // Up
            if state.elevator < TOP_FLOOR {
                let mut new_positions = state.positions.clone();
                match i % 2 {
                    0 => new_positions[i / 2].microchip += 1,
                    1 => new_positions[i / 2].generator += 1,
                    _ => unreachable!(),
                }
                match j % 2 {
                    0 => new_positions[j / 2].microchip += 1,
                    1 => new_positions[j / 2].generator += 1,
                    _ => unreachable!(),
                }
                new_positions.sort_unstable();
                insert_new!(State::new(
                    state.moves + 1,
                    state.elevator + 1,
                    new_positions
                ));
            }
            // Down
            if should_go_down && state.elevator > 0 {
                let mut new_positions = state.positions.clone();
                match i % 2 {
                    0 => new_positions[i / 2].microchip -= 1,
                    1 => new_positions[i / 2].generator -= 1,
                    _ => unreachable!(),
                }
                match j % 2 {
                    0 => new_positions[j / 2].microchip -= 1,
                    1 => new_positions[j / 2].generator -= 1,
                    _ => unreachable!(),
                }
                new_positions.sort_unstable();
                insert_new!(State::new(
                    state.moves + 1,
                    state.elevator - 1,
                    new_positions
                ));
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    microchip: usize,
    generator: usize,
}

#[derive(Debug, Clone, Hash)]
struct State {
    moves: usize,
    elevator: usize,
    positions: Vec<Pair>,
    heuristic: usize,
}

impl State {
    fn new(moves: usize, elevator: usize, positions: Vec<Pair>) -> Self {
        let heuristic = positions
            .iter()
            .map(|pair| TOP_FLOOR - pair.microchip + TOP_FLOOR - pair.generator)
            .sum::<usize>();
        Self {
            moves,
            elevator,
            positions,
            heuristic,
        }
    }

    fn is_complete(&self) -> bool {
        self.positions
            .iter()
            .all(|p| p.microchip == TOP_FLOOR && p.generator == TOP_FLOOR)
    }

    fn is_valid(&self) -> bool {
        self.positions.iter().all(|pair| {
            pair.microchip == pair.generator
                || self
                    .positions
                    .iter()
                    .all(|other| other.generator != pair.microchip)
        })
    }
}

#[derive(Debug)]
enum RawObject {
    Generator(String),
    Microchip(String),
}
