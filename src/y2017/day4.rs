use std::collections::HashSet;

use crate::Exercise;

pub struct Day4;

impl Exercise for Day4 {
    fn part1(&self, input: &str) -> String {
        let phrases = input
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        phrases
            .iter()
            .filter(|phrase| {
                let mut set = HashSet::new();
                for p in phrase.iter() {
                    set.insert(p);
                }
                set.len() == phrase.len()
            })
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let phrases = input
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        phrases
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
            .count()
            .to_string()
    }
}
