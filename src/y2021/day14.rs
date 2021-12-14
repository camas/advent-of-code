use std::{collections::HashMap, convert::TryInto};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();
    let initial = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next().unwrap();
    let rules = lines.map(Rule::from_str).collect::<Vec<_>>();

    // Part 1 - done naively
    let mut state = initial.clone();
    for _ in 0..10 {
        let mut new_state = state
            .iter()
            .zip(state.iter().skip(1))
            .map(|(a, b)| {
                // Find matching rules
                let rule_match = rules.iter().find(|rule| rule.pattern == [*a, *b]);
                if let Some(rule) = rule_match {
                    vec![*a, rule.result]
                } else {
                    vec![*a]
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        new_state.push(*state.last().unwrap());
        state = new_state;
    }
    let mut counts = HashMap::<char, u64>::new();
    for c in state.iter() {
        *counts.entry(*c).or_default() += 1;
    }
    let part1 = counts.values().max().unwrap() - counts.values().min().unwrap();

    // Part 2 - optimized by only keeping track of unique pairs
    let mut state = HashMap::<[char; 2], u64>::new();
    for (a, b) in initial.iter().zip(initial.iter().skip(1)) {
        *state.entry([*a, *b]).or_default() += 1;
    }
    for _ in 0..40 {
        let mut next_state = HashMap::new();
        for (k, v) in state.iter() {
            let rule_match = rules.iter().find(|rule| rule.pattern == *k);
            if rule_match.is_none() {
                continue;
            }
            let rule = rule_match.unwrap();
            let pattern1 = [k[0], rule.result];
            let pattern2 = [rule.result, k[1]];
            *next_state.entry(pattern1).or_default() += v;
            *next_state.entry(pattern2).or_default() += v;
        }
        state = next_state;
    }
    // Count total by counting all the first chars in each pattern then addding the
    // last char from the initial state which will still be the last char in the final
    // state
    let mut counts = HashMap::<char, u64>::new();
    for (k, v) in state.iter() {
        *counts.entry(k[0]).or_default() += *v;
    }
    *counts.entry(*initial.last().unwrap()).or_default() += 1;
    let part2 = counts.values().max().unwrap() - counts.values().min().unwrap();

    (part1, part2)
}

struct Rule {
    pattern: [char; 2],
    result: char,
}

impl Rule {
    fn from_str(data: &str) -> Self {
        let (a, b) = data.split_once(" -> ").unwrap();
        assert_eq!(a.chars().count(), 2);
        assert_eq!(b.chars().count(), 1);
        Self {
            pattern: a.chars().collect::<Vec<_>>().try_into().unwrap(),
            result: b.chars().next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let data = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let (part1, part2) = solve(data);
        assert_eq!(part1.to_string(), "1588");
        assert_eq!(part2.to_string(), "2188189693529");
    }
}
