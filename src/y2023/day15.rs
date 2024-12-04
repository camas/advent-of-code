use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let part1 = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|step| hash(step) as u64)
        .sum::<u64>();

    let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
    for instruction in input.lines().flat_map(|line| line.split(',')) {
        if let Some(key) = instruction.strip_suffix('-') {
            let box_ = boxes.entry(hash(key)).or_default();
            box_.retain(|(lens_key, _)| lens_key != key);
        } else {
            let (key, value) = instruction.split_once('=').unwrap();
            let value = value.parse::<u8>().unwrap();
            let box_ = boxes.entry(hash(key)).or_default();
            match box_.iter_mut().find(|(lens_key, _)| lens_key == key) {
                Some((_, lens_value)) => {
                    *lens_value = value;
                }
                None => box_.push((key.to_string(), value)),
            }
        }
    }

    let part2 = boxes
        .into_iter()
        .map(|(box_key, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(i, (_, lens_value))| {
                    (box_key as u64 + 1) * (i as u64 + 1) * lens_value as u64
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    (part1, part2)
}

fn hash(data: &str) -> u8 {
    let mut value = 0_u8;
    for byte in data.as_bytes() {
        value = value.wrapping_add(*byte);
        value = value.wrapping_mul(17);
    }
    value
}
