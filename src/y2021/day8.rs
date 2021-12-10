use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let displays = input
        .lines()
        .map(|l| {
            let mut parts = l.split(" | ");
            let signals = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            let outputs = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            (signals, outputs)
        })
        .collect::<Vec<_>>();

    let part1 = displays
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|o| matches!(o.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum::<usize>();

    let mut part2 = 0;
    for (mut signals, outputs) in displays {
        let mut known_digits = HashMap::new();

        macro_rules! find_digit {
            ($digit:expr, $pattern:expr) => {{
                let index = signals.iter().position($pattern).unwrap();
                known_digits.insert($digit, signals.swap_remove(index));
            }};
        }

        // First get digits with unique indexes
        find_digit!(1, |s| s.len() == 2);
        find_digit!(7, |s| s.len() == 3);
        find_digit!(4, |s| s.len() == 4);
        find_digit!(8, |s| s.len() == 7);

        // Only 9 contains the same positions as 4
        find_digit!(9, |s| s.is_superset(&known_digits[&4]));

        // 0 only 6 segment digit containing 7
        find_digit!(0, |s| s.len() == 6 && s.is_superset(&known_digits[&7]));

        // 6 the only 6 segment digit left
        find_digit!(6, |s| s.len() == 6);

        // 3 only one left that contains 1
        find_digit!(3, |s| s.is_superset(&known_digits[&1]));

        // 5 the only one left that's a subset of 6
        find_digit!(5, |s| s.is_subset(&known_digits[&6]));

        // 2 the last one
        known_digits.insert(2, signals.pop().unwrap());

        assert!(signals.is_empty());

        let digits = outputs.into_iter().map(|o| {
            known_digits
                .iter()
                .find_map(|(k, v)| if v == &o { Some(k) } else { None })
                .unwrap()
        });
        let output = digits
            .enumerate()
            .map(|(i, d)| d * 10_i64.pow((3 - i) as u32))
            .sum::<i64>();
        part2 += output;
    }

    (part1, part2)
}
