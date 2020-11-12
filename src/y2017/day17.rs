use std::collections::VecDeque;

use crate::Exercise;

pub struct Day17;

impl Exercise for Day17 {
    fn part1(&self, input: &str) -> String {
        let step_count = input.trim().parse::<usize>().unwrap();
        let mut spinlock = VecDeque::new();
        spinlock.push_front(0);
        for i in 1..=2017 {
            let shift = (step_count + 1) % spinlock.len();
            spinlock.rotate_left(shift);
            spinlock.push_front(i);
        }
        spinlock[1].to_string()
    }

    fn part2(&self, input: &str) -> String {
        let step_count = input.trim().parse::<usize>().unwrap();
        let mut spinlock = VecDeque::new();
        spinlock.push_front(0);
        for i in 1..=50_000_000 {
            let shift = (step_count + 1) % spinlock.len();
            spinlock.rotate_left(shift);
            spinlock.push_front(i);
        }
        let pos = spinlock.iter().position(|&ele| ele == 0).unwrap();
        spinlock[pos + 1].to_string()
    }
}
