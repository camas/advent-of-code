use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let rules = input.lines().map(|line| line.into()).collect::<Vec<Rule>>();
    let rule_dict = rules
        .iter()
        .map(|rule| (&rule.bag, rule))
        .collect::<HashMap<_, _>>();

    // Part 1: Find bags that contain a shiny gold bag, any layer deep
    let shiny_gold = Bag {
        description: "shiny",
        color: "gold",
    };
    let mut queue = vec![&shiny_gold];
    let mut found: HashSet<&Bag> = HashSet::new();
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        let contained_in = rules
            .iter()
            .filter(|rule| rule.contains.iter().any(|(inner, _)| inner == current))
            .map(|rule| &rule.bag)
            .collect::<Vec<_>>();
        found.extend(&contained_in);
        queue.extend(&contained_in);
    }
    let part1 = found.len();

    // Part 2: Count bags inside a single shiny gold bag
    // Find all children
    let mut queue = vec![&shiny_gold];
    let mut found = HashSet::new();
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        found.insert(current);
        let current_rule = rule_dict.get(current).unwrap();
        queue.extend(current_rule.contains.iter().map(|(bag, _)| bag));
    }
    // Find child count for each bag
    let mut count_dict = HashMap::new();
    while !found.is_empty() {
        // Find a bag with all dependencies already counted
        let current = *found
            .iter()
            .find(|b| {
                let b_rule = rule_dict.get(*b).unwrap();
                b_rule
                    .contains
                    .iter()
                    .all(|(inner, _)| count_dict.contains_key(&inner))
            })
            .unwrap();
        found.remove(current);
        let rule = rule_dict.get(current).unwrap();
        let count: u64 = rule
            .contains
            .iter()
            .map(|(bag, count)| (count_dict.get(&bag).unwrap() + 1) * count)
            .sum::<u64>();
        count_dict.insert(current, count);
    }
    let part2 = *count_dict.get(&shiny_gold).unwrap();

    (part1, part2)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Rule<'a> {
    bag: Bag<'a>,
    contains: Vec<(Bag<'a>, u64)>,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let mut initial_split = s.split(" contain ");
        let bag = initial_split
            .next()
            .unwrap()
            .trim_end_matches(" bags")
            .into();
        let end = initial_split.next().unwrap();
        let contains = if end == "no other bags." {
            Vec::new()
        } else {
            end.split(',')
                .map(|s| {
                    let mut split = s.trim_start().splitn(2, ' ');
                    let num = split.next().unwrap().parse::<u64>().unwrap();
                    let bag = split
                        .next()
                        .unwrap()
                        .trim_end_matches('.')
                        .trim_end_matches(" bags")
                        .trim_end_matches("bag")
                        .into();
                    (bag, num)
                })
                .collect()
        };
        Self { bag, contains }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Bag<'a> {
    description: &'a str,
    color: &'a str,
}

impl<'a> From<&'a str> for Bag<'a> {
    fn from(s: &'a str) -> Self {
        let mut split = s.split(' ');
        Self {
            description: split.next().unwrap(),
            color: split.next().unwrap(),
        }
    }
}
