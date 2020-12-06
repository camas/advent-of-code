use std::collections::HashSet;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let phrases = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let part1 = phrases
        .iter()
        .filter(|phrase| {
            let mut set = HashSet::new();
            for p in phrase.iter() {
                set.insert(p);
            }
            set.len() == phrase.len()
        })
        .count();

    let part2 = phrases
        .iter()
        .filter(|phrase| {
            let mut set = HashSet::new();
            for p in phrase.iter() {
                let mut chars = p.chars().collect::<Vec<_>>();
                chars.sort_unstable();
                set.insert(chars);
            }
            set.len() == phrase.len()
        })
        .count();

    (part1, part2)
}
