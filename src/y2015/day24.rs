use itertools::*;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut weights = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let weights_sum: u64 = weights.iter().sum();
    assert_eq!(weights_sum % 3, 0);
    let target: u64 = weights.iter().sum::<u64>() / 3;
    weights.sort_unstable();
    weights.reverse();
    let mut best_ent = u64::MAX;
    for comb_length in 1..=weights.len() {
        let mut found_some = false;
        for comb in weights.iter().combinations(comb_length) {
            let comb = comb.into_iter().cloned().collect::<Vec<_>>();
            if comb.iter().sum::<u64>() != target {
                continue;
            }
            found_some = true;
            let ent = comb.iter().product();
            if ent < best_ent {
                best_ent = ent;
            }
        }

        if found_some {
            break;
        }
    }
    let part1 = best_ent;

    let mut weights = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let weights_sum: u64 = weights.iter().sum();
    assert_eq!(weights_sum % 3, 0);
    let target: u64 = weights.iter().sum::<u64>() / 4;
    weights.sort_unstable();
    weights.reverse();
    let mut best_ent = u64::MAX;
    for comb_length in 1..=weights.len() {
        let mut found_some = false;
        for comb in weights.iter().combinations(comb_length) {
            let comb = comb.into_iter().cloned().collect::<Vec<_>>();
            if comb.iter().sum::<u64>() != target {
                continue;
            }
            found_some = true;
            let ent = comb.iter().product();
            if ent < best_ent {
                best_ent = ent;
            }
        }

        if found_some {
            break;
        }
    }
    let part2 = best_ent;

    (part1, part2)
}
