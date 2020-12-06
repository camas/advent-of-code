pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let scanners = input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let depth = parts[0].parse::<u32>().unwrap();
            let range = parts[1].parse::<u32>().unwrap();
            (depth, range)
        })
        .collect::<Vec<_>>();

    let severity = scanners
        .iter()
        .map(|(depth, range)| {
            if depth % (2 * range - 2) == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum::<u32>();
    let part1 = severity;

    for wait in 0.. {
        if scanners
            .iter()
            .find(|(depth, range)| (wait + depth) % (2 * range - 2) == 0)
            .is_none()
        {
            return (part1, wait);
        }
    }
    unreachable!();
}
