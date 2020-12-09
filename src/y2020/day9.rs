pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let part1_index = (25..input.len())
        .find(|&i| {
            let num = input[i];
            let mut found = false;
            'outer: for (u, &a) in input.iter().skip(i - 25).take(25).enumerate() {
                for (v, &b) in input.iter().skip(i - 25).take(25).enumerate() {
                    if u == v {
                        continue;
                    }
                    if a + b == num {
                        found = true;
                        break 'outer;
                    }
                }
            }
            !found
        })
        .unwrap();
    let part1 = input[part1_index];

    let mut part2 = 0;
    'outer: for i in 0..input.len() {
        let mut sum = 0;
        for (j, &value) in input[i..].iter().enumerate() {
            sum += value;
            if sum == part1 {
                let range = &input[i..(i + j)];
                part2 = range.iter().max().unwrap() + range.iter().min().unwrap();
                break 'outer;
            }
            if sum > part1 {
                continue 'outer;
            }
        }
    }

    (part1, part2)
}
