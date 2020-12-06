use std::collections::VecDeque;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let step_count = input.trim().parse::<usize>().unwrap();

    let mut spinlock = VecDeque::new();
    spinlock.push_front(0);
    for i in 1..=2017 {
        let shift = (step_count + 1) % spinlock.len();
        spinlock.rotate_left(shift);
        spinlock.push_front(i);
    }
    let part1 = spinlock[1];

    let mut spinlock = VecDeque::new();
    spinlock.push_front(0);
    for i in 1..=50_000_000 {
        let shift = (step_count + 1) % spinlock.len();
        spinlock.rotate_left(shift);
        spinlock.push_front(i);
    }
    let pos = spinlock.iter().position(|&ele| ele == 0).unwrap();
    let part2 = spinlock[pos + 1];

    (part1, part2)
}
