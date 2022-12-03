pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let calories = input
        .split("\n\n")
        .map(|part| {
            part.trim()
                .split('\n')
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut calories_summed = calories
        .iter()
        .map(|calories| calories.iter().sum::<i64>())
        .collect::<Vec<_>>();

    let part1 = *calories_summed.iter().max().unwrap();

    calories_summed.sort_unstable_by(|a, b| a.cmp(b).reverse());
    let part2 = calories_summed.iter().take(3).sum::<i64>();

    (part1, part2)
}
