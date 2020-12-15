use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut spoken = numbers.clone();
    while spoken.len() < 2020 {
        let last = spoken.last().unwrap();
        let prev = spoken
            .iter()
            .rev()
            .skip(1)
            .enumerate()
            .find(|(_, e)| *e == last);
        let next = match prev {
            Some((rev_offset, _)) => rev_offset + 1,
            None => 0,
        };
        spoken.push(next as u64);
    }
    let part1 = spoken[2020 - 1];

    // Instead of storing all seen numbers, only keep the last time a number was seen
    let mut last_seen = HashMap::new();
    for (i, n) in numbers[..(numbers.len() - 1)].iter().enumerate() {
        last_seen.insert(*n, i as u64);
    }
    let mut current = *numbers.last().unwrap();
    for time in (numbers.len() as u64 - 1)..(30000000 - 1) {
        let new = match last_seen.get(&current) {
            Some(value) => time - value,
            None => 0,
        };
        last_seen.insert(current, time);
        current = new;
    }
    let part2 = current;

    (part1, part2)
}
