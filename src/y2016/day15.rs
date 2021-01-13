use std::{num::ParseIntError, str::FromStr};

use crate::common::chinese_remainder;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut disks = input
        .lines()
        .map(|l| l.parse::<Disk>().unwrap())
        .collect::<Vec<_>>();

    let part1 = work(&disks);
    disks.push(Disk {
        size: 11,
        offset: 0,
    });
    let part2 = work(&disks);

    (part1, part2)
}

fn work(disks: &[Disk]) -> i64 {
    let offsets = disks
        .iter()
        .enumerate()
        .map(|(i, disk)| (-disk.offset - (i as i64 + 1)).rem_euclid(disk.size))
        .collect::<Vec<_>>();
    let m = disks.iter().map(|d| d.size).collect::<Vec<_>>();
    chinese_remainder(&offsets, &m).unwrap()
}

#[derive(Debug)]
struct Disk {
    size: i64,
    offset: i64,
}

impl FromStr for Disk {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let size = parts[3].parse()?;
        let offset = parts[11].trim_end_matches('.').parse()?;
        Ok(Self { size, offset })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
        let res = solve(input);
        assert_eq!(res.0.to_string(), "5");
    }
}
