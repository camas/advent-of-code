pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let spreadsheet = input
        .lines()
        .map(|line| {
            line.split('\t')
                .map(|value| value.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let checksum: u64 = spreadsheet
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum();
    let part1 = checksum;

    let spreadsheet = input
        .lines()
        .map(|line| {
            line.split('\t')
                .map(|value| value.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let part2 = spreadsheet
        .iter()
        .map(|row| -> u64 {
            let mut value = 0;
            'outer: for (i, &a) in row.iter().enumerate() {
                for &b in row.iter().skip(i + 1) {
                    if a % b == 0 {
                        value = a / b;
                        break 'outer;
                    }
                    if b % a == 0 {
                        value = b / a;
                        break 'outer;
                    }
                }
            }
            assert_ne!(value, 0);
            value
        })
        .sum::<u64>();

    (part1, part2)
}
