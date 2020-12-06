use std::{collections::HashSet, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let groups = input
        .split("\n\n")
        .map(|s| s.parse::<Group>().unwrap())
        .collect::<Vec<_>>();

    let part1 = groups
        .iter()
        .map(|group| group.any_yes().len())
        .sum::<usize>();

    let part2 = groups
        .iter()
        .map(|group| group.all_yes().len())
        .sum::<usize>();

    (part1, part2)
}

struct Group {
    answers: Vec<HashSet<char>>,
}

impl Group {
    fn any_yes(&self) -> HashSet<char> {
        let mut result = HashSet::new();
        for a in self.answers.iter() {
            for b in a {
                result.insert(*b);
            }
        }
        result
    }

    fn all_yes(&self) -> Vec<char> {
        ('a'..='z')
            .filter(|c| self.answers.iter().all(|a| a.contains(c)))
            .collect()
    }
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let answers = s
            .lines()
            .map(|line| line.chars().collect::<HashSet<_>>())
            .collect();
        Ok(Self { answers })
    }
}
