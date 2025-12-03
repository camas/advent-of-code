use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use winnow::{combinator::separated, Parser};

use crate::common::parse_u32;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let mut before_map = HashMap::<u32, HashSet<u32>>::new();
    for rule in &input.rules {
        before_map.entry(rule.x).or_default().insert(rule.y);
    }

    let mut rule_map = HashMap::new();
    for rule in &input.rules {
        rule_map.insert((rule.x, rule.y), Ordering::Less);
        rule_map.insert((rule.y, rule.x), Ordering::Greater);
    }

    let is_correctly_ordered = |pages: &[u32]| {
        let mut seen = HashSet::new();
        for value in pages {
            seen.insert(*value);
            let Some(before) = before_map.get(value) else {
                continue;
            };
            if !seen.is_disjoint(before) {
                return false;
            }
        }

        true
    };

    let mut part1 = 0;
    let mut part2 = 0;
    for mut update in input.updates {
        if is_correctly_ordered(&update.pages) {
            part1 += update.pages[update.pages.len() / 2];
        } else {
            update
                .pages
                .sort_by(|a, b| *rule_map.get(&(*a, *b)).unwrap_or(&Ordering::Equal));
            part2 += update.pages[update.pages.len() / 2];
        }
    }

    (part1, part2)
}

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Input {
    fn parse(input: &str) -> Input {
        (
            separated(1.., Rule::parse, '\n'),
            "\n\n",
            separated(1.., Update::parse, '\n'),
        )
            .map(|(rules, _, updates)| Input { rules, updates })
            .parse(input.trim())
            .unwrap()
    }
}

struct Rule {
    x: u32,
    y: u32,
}

impl Rule {
    fn parse(input: &mut &str) -> winnow::Result<Self> {
        (parse_u32, '|', parse_u32)
            .map(|(x, _, y)| Rule { x, y })
            .parse_next(input)
    }
}

struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn parse(input: &mut &str) -> winnow::Result<Self> {
        (separated(1.., parse_u32, ','))
            .map(|pages| Update { pages })
            .parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "143");
        assert_eq!(part2.to_string(), "123");
    }
}
