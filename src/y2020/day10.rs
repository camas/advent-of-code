use std::iter;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let adapters = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Part 1: Connect all adaptors and find the differences between them
    // Add 0, max + 3 and sort
    let mut full_adapters = iter::once(&0)
        .chain(adapters.iter())
        .chain(iter::once(&(adapters.iter().max().unwrap() + 3)))
        .cloned()
        .collect::<Vec<_>>();
    full_adapters.sort_unstable();
    // Calculate differences
    let differences = full_adapters
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();
    // Count differences
    let mut threes = 0;
    let mut ones = 0;
    differences.iter().for_each(|a| match a {
        3 => threes += 1,
        1 => ones += 1,
        _ => unreachable!(),
    });
    // Calculate part 1
    let part1 = ones * threes;

    // Part 2: Count the possible ways to connect 0 and max + 3
    // Essentially just need to work out how many different ways you can connect a series of 1 difference adaptors
    // Sequence starts with 1, 2, 4 and each value after is the sum of the previous 3
    // We'll take the simple route and calculate new values as needed
    let mut one_count = 0;
    let mut one_seqs = Vec::new();
    for &diff in differences.iter() {
        match diff {
            1 => one_count += 1,
            3 => {
                if one_count > 0 {
                    one_seqs.push(one_count);
                    one_count = 0;
                }
            }
            _ => unreachable!(),
        }
    }
    if one_count > 0 {
        one_seqs.push(one_count);
    }
    // Answer is the product of the number of different ways you can do each 1,1,1... section
    let mut seqs = vec![1, 2, 4];
    let mut multipliers = Vec::new();
    for &one_count in one_seqs.iter() {
        while seqs.len() < one_count {
            seqs.push(seqs[seqs.len() - 1] + seqs[seqs.len() - 2] + seqs[seqs.len() - 3]);
        }
        multipliers.push(seqs[one_count - 1]);
    }
    let part2 = multipliers.into_iter().product::<usize>();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let results = solve(input);
        assert_eq!(results.0.to_string(), "35");
        assert_eq!(results.1.to_string(), "8");
    }
}
