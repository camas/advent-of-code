use crate::Exercise;

pub struct Day1;

impl Exercise for Day1 {
    fn part1(&self, input: &str) -> String {
        let digits = input
            .trim()
            .chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        let mut sum = digits
            .iter()
            .zip(&digits[1..])
            .fold(
                0_u64,
                |curr, (&a, &b)| {
                    if a == b {
                        curr + a as u64
                    } else {
                        curr
                    }
                },
            );
        if &digits[0] == digits.last().unwrap() {
            sum += digits[0] as u64;
        }
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let digits = input
            .trim()
            .chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        let half_shift = [&digits[digits.len() / 2..], &digits[..digits.len() / 2]].concat();
        let sum =
            digits.iter().zip(&half_shift).fold(
                0_u64,
                |curr, (&a, &b)| {
                    if a == b {
                        curr + a as u64
                    } else {
                        curr
                    }
                },
            );
        sum.to_string()
    }
}
