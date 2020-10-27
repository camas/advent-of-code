use crate::Exercise;

pub struct Day10;

impl Exercise for Day10 {
    fn part1(&self, input: &str) -> String {
        let digits = input
            .trim()
            .chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        let digits = do_steps(digits, 40);
        digits.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let digits = input
            .trim()
            .chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        let digits = do_steps(digits, 50);
        digits.len().to_string()
    }
}

fn do_steps(initial_digits: Vec<u8>, steps: u32) -> Vec<u8> {
    let mut digits = initial_digits;
    for _ in 0..steps {
        let mut new_digits = Vec::new();
        let mut cur_char = digits[0];
        let mut cur_length = 1;
        for &c in &digits[1..] {
            if c == cur_char {
                cur_length += 1;
            } else {
                new_digits.push(cur_length);
                new_digits.push(cur_char);
                cur_length = 1;
                cur_char = c;
            }
        }
        new_digits.push(cur_length);
        new_digits.push(cur_char);
        digits = new_digits;
    }
    digits
}
