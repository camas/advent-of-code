use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let rooms = input.lines().map(|l| l.parse::<Room>().unwrap());

    let real = rooms.filter(|r| r.is_real()).collect::<Vec<_>>();
    let part1 = real.iter().map(|r| r.id).sum::<u64>();

    let north_pole_room = real
        .iter()
        .find(|r| r.decrypt().contains("northpole"))
        .unwrap();
    let part2 = north_pole_room.id;

    (part1, part2)
}

struct Room {
    id: u64,
    checksum: Vec<char>,
    name: Vec<Vec<char>>,
}

impl Room {
    fn is_real(&self) -> bool {
        let mut freq = HashMap::new();
        for name_part in self.name.iter() {
            for c in name_part.iter() {
                *freq.entry(*c).or_insert(0) += 1_u64;
            }
        }
        let mut expected = freq.into_iter().collect::<Vec<_>>();
        expected.sort_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other.reverse(),
        });
        expected
            .iter()
            .map(|(c, _)| c)
            .zip(self.checksum.iter())
            .all(|(a, b)| a == b)
    }

    fn decrypt(&self) -> String {
        self.name
            .iter()
            .map(|p| {
                p.iter().map(|&c| {
                    let i = c as u64 - b'a' as u64;
                    let decrypted_i = (i + self.id) % 26;
                    (decrypted_i + 'a' as u64) as u8 as char
                })
            })
            .fold(String::new(), |mut acc, curr| {
                curr.into_iter().for_each(|c| acc.push(c));
                acc.push(' ');
                acc
            })
    }
}

impl FromStr for Room {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<_>>();
        let mut last_parts = parts.last().unwrap().split('[');
        let id = last_parts.next().unwrap().parse::<u64>()?;
        let checksum = last_parts
            .next()
            .unwrap()
            .trim_end_matches(']')
            .chars()
            .collect();
        let name = parts[0..(parts.len() - 1)]
            .iter()
            .map(|p| p.chars().collect())
            .collect();
        Ok(Self { id, name, checksum })
    }
}
