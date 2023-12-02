use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let map = Map::from_str(input);
    let part1 = map.count_distinct_paths(false);
    let part2 = map.count_distinct_paths(true);
    (part1, part2)
}

struct Map<'a> {
    paths: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

impl<'a> Map<'a> {
    fn from_str(input: &'a str) -> Self {
        let mut paths: HashMap<Cave<'a>, Vec<Cave<'a>>> = HashMap::new();
        for line in input.lines() {
            let (a, b) = line.split_once('-').unwrap();
            let (a, b) = (Cave::from_str(a), Cave::from_str(b));
            paths.entry(a).or_default().push(b);
            paths.entry(b).or_default().push(a);
        }
        Map { paths }
    }

    fn count_distinct_paths(&self, part2: bool) -> u64 {
        // Set up queue starting from the starting cave
        let initial = SearchState {
            current: Cave::Start,
            visited: HashMap::new(),
        };
        let mut queue = vec![initial];
        let mut distinct_paths = 0;
        while let Some(mut curr) = queue.pop() {
            if matches!(curr.current, Cave::End) {
                distinct_paths += 1;
                continue;
            }

            // Check if we've already visited this cave
            if curr.visited.get_mut(&curr.current).is_some() {
                // If part 1 then ignore path
                if !part2 {
                    continue;
                }
                // If part two we can visit if no other cave visited more than once
                if !curr.visited.values().all(|v| *v < 2) {
                    continue;
                }
            }

            // Mark visit if small cave
            if matches!(curr.current, Cave::Small(_)) {
                *curr.visited.entry(curr.current).or_default() += 1;
            }

            // Add new states to queue for all possible moves
            for other in self.paths.get(&curr.current).unwrap() {
                if matches!(other, Cave::Start) {
                    continue;
                }
                let mut new_state = curr.clone();
                new_state.current = *other;
                queue.push(new_state);
            }
        }
        distinct_paths
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    Small(&'a str),
    Large(&'a str),
    End,
}

impl<'a> Cave<'a> {
    fn from_str(data: &'a str) -> Self {
        match data {
            "start" => Cave::Start,
            "end" => Cave::End,
            v if v.chars().all(char::is_lowercase) => Cave::Small(v),
            v => Cave::Large(v),
        }
    }
}

#[derive(Debug, Clone)]
struct SearchState<'a> {
    current: Cave<'a>,
    visited: HashMap<Cave<'a>, u8>,
}
