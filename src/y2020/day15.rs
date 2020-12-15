use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = van_eck(&numbers, 2020);
    let part2 = van_eck(&numbers, 30000000);

    (part1, part2)
}

/// Returns the nth (one-indexed) number of a van eck sequence with the given initial numbers
fn van_eck(initial: &[u64], target: u64) -> u64 {
    let mut seen = initial[..initial.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i as u64 + 1))
        .collect::<HashMap<_, _>>();
    ((initial.len() as u64)..target).fold(*initial.last().unwrap(), |last, time| {
        seen.insert(last, time)
            .map(|last_seen| time - last_seen)
            .unwrap_or(0)
    })
}
