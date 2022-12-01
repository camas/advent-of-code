pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let (start, end) = input.trim().split_once('-').unwrap();
    let (start, end) = (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap());

    let mut part1 = 0;
    let mut part2 = 0;
    for n in start..=end {
        let digits = (0..6)
            .map(|i| (n / 10_i64.pow(i)) % 10)
            .rev()
            .collect::<Vec<_>>();
        let two_consecutive = digits.iter().zip(digits[1..].iter()).any(|(a, b)| a == b);
        if !two_consecutive {
            continue;
        }
        let never_decrease = digits.iter().zip(digits[1..].iter()).all(|(a, b)| a <= b);
        if !never_decrease {
            continue;
        }
        part1 += 1;
        let contains_pair = (0..(digits.len() - 1)).any(|i| {
            digits[i] == digits[i + 1]
                && (i == 0 || digits[i - 1] != digits[i])
                && (i >= digits.len() - 2 || digits[i + 2] != digits[i])
        });
        if !contains_pair {
            continue;
        }
        part2 += 1;
    }

    (part1, part2)
}
