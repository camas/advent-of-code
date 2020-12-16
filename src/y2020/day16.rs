use std::{collections::HashMap, num::ParseIntError, ops::RangeInclusive, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input
    let mut lines = input.lines();
    let rules = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| l.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let own_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    lines.next().unwrap();
    lines.next().unwrap();
    let tickets = lines
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ticket_len = tickets[0].len();

    // Part 1: Find invalid tickets
    let part1 = tickets
        .iter()
        .map(|values| {
            values
                .iter()
                .filter(|v| !rules.iter().any(|r| r.contains(v)))
                .sum::<u64>()
        })
        .sum::<u64>();

    // Part 2: Match rules to columns
    // Filter invalid tickets
    let valid_tickets = tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|v| rules.iter().any(|r| r.contains(v))))
        .collect::<Vec<_>>();
    // Find all possible valid columns for each rule
    let rule_matches = rules
        .iter()
        .map(|rule| {
            (0..ticket_len)
                .filter(|i| {
                    valid_tickets
                        .iter()
                        .all(|ticket| rule.contains(&ticket[*i]))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // Match rules to columns
    // There will be one rule that has only one possible column each step
    let mut rule_indexes = HashMap::new();
    let mut queue = rule_matches;
    for _ in 0..rules.len() {
        let (rule_index, potential) = queue
            .iter()
            .enumerate()
            .find(|(_, potential)| potential.len() == 1)
            .unwrap();
        let column = potential[0];
        rule_indexes.insert(&rules[rule_index], column);
        queue.iter_mut().for_each(|p| p.retain(|v| v != &column));
    }
    // Calculate product of all `departure` entries on own ticket
    let part2 = rule_indexes
        .iter()
        .filter(|(r, _)| r.name.starts_with("departure"))
        .map(|(_, i)| own_ticket[*i])
        .product::<u64>();

    (part1, part2)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    range_a: RangeInclusive<u64>,
    range_b: RangeInclusive<u64>,
}

impl Rule {
    /// Test if `value` is contained in either of this rule's ranges
    fn contains(&self, value: &u64) -> bool {
        self.range_a.contains(value) || self.range_b.contains(value)
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, ": ");
        let name = split.next().unwrap().to_string();
        let mut ranges = split.next().unwrap().split(" or ").map(|s| {
            let mut nums = s.split('-').map(|s| s.parse::<u64>().unwrap());
            nums.next().unwrap()..=nums.next().unwrap()
        });
        Ok(Self {
            name,
            range_a: ranges.next().unwrap(),
            range_b: ranges.next().unwrap(),
        })
    }
}
