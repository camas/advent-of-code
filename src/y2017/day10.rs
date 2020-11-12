use crate::Exercise;

pub struct Day10;

impl Exercise for Day10 {
    fn part1(&self, input: &str) -> String {
        let lengths = input
            .trim()
            .split(',')
            .map(|part| part.parse::<usize>().unwrap());
        let mut numbers = (0..=255).collect::<Vec<_>>();
        let mut position = 0;
        for (skip_size, length) in lengths.enumerate() {
            for i in 0..(length / 2) {
                let swap_a = (position + i) % numbers.len();
                let swap_b = (position + length - i - 1) % numbers.len();
                numbers.swap(swap_a, swap_b);
            }
            position = (position + skip_size + length) % numbers.len();
        }
        (numbers[0] * numbers[1]).to_string()
    }

    fn part2(&self, input: &str) -> String {
        str_knot_hash(input.trim())
    }
}

pub fn str_knot_hash(input: &str) -> String {
    let lengths = input.chars().map(|c| c as usize).collect::<Vec<_>>();
    let result = knot_hash(&lengths);
    result
        .iter()
        .map(|num| format!("{:0>2x?}", num))
        .collect::<Vec<_>>()
        .join("")
}

pub fn knot_hash(input: &[usize]) -> Vec<u8> {
    const ROUNDS: usize = 64;
    let mut lengths = input.to_vec();
    lengths.extend([17, 31, 73, 47, 23].iter());
    let mut numbers = (0..=255).collect::<Vec<_>>();
    let mut position = 0;
    for (skip_size, length) in lengths
        .iter()
        .cycle()
        .take(lengths.len() * ROUNDS)
        .enumerate()
    {
        for i in 0..(length / 2) {
            let swap_a = (position + i) % numbers.len();
            let swap_b = (position + length - i - 1) % numbers.len();
            numbers.swap(swap_a, swap_b);
        }
        position = (position + skip_size + length) % numbers.len();
    }

    let dense_hash = numbers
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, val| acc ^ val))
        .collect::<Vec<_>>();
    assert_eq!(dense_hash.len(), 16);
    dense_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        assert_eq!(str_knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            str_knot_hash("AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(str_knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(str_knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
