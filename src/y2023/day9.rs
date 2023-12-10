pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let sensor_data = parse_input(input);

    let sensor_data_with_differences = sensor_data
        .into_iter()
        .map(|values| {
            let mut rows = vec![values];
            loop {
                let current_row = rows.last().unwrap();
                if current_row.iter().all(|v| *v == 0) {
                    break;
                }

                let new_row = current_row
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>();
                rows.push(new_row);
            }
            rows
        })
        .collect::<Vec<_>>();

    let part1 = sensor_data_with_differences
        .iter()
        .map(|rows| {
            rows.iter()
                .rev()
                .fold(0, |acc, row| acc + row.last().unwrap())
        })
        .sum::<i64>();

    let part2 = sensor_data_with_differences
        .iter()
        .map(|rows| rows.iter().rev().fold(0, |acc, row| row[0] - acc))
        .sum::<i64>();

    (part1, part2)
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|value| value.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "114");
        assert_eq!(part2.to_string(), "2");
    }
}
