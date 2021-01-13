pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let initial = input
        .trim()
        .chars()
        .map(|c| match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let part1 = count_traps(&initial, 40);
    let part2 = count_traps(&initial, 400_000);

    (part1, part2)
}

fn count_traps(initial: &[Tile], rows: usize) -> usize {
    let mut total = 0;
    let mut curr = initial.to_vec();
    for _ in 0..rows {
        let count = curr.iter().filter(|t| matches!(t, Tile::Safe)).count();
        total += count;
        let next = (0..curr.len())
            .map(|i| {
                let left_trap = if i == 0 {
                    false
                } else {
                    matches!(curr[i - 1], Tile::Trap)
                };
                // Simplified out
                // let middle_trap = matches!(curr[i], Tile::Trap);
                let right_trap = if i == curr.len() - 1 {
                    false
                } else {
                    matches!(curr[i + 1], Tile::Trap)
                };
                if (left_trap && !right_trap) || (!left_trap && right_trap) {
                    Tile::Trap
                } else {
                    Tile::Safe
                }
            })
            .collect();
        curr = next;
    }
    total
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Trap,
    Safe,
}
