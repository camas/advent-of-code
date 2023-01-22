pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let data = input
        .trim()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mixed_data = mix(&data, 1);
    let zero_index = mixed_data.iter().position(|v| *v == 0).unwrap();
    let part1 = [1000, 2000, 3000]
        .iter()
        .map(|i| mixed_data[(zero_index + i) % mixed_data.len()])
        .sum::<i64>();

    let data = data.into_iter().map(|v| v * 811589153).collect::<Vec<_>>();
    let mixed_data = mix(&data, 10);
    let zero_index = mixed_data.iter().position(|v| *v == 0).unwrap();
    let part2 = [1000, 2000, 3000]
        .iter()
        .map(|i| mixed_data[(zero_index + i) % mixed_data.len()])
        .sum::<i64>();

    (part1, part2)
}

fn mix(data: &[i64], mix_count: u32) -> Vec<i64> {
    let data_len = data.len();

    let mut data = data
        .iter()
        .enumerate()
        .map(|(i, v)| Entry {
            initial_index: i,
            value: *v,
        })
        .collect::<Vec<_>>();

    for _ in 0..mix_count {
        for i in 0..data_len {
            // println!("{:?}", data.iter().map(|e| e.value.to_string()).join(", "));
            let entry_index = data.iter().position(|e| e.initial_index == i).unwrap();
            let entry = data.remove(entry_index);
            let new_offset =
                (entry_index as i64 + entry.value).rem_euclid(data_len as i64 - 1) as usize;
            data.insert(new_offset, entry);
        }
    }

    data.into_iter().map(|e| e.value).collect()
}

#[derive(Debug)]
struct Entry {
    initial_index: usize,
    value: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "1
2
-3
3
-2
0
4";
        let result = solve(input);

        assert_eq!(result.0.to_string(), 3.to_string());
        assert_eq!(result.1.to_string(), 1623178306.to_string());
    }
}
