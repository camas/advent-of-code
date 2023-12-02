pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let part1 = input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();

            (first_digit * 10 + last_digit) as u64
        })
        .sum::<u64>();

    let part2 = input
        .lines()
        .map(|line| {
            let first_digit = (0..line.len())
                .find_map(|i| parse_number(&line[i..]))
                .unwrap();

            let last_digit = (0..line.len())
                .rev()
                .find_map(|i| parse_number(&line[i..]))
                .unwrap();

            first_digit * 10 + last_digit
        })
        .sum::<u64>();

    (part1, part2)
}

const NUMBER_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_number(string: &str) -> Option<u64> {
    if let Some(value) = string.chars().next().unwrap().to_digit(10) {
        return Some(value as u64);
    }

    for (i, number_str) in NUMBER_STRS.iter().enumerate() {
        if string.starts_with(number_str) {
            return Some(i as u64 + 1);
        }
    }

    None
}
