use std::collections::HashMap;

use crate::Exercise;

pub struct Day17;

impl Exercise for Day17 {
    #[allow(clippy::needless_collect)]
    fn part1(&self, input: &str) -> String {
        let sizes = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        const FILL_SIZE: u32 = 150;

        let max_mask = (0..sizes.len()).fold(0_u32, |curr, i| curr + (1 << i));
        (0..=max_mask)
            .filter(|mask| {
                let value = sizes.iter().enumerate().fold(0, |curr, (i, size)| {
                    if mask & (1 << i) > 0 {
                        curr + size
                    } else {
                        curr
                    }
                });
                value == FILL_SIZE
            })
            .count()
            .to_string()
    }

    #[allow(clippy::needless_collect)]
    fn part2(&self, input: &str) -> String {
        let sizes = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        const FILL_SIZE: u32 = 150;

        let max_mask = (0..sizes.len()).fold(0_u32, |curr, i| curr + (1 << i));
        let mut ways = HashMap::new();
        for mask in 0..=max_mask {
            let (value, used) = sizes
                .iter()
                .enumerate()
                .fold((0, 0), |(curr, used), (i, size)| {
                    if mask & (1 << i) > 0 {
                        (curr + size, used + 1)
                    } else {
                        (curr, used)
                    }
                });
            if value == FILL_SIZE {
                *ways.entry(used).or_insert(0) += 1;
            }
        }

        let key = ways.keys().min().unwrap();
        ways[key].to_string()
    }
}
