use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::Exercise;

pub struct Day9;

impl Exercise for Day9 {
    fn part1(&self, input: &str) -> String {
        // Parse
        let mut locations = HashSet::new();
        let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();
        for line in input.lines() {
            let parts = line.split(' ').collect::<Vec<_>>();
            let from = parts[0];
            let to = parts[2];
            let dist = parts[4].parse::<u32>().unwrap();

            locations.insert(from.to_string());
            locations.insert(to.to_string());

            distances
                .entry(from.to_string())
                .or_default()
                .insert(to.to_string(), dist);
            distances
                .entry(to.to_string())
                .or_default()
                .insert(from.to_string(), dist);
        }

        // Brute force
        locations
            .par_iter()
            .map(|start| {
                let initial_state = SearchState {
                    visited: vec![&start],
                    total: 0,
                };
                let mut states = vec![initial_state];
                let mut best = u32::MAX;
                while !states.is_empty() {
                    let current_state = states.pop().unwrap();
                    if current_state.visited.len() == locations.len() {
                        // Check result
                        if current_state.total < best {
                            best = current_state.total;
                        }
                        continue;
                    }

                    let can_visit = distances[current_state.last_location()]
                        .iter()
                        .filter(|(other, _)| !current_state.visited.contains(&other.as_str()))
                        .collect::<Vec<_>>();
                    for (other, dist) in can_visit {
                        let mut visited = current_state.visited.clone();
                        visited.push(other);
                        let new_state = SearchState {
                            visited,
                            total: current_state.total + dist,
                        };
                        states.push(new_state);
                    }
                }
                best
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Parse
        let mut locations = HashSet::new();
        let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();
        for line in input.lines() {
            let parts = line.split(' ').collect::<Vec<_>>();
            let from = parts[0];
            let to = parts[2];
            let dist = parts[4].parse::<u32>().unwrap();

            locations.insert(from.to_string());
            locations.insert(to.to_string());

            distances
                .entry(from.to_string())
                .or_default()
                .insert(to.to_string(), dist);
            distances
                .entry(to.to_string())
                .or_default()
                .insert(from.to_string(), dist);
        }

        // Brute force
        locations
            .par_iter()
            .map(|start| {
                let initial_state = SearchState {
                    visited: vec![&start],
                    total: 0,
                };
                let mut states = vec![initial_state];
                let mut best = u32::MIN;
                while !states.is_empty() {
                    let current_state = states.pop().unwrap();
                    if current_state.visited.len() == locations.len() {
                        // Check result
                        if current_state.total > best {
                            best = current_state.total;
                        }
                        continue;
                    }

                    let can_visit = distances[current_state.last_location()]
                        .iter()
                        .filter(|(other, _)| !current_state.visited.contains(&other.as_str()))
                        .collect::<Vec<_>>();
                    for (other, dist) in can_visit {
                        let mut visited = current_state.visited.clone();
                        visited.push(other);
                        let new_state = SearchState {
                            visited,
                            total: current_state.total + dist,
                        };
                        states.push(new_state);
                    }
                }
                best
            })
            .max()
            .unwrap()
            .to_string()
    }
}

struct SearchState<'a> {
    visited: Vec<&'a str>,
    total: u32,
}

impl<'a> SearchState<'a> {
    fn last_location(&self) -> &'a str {
        self.visited.last().unwrap()
    }
}
