pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let depths = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let part1 = depths.windows(2).filter(|x| x[0] < x[1]).count();
    let windows = depths
        .windows(3)
        .map(|x| x.iter().sum::<i32>())
        .collect::<Vec<_>>();
    let part2 = windows.windows(2).filter(|x| x[0] < x[1]).count();

    (part1, part2)
}
