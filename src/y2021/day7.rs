use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let horizontals = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut crabs = HashMap::<i64, i64>::new();
    for h in horizontals {
        *crabs.entry(h).or_default() += 1;
    }

    let mut best = i64::MAX;
    let mut best2 = i64::MAX;
    for pos in 0..=*crabs.keys().max().unwrap() {
        let value = crabs
            .iter()
            .map(|(k, v)| {
                let offset = *k - pos;
                (offset * *v).abs()
            })
            .sum::<i64>();
        if value < best {
            best = value;
        }
        let value = crabs
            .iter()
            .map(|(k, v)| {
                let n = (*k - pos).abs();
                v * (n * (n + 1)) / 2
            })
            .sum::<i64>();
        if value < best2 {
            best2 = value;
        }
    }
    let part1 = best;
    let part2 = best2;

    (part1, part2)
}
