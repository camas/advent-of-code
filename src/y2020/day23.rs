use std::{fmt::Write, iter};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let cup_values = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let highest_val = *cup_values.iter().max().unwrap();
    assert_eq!(highest_val, cup_values.len());
    let first_cup = cup_values[0];

    let mut cups = vec![0; cup_values.len() + 1];
    for (a, b) in cup_values.iter().zip(cup_values.iter().skip(1)) {
        cups[*a] = *b;
    }
    cups[*cup_values.last().unwrap()] = cup_values[0];
    let result1 = cup_game(cups.clone(), 100, first_cup);
    let mut part1_digits = Vec::new();
    let mut index = 1;
    for _ in 0..(highest_val - 1) {
        index = result1[index];
        part1_digits.push(index);
    }
    let part1 = part1_digits
        .into_iter()
        .fold(String::new(), |mut result, d| {
            write!(result, "{d}").unwrap();
            result
        });

    let mut cups = cups
        .into_iter()
        .chain((highest_val + 2)..=1_000_000)
        .chain(iter::once(cup_values[0]))
        .collect::<Vec<_>>();
    cups[*cup_values.last().unwrap()] = 10;
    let result2 = cup_game(cups, 10_000_000, first_cup);
    let part2 = result2[1] * result2[result2[1]];

    (part1, part2)
}

fn cup_game(initial_cups: Vec<usize>, rounds: usize, first_cup: usize) -> Vec<usize> {
    let mut cups = initial_cups;
    let highest = cups.len() - 1;
    let mut index = first_cup;
    for _ in 0..rounds {
        let start = index;
        let a = cups[start];
        let b = cups[a];
        let c = cups[b];
        let end = cups[c];
        let mut after = start - 1;
        if after == 0 {
            after = highest;
        }
        while after == a || after == b || after == c {
            after -= 1;
            if after == 0 {
                after = highest;
            }
        }
        let temp = cups[after];
        cups[index] = end;
        cups[after] = a;
        cups[c] = temp;

        index = cups[index];
    }
    cups
}
