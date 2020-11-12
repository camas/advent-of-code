use crate::Exercise;

pub struct Day5;

impl Exercise for Day5 {
    fn part1(&self, input: &str) -> String {
        let mut offsets = input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut pointer = 0_i32;
        for step in 0.. {
            if pointer < 0 || pointer >= offsets.len() as i32 {
                return step.to_string();
            }
            let offset = offsets[pointer as usize];
            offsets[pointer as usize] += 1;
            pointer += offset;
        }
        unreachable!();
    }

    fn part2(&self, input: &str) -> String {
        let mut offsets = input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut pointer = 0_i32;
        for step in 0.. {
            if pointer < 0 || pointer >= offsets.len() as i32 {
                return step.to_string();
            }
            let offset = offsets[pointer as usize];
            if offset >= 3 {
                offsets[pointer as usize] -= 1;
            } else {
                offsets[pointer as usize] += 1;
            }
            pointer += offset;
        }
        unreachable!();
    }
}
