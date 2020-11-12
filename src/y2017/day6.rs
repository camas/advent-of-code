use std::collections::{HashMap, HashSet};

use crate::Exercise;

pub struct Day6;

impl Exercise for Day6 {
    fn part1(&self, input: &str) -> String {
        let mut banks = input
            .trim()
            .split('\t')
            .map(|data| data.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut seen = HashSet::new();
        seen.insert(banks.clone());
        loop {
            let mut highest_index = 0;
            let mut highest_value = u32::MIN;
            for (i, val) in banks.iter().enumerate() {
                if val > &highest_value {
                    highest_index = i;
                    highest_value = *val;
                }
            }
            let mut to_dist = banks[highest_index];
            banks[highest_index] = 0;
            for index in (0..banks.len()).cycle().skip(highest_index + 1) {
                if to_dist == 0 {
                    break;
                }
                to_dist -= 1;
                banks[index] += 1;
            }
            if !seen.insert(banks.clone()) {
                return seen.len().to_string();
            }
        }
    }

    fn part2(&self, input: &str) -> String {
        let mut banks = input
            .trim()
            .split('\t')
            .map(|data| data.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut seen = HashMap::new();
        seen.insert(banks.clone(), 0);
        for step in 1.. {
            let mut highest_index = 0;
            let mut highest_value = u32::MIN;
            for (i, val) in banks.iter().enumerate() {
                if val > &highest_value {
                    highest_index = i;
                    highest_value = *val;
                }
            }
            let mut to_dist = banks[highest_index];
            banks[highest_index] = 0;
            for index in (0..banks.len()).cycle().skip(highest_index + 1) {
                if to_dist == 0 {
                    break;
                }
                to_dist -= 1;
                banks[index] += 1;
            }
            if let Some(old_step) = seen.insert(banks.clone(), step) {
                return (step - old_step).to_string();
            }
        }
        unreachable!()
    }
}
