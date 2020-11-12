use crate::Exercise;

pub struct Day15;

impl Exercise for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let initial_a = lines.next().unwrap()[24..].parse::<u64>().unwrap();
        let initial_b = lines.next().unwrap()[24..].parse::<u64>().unwrap();
        let mut matches = 0;
        let mut a = initial_a;
        let mut b = initial_b;
        for _step in 0..40_000_000 {
            a = (a * 16_807) % 2_147_483_647;
            b = (b * 48_271) % 2_147_483_647;
            if (a & 0xffff) == (b & 0xffff) {
                matches += 1;
            }
        }
        matches.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let initial_a = lines.next().unwrap()[24..].parse::<u64>().unwrap();
        let initial_b = lines.next().unwrap()[24..].parse::<u64>().unwrap();
        let mut matches = 0;
        let mut a = initial_a;
        let mut b = initial_b;
        for _step in 0..5_000_000 {
            loop {
                a = (a * 16_807) % 2_147_483_647;
                if a % 4 == 0 {
                    break;
                }
            }
            loop {
                b = (b * 48_271) % 2_147_483_647;
                if b % 8 == 0 {
                    break;
                }
            }
            if (a & 0xffff) == (b & 0xffff) {
                matches += 1;
            }
        }
        matches.to_string()
    }
}
