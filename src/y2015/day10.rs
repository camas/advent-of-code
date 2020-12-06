pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let initial_digits = input
        .trim()
        .chars()
        .map(|c| c as u8 - b'0')
        .collect::<Vec<_>>();

    let digits = do_steps(&initial_digits, 40);
    let part1 = digits.len();

    let digits = do_steps(&initial_digits, 50);
    let part2 = digits.len();
    (part1, part2)
}

fn do_steps(initial_digits: &[u8], steps: u32) -> Vec<u8> {
    let mut digits = initial_digits.to_vec();
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
