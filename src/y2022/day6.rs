pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let chars = input.chars().collect::<Vec<_>>();

    let part1 = find_first_distinct(&chars, 4);
    let part2 = find_first_distinct(&chars, 14);

    (part1, part2)
}

fn find_first_distinct(chars: &[char], size: usize) -> usize {
    chars
        .windows(size)
        .enumerate()
        .find(|(_, window)| {
            !window
                .iter()
                .enumerate()
                .any(|(i, a)| window.iter().skip(i + 1).any(|b| a == b))
        })
        .unwrap()
        .0
        + size
}
