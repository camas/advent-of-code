use std::str::FromStr;

use crate::Exercise;

pub struct Day7;

impl Exercise for Day7 {
    fn part1(&self, input: &str) -> String {
        let tree = input.parse::<Tree>().unwrap();
        tree.find_root().name.clone()
    }

    fn part2(&self, input: &str) -> String {
        let tree = input.parse::<Tree>().unwrap();
        enum FindResult {
            Value(i32),
            Discrepancy(i32),
        }

        fn find_discrepancy(leaf: &Leaf, leaves: &[Leaf]) -> FindResult {
            if leaf.children.is_empty() {
                FindResult::Value(leaf.value)
            } else {
                let child_leaves = leaf
                    .children
                    .iter()
                    .map(|name| leaves.iter().find(|l| &l.name == name).unwrap())
                    .collect::<Vec<_>>();
                let child_values = child_leaves
                    .iter()
                    .map(|child| find_discrepancy(child, leaves))
                    .collect::<Vec<_>>();
                for value in child_values.iter() {
                    if let FindResult::Discrepancy(disc) = value {
                        return FindResult::Discrepancy(*disc);
                    }
                }
                let child_values = child_values
                    .into_iter()
                    .map(|val| match val {
                        FindResult::Value(value) => value,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();
                if child_values.len() > 2 {
                    if child_values[0] != child_values[1] {
                        if child_values[0] != child_values[2] {
                            return FindResult::Discrepancy(
                                child_leaves[0].value - (child_values[0] - child_values[1]),
                            );
                        } else {
                            return FindResult::Discrepancy(
                                child_leaves[1].value - (child_values[1] - child_values[0]),
                            );
                        }
                    } else if let Some(i) = child_values
                        .iter()
                        .position(|other| other != &child_values[0])
                    {
                        return FindResult::Discrepancy(
                            child_leaves[i].value - (child_values[i] - child_values[0]),
                        );
                    }
                }
                FindResult::Value(child_values.iter().sum::<i32>() + leaf.value)
            }
        }

        match find_discrepancy(tree.find_root(), &tree.leaves) {
            FindResult::Discrepancy(value) => value.to_string(),
            _ => unreachable!(),
        }
    }
}

struct Tree {
    leaves: Vec<Leaf>,
}

impl Tree {
    fn find_root(&self) -> &Leaf {
        let mut has_parents = Vec::new();
        for leaf in self.leaves.iter() {
            has_parents.extend(leaf.children.iter());
        }
        for leaf in self.leaves.iter() {
            if !has_parents.contains(&&leaf.name) {
                return leaf;
            }
        }
        unreachable!()
    }
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let leaves = s
            .lines()
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<_>>();
                let name = parts[0].to_string();
                let value = parts[1]
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .parse::<i32>()
                    .unwrap();
                let children = if parts.len() > 2 {
                    parts[3..]
                        .iter()
                        .map(|part| part.trim_end_matches(',').to_string())
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                Leaf {
                    name,
                    value,
                    children,
                }
            })
            .collect::<Vec<_>>();
        Ok(Self { leaves })
    }
}

#[derive(Debug, PartialEq)]
struct Leaf {
    name: String,
    value: i32,
    children: Vec<String>,
}
