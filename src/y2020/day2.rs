use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = input
        .lines()
        .map(|line| line.parse::<PasswordEntry>().unwrap())
        .collect::<Vec<_>>();

    let part1 = input.iter().filter(|entry| entry.is_valid()).count();

    let part2 = input.iter().filter(|entry| entry.is_valid_2()).count();

    (part1, part2)
}

struct PasswordEntry {
    min_count: u64,
    max_count: u64,
    req_char: char,
    password: String,
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.req_char {
                count += 1;
            }
        }
        count >= self.min_count && count <= self.max_count
    }

    fn is_valid_2(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        let first = chars.get(self.min_count as usize - 1) == Some(&self.req_char);
        let second = chars.get(self.max_count as usize - 1) == Some(&self.req_char);
        (first && !second) || (!first && second)
    }
}

impl FromStr for PasswordEntry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let range = parts[0].split('-').collect::<Vec<_>>();
        Ok(PasswordEntry {
            min_count: range[0].parse()?,
            max_count: range[1].parse()?,
            req_char: parts[1].chars().next().unwrap(),
            password: parts[2].to_string(),
        })
    }
}
