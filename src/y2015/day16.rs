use std::collections::HashMap;

use crate::Exercise;

pub struct Day16;

impl Exercise for Day16 {
    fn part1(&self, input: &str) -> String {
        let sues = input
            .lines()
            .map(|line| Sue::from_str(line))
            .collect::<Vec<_>>();
        let possibles = sues
            .iter()
            .filter(|sue| {
                for (key, &value) in sue.items.iter() {
                    let is_ok = match *key {
                        "children" => value == 3,
                        "cats" => value == 7,
                        "samoyeds" => value == 2,
                        "pomeranians" => value == 3,
                        "akitas" => value == 0,
                        "vizslas" => value == 0,
                        "goldfish" => value == 5,
                        "trees" => value == 3,
                        "cars" => value == 2,
                        "perfumes" => value == 1,
                        _ => unreachable!(),
                    };
                    if !is_ok {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();
        assert_eq!(possibles.len(), 1);
        possibles[0].index.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let sues = input
            .lines()
            .map(|line| Sue::from_str(line))
            .collect::<Vec<_>>();
        let possibles = sues
            .iter()
            .filter(|sue| {
                for (key, &value) in sue.items.iter() {
                    let is_ok = match *key {
                        "children" => value == 3,
                        "cats" => value > 7,
                        "samoyeds" => value == 2,
                        "pomeranians" => value < 3,
                        "akitas" => value == 0,
                        "vizslas" => value == 0,
                        "goldfish" => value < 5,
                        "trees" => value > 3,
                        "cars" => value == 2,
                        "perfumes" => value == 1,
                        _ => unreachable!(),
                    };
                    if !is_ok {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();
        assert_eq!(possibles.len(), 1);
        possibles[0].index.to_string()
    }
}

struct Sue<'a> {
    index: u32,
    items: HashMap<&'a str, u32>,
}

impl<'a> Sue<'a> {
    fn from_str(s: &'a str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        let index = parts[1].trim_end_matches(':').parse().unwrap();
        let mut items = HashMap::new();
        for i in (2..parts.len()).step_by(2) {
            let name = parts[i].trim_end_matches(':');
            let value = parts[i + 1].trim_end_matches(',').parse().unwrap();
            items.insert(name, value);
        }
        Self { index, items }
    }
}
