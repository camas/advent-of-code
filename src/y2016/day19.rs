use std::collections::VecDeque;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let elf_count = input.trim().parse::<usize>().unwrap();

    let mut elves = (1..=elf_count).collect::<VecDeque<_>>();
    for _ in 0..(elves.len() - 1) {
        let front = elves.pop_front().unwrap();
        elves.push_back(front);
        elves.pop_front();
    }
    let part1 = elves[0];

    let part2 = solve_across(elf_count);

    (part1, part2)
}

fn solve_across(elf_count: usize) -> usize {
    let mut lower = 1;
    while lower * 3 < elf_count {
        lower *= 3;
    }
    if elf_count == lower {
        return elf_count;
    }
    let upper = lower * 3;
    let diff = upper - lower;
    let offset = elf_count - lower;
    if offset <= diff / 2 {
        offset
    } else {
        let twos = offset - (diff / 2);
        (diff / 2) + twos * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_across() {
        for test in 1..5_000 {
            let mut elves = (1..=test).collect::<VecDeque<_>>();
            for _ in 0..(elves.len() - 1) {
                elves.remove(elves.len() / 2);
                let front = elves.pop_front().unwrap();
                elves.push_back(front);
            }
            assert_eq!(elves[0], solve_across(test));
        }
    }
}
