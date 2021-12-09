use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input
    let mut lines = input.lines();
    let mut rules = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(": ");
            let id = parts.next().unwrap().parse().unwrap();
            let children_strs = parts.next().unwrap().split(" | ").collect::<Vec<_>>();
            let children = if children_strs.len() == 1 && children_strs[0].starts_with('"') {
                ChildType::Char(children_strs[0].chars().nth(1).unwrap())
            } else {
                ChildType::Rules(
                    children_strs
                        .iter()
                        .map(|s| s.split(' ').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
                        .collect::<Vec<_>>(),
                )
            };
            let rule = Rule { id, children };
            (id, rule)
        })
        .collect::<HashMap<_, _>>();
    let messages = lines.collect::<Vec<_>>();

    // Count all matches
    let part1 = messages
        .iter()
        .filter(|m| rules[&0].matches(m, &rules))
        .count();

    // Return early if rule 8 doesn't exist
    if !rules.contains_key(&8) {
        return (part1, 0);
    }

    // Change to recursive rules and count all matches
    rules.get_mut(&8).unwrap().children = ChildType::Rules(vec![vec![42], vec![42, 8]]);
    rules.get_mut(&11).unwrap().children = ChildType::Rules(vec![vec![42, 31], vec![42, 11, 31]]);
    let part2 = messages
        .iter()
        .filter(|m| rules[&0].matches(m, &rules))
        .count();

    (part1, part2)
}

#[derive(Debug)]
struct Rule {
    #[allow(dead_code)]
    id: usize,
    children: ChildType,
}

#[derive(Debug)]
enum ChildType {
    Char(char),
    Rules(Vec<Vec<usize>>),
}

impl Rule {
    fn matches(&self, message: &str, rules: &HashMap<usize, Rule>) -> bool {
        self.inner_matches(&message.chars().collect::<Vec<_>>(), 0, rules)
            .iter()
            .any(|m| *m == message.len())
    }

    fn inner_matches(
        &self,
        message: &[char],
        index: usize,
        rules: &HashMap<usize, Rule>,
    ) -> Vec<usize> {
        match &self.children {
            ChildType::Char(c) => {
                if index < message.len() && message[index] == *c {
                    vec![index + 1]
                } else {
                    Vec::new()
                }
            }
            ChildType::Rules(options) => options
                .iter()
                .flat_map(|rule_set| {
                    rule_set.iter().fold(vec![index], |curr, rule| {
                        curr.iter()
                            .flat_map(|i| rules[rule].inner_matches(message, *i, rules))
                            .collect()
                    })
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let results = solve(input);
        assert_eq!(results.0.to_string(), "2");
    }
}
