use std::cmp::Ordering;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let numbers = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let inner_len = numbers[0].len();

    let gamma = most_common_bits(&numbers);
    let epsilon = gamma.iter().map(|v| u32::from(*v != 1)).collect::<Vec<_>>();

    // Part 1 = most common (gamma) * least common (epsilon)
    let part1 = bits_to_int(&gamma) * bits_to_int(&epsilon);

    let mut search_space = numbers.clone();
    for i in 0..inner_len {
        let most_common = most_common_bits(&search_space);
        search_space.retain(|v| v[i] == most_common[i]);
        if search_space.len() == 1 {
            break;
        }
    }
    let oxygen_rating = bits_to_int(&search_space[0]);

    let mut search_space = numbers;
    for i in 0..inner_len {
        let most_common = most_common_bits(&search_space);
        let least_common = most_common
            .into_iter()
            .map(|v| u32::from(v != 1))
            .collect::<Vec<_>>();
        search_space.retain(|v| v[i] == least_common[i]);
        if search_space.len() == 1 {
            break;
        }
    }
    let co2_rating = bits_to_int(&search_space[0]);

    let part2 = oxygen_rating * co2_rating;

    (part1, part2)
}

fn most_common_bits(numbers: &[Vec<u32>]) -> Vec<u32> {
    // Count the number of each bit in each position
    let mut counts = vec![(0, 0); numbers[0].len()];
    for n in numbers.iter() {
        for (i, &v) in n.iter().enumerate() {
            if v == 0 {
                counts[i].0 += 1;
            } else {
                counts[i].1 += 1;
            }
        }
    }
    // Return the most common bit, or 1 if there is a tie
    counts
        .into_iter()
        .map(|(zeros, ones)| match zeros.cmp(&ones) {
            Ordering::Less | Ordering::Equal => 1,
            Ordering::Greater => 0,
        })
        .collect::<Vec<_>>()
}

fn bits_to_int(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |acc, &v| acc * 2 + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_int() {
        assert_eq!(bits_to_int(&[1, 1, 0, 1]), 13);
    }
}
