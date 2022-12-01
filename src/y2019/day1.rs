pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let masses = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = masses.iter().map(|mass| (mass / 3) - 2).sum::<i64>();

    let mut total = 0;
    for mass in masses {
        let mut curr = mass;
        loop {
            let fuel = (curr / 3) - 2;
            if fuel <= 0 {
                break;
            }
            total += fuel;
            curr = fuel;
        }
    }
    let part2 = total;

    (part1, part2)
}
