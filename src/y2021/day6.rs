use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let ages = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut fish = HashMap::<u64, u64>::new();
    for age in ages {
        *fish.entry(age).or_default() += 1;
    }

    let mut part1 = None;
    for day in 0..256 {
        let mut new_fish = HashMap::<u64, u64>::new();
        for (age, count) in fish {
            if age == 0 {
                *new_fish.entry(6).or_default() += count;
                *new_fish.entry(8).or_default() += count;
            } else {
                *new_fish.entry(age - 1).or_default() += count;
            }
        }
        fish = new_fish;
        if day == 79 {
            part1 = Some(fish.values().sum::<u64>());
        }
    }
    let part2 = fish.values().sum::<u64>();

    (part1.unwrap(), part2)
}
