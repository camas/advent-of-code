use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let min_max = (0..data[0].len())
        .map(|i| {
            let mut freq = HashMap::new();
            for d in data.iter() {
                *freq.entry(d[i]).or_insert(0) += 1;
            }
            let min = *freq.iter().min_by_key(|(_, v)| **v).unwrap().0;
            let max = *freq.iter().max_by_key(|(_, v)| **v).unwrap().0;
            (min, max)
        })
        .collect::<Vec<_>>();

    let part1 = min_max.iter().map(|(_, b)| *b).collect::<String>();
    let part2 = min_max.iter().map(|(a, _)| *a).collect::<String>();

    (part1, part2)
}
