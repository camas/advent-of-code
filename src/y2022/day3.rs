use std::{collections::HashSet, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let rucksacks = input
        .trim()
        .lines()
        .map(|l| Rucksack::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let part1 = rucksacks
        .iter()
        .map(|r| {
            r.compartment1
                .intersection(&r.compartment2)
                .map(|i| i.score() as u64)
                .sum::<u64>()
        })
        .sum::<u64>();

    let part2 = rucksacks
        .chunks(3)
        // lol
        .map(|chunk| {
            let mut unions = chunk.iter().map(|r| {
                r.compartment1
                    .union(&r.compartment2)
                    .copied()
                    .collect::<HashSet<Item>>()
            });
            unions
                .next()
                .map(|set| {
                    unions
                        .fold(set, |a, b| {
                            a.intersection(&b).copied().collect::<HashSet<Item>>()
                        })
                        .iter()
                        .map(|i| i.score() as u64)
                        .sum::<u64>()
                })
                .unwrap()
        })
        .sum::<u64>();

    (part1, part2)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Item(char);

impl Item {
    fn score(self) -> u32 {
        match self.0 {
            'a'..='z' => self.0 as u32 - 'a' as u32 + 1,
            'A'..='Z' => self.0 as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: HashSet<Item>,
    compartment2: HashSet<Item>,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut compartment1 = s.chars().map(Item).collect::<Vec<_>>();
        let compartment2 = compartment1.split_off(compartment1.len() / 2);

        Ok(Rucksack {
            compartment1: HashSet::from_iter(compartment1.into_iter()),
            compartment2: HashSet::from_iter(compartment2.into_iter()),
        })
    }
}
