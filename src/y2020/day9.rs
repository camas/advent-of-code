pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = (25..input.len())
        .find_map(|i| {
            let num = input[i];
            let has_sum = input
                .iter()
                .skip(i - 25)
                .take(25)
                .enumerate()
                .any(|(u, &a)| {
                    input
                        .iter()
                        .skip(i - 25)
                        .take(25)
                        .enumerate()
                        .any(|(v, &b)| u != v && a + b == num)
                });
            if !has_sum {
                Some(num)
            } else {
                None
            }
        })
        .unwrap();

    let part2 = (0..input.len())
        .find_map(|i| {
            let mut sum = 0;
            let j = input[i..]
                .iter()
                .enumerate()
                .find(|(_, &n)| {
                    sum += n;
                    sum >= part1
                })
                .unwrap()
                .0;
            if sum == part1 {
                let range = &input[i..(i + j)];
                Some(range.iter().max().unwrap() + range.iter().min().unwrap())
            } else {
                None
            }
        })
        .unwrap();

    (part1, part2)
}
