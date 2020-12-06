pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = input
        .iter()
        .enumerate()
        .find_map(|(i, a)| {
            input
                .iter()
                .skip(i + 1)
                .find_map(|b| if a + b == 2020 { Some(a * b) } else { None })
        })
        .unwrap();

    let part2 = input
        .iter()
        .enumerate()
        .find_map(|(i, a)| {
            input.iter().enumerate().skip(i + 1).find_map(|(j, b)| {
                input.iter().skip(j + 1).find_map(|c| {
                    if a + b + c == 2020 {
                        Some(a * b * c)
                    } else {
                        None
                    }
                })
            })
        })
        .unwrap();

    (part1, part2)
}
