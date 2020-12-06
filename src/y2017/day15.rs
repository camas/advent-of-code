
pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();
    let initial_a = lines.next().unwrap()[24..].parse::<u64>().unwrap();
    let initial_b = lines.next().unwrap()[24..].parse::<u64>().unwrap();
    let mut matches = 0;
    let mut a = initial_a;
    let mut b = initial_b;
    for _step in 0..40_000_000 {
        a = (a * 16_807) % 2_147_483_647;
        b = (b * 48_271) % 2_147_483_647;
        if (a & 0xffff) == (b & 0xffff) {
            matches += 1;
        }
    }
    let part1 = matches;

    let mut matches = 0;
    let mut a = initial_a;
    let mut b = initial_b;
    for _step in 0..5_000_000 {
        loop {
            a = (a * 16_807) % 2_147_483_647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = (b * 48_271) % 2_147_483_647;
            if b % 8 == 0 {
                break;
            }
        }
        if (a & 0xffff) == (b & 0xffff) {
            matches += 1;
        }
    }
    let part2 = matches;

    (part1, part2)
}
