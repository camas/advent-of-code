pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let numbers = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Count the number of set bits for each position
    let mut zero_counts = vec![0; numbers[0].len()];
    let mut one_counts = vec![0; numbers[0].len()];
    for n in numbers.iter() {
        for (i, &v) in n.iter().enumerate() {
            if v == 1 {
                one_counts[i] += 1;
            } else {
                zero_counts[i] += 1;
            }
        }
    }

    let gamma = one_counts
        .iter()
        .map(|c| {
            if *c > (numbers.len() as u32 / 2) {
                1
            } else {
                0
            }
        })
        .collect::<Vec<_>>();
    let epsilon = gamma
        .iter()
        .map(|v| if *v == 1 { 0 } else { 1 })
        .collect::<Vec<_>>();

    // Part 1 = most common (gamma) * least common (epsilon)
    let part1 = bits_to_int(&gamma) * bits_to_int(&epsilon);

    let most_common = one_counts
        .iter()
        .zip(zero_counts.iter())
        .map(|(o, z)| if o >= z { 1 } else { 0 })
        .collect::<Vec<_>>();
    let mut search_space = numbers.clone();
    let mut position = 0;
    while search_space.len() > 1 {
        let most_common = most_common[position];
        search_space = search_space
            .into_iter()
            .filter(|n| n[position] == most_common)
            .collect();
        position += 1;
    }
    let oxygen_rating = bits_to_int(&search_space[0]);

    let least_common = one_counts
        .iter()
        .zip(zero_counts.iter())
        .map(|(o, z)| if z <= o { 0 } else { 1 })
        .collect::<Vec<_>>();
    let mut search_space = numbers;
    let mut position = 0;
    while search_space.len() > 1 {
        let least_common = least_common[position];
        search_space = search_space
            .into_iter()
            .filter(|n| n[position] == least_common)
            .collect();
        position += 1;
    }
    let co2_rating = bits_to_int(&search_space[0]);

    let part2 = oxygen_rating * co2_rating;

    (part1, part2)
}

fn bits_to_int(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |acc, &v| acc * 2 + v as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_int() {
        assert_eq!(bits_to_int(&[1, 1, 0, 1]), 13);
    }
}
