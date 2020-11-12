use std::collections::{HashMap, HashSet};

use crate::Exercise;

pub struct Day12;

impl Exercise for Day12 {
    fn part1(&self, input: &str) -> String {
        let pipes = input
            .lines()
            .map(|line| {
                let parts = line.split(" <-> ").collect::<Vec<_>>();
                let from = parts[0].parse::<u32>().unwrap();
                let to = parts[1]
                    .split(", ")
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                (from, to)
            })
            .collect::<HashMap<_, _>>();
        let mut seen = HashSet::new();
        let mut queue = vec![0];
        while !queue.is_empty() {
            let id = queue.pop().unwrap();
            if !seen.insert(id) {
                continue;
            }
            queue.extend(pipes[&id].iter());
        }
        seen.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let pipes = input
            .lines()
            .map(|line| {
                let parts = line.split(" <-> ").collect::<Vec<_>>();
                let from = parts[0].parse::<u32>().unwrap();
                let to = parts[1]
                    .split(", ")
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                (from, to)
            })
            .collect::<HashMap<_, _>>();

        let mut not_seen = pipes.keys().cloned().collect::<Vec<_>>();
        let mut groups = 0;
        while !not_seen.is_empty() {
            groups += 1;
            let initial_id = not_seen.pop().unwrap();
            let mut seen = HashSet::new();
            let mut queue = vec![initial_id];
            while !queue.is_empty() {
                let id = queue.pop().unwrap();
                if !seen.insert(id) {
                    continue;
                }
                not_seen.retain(|e| e != &id);
                queue.extend(pipes[&id].iter());
            }
        }
        groups.to_string()
    }
}
