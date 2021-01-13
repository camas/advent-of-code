pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let nums = input
        .lines()
        .map(|l| {
            l.trim_start()
                .split_whitespace()
                .map(|p| p.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = nums
        .iter()
        .filter(|row| possible(row[0], row[1], row[2]))
        .count();

    let part2 = nums
        .chunks(3)
        .map(|chunk| {
            (0..3)
                .filter(|&i| possible(chunk[0][i], chunk[1][i], chunk[2][i]))
                .count()
        })
        .sum::<usize>();

    (part1, part2)
}

fn possible(a: u64, b: u64, c: u64) -> bool {
    if a > b {
        if a > c {
            b + c > a
        } else {
            a + b > c
        }
    } else if b > c {
        a + c > b
    } else {
        a + b > c
    }
}
