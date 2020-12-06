pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let target = input.trim().parse::<usize>().unwrap() / 10;
    let mut sieve = [0; 1_000_000];
    let mut part1 = 0;
    for factor in 1..sieve.len() {
        for product in (factor..sieve.len()).step_by(factor) {
            sieve[product] += factor;
        }
        if sieve[factor] >= target {
            part1 = factor;
            break;
        }
    }

    let target = input.trim().parse::<usize>().unwrap() / 11;
    sieve = [0; 1_000_000];
    for factor in 1..sieve.len() {
        for product in (factor..sieve.len()).step_by(factor).take(50) {
            sieve[product] += factor;
        }
        if sieve[factor] >= target {
            let part2 = factor.to_string();
            return (part1, part2);
        }
    }
    unreachable!();
}
