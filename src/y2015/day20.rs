use crate::Exercise;

pub struct Day20;

impl Exercise for Day20 {
    fn part1(&self, input: &str) -> String {
        let target = input.trim().parse::<usize>().unwrap() / 10;
        let mut sieve = [0; 1_000_000];
        for factor in 1..sieve.len() {
            for product in (factor..sieve.len()).step_by(factor) {
                sieve[product] += factor;
            }
            if sieve[factor] >= target {
                return factor.to_string();
            }
        }
        unreachable!();
    }

    fn part2(&self, input: &str) -> String {
        let target = input.trim().parse::<usize>().unwrap() / 11;
        let mut sieve = [0; 1_000_000];
        for factor in 1..sieve.len() {
            for product in (factor..sieve.len()).step_by(factor).take(50) {
                sieve[product] += factor;
            }
            if sieve[factor] >= target {
                return factor.to_string();
            }
        }
        unreachable!();
    }
}
