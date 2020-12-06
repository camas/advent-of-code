
pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut offsets = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut pointer = 0_i32;
    let mut part1 = 0;
    for step in 0.. {
        if pointer < 0 || pointer >= offsets.len() as i32 {
            part1 = step;
            break;
        }
        let offset = offsets[pointer as usize];
        offsets[pointer as usize] += 1;
        pointer += offset;
    }

    let mut offsets = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut pointer = 0_i32;
    for step in 0.. {
        if pointer < 0 || pointer >= offsets.len() as i32 {
            return (part1, step);
        }
        let offset = offsets[pointer as usize];
        if offset >= 3 {
            offsets[pointer as usize] -= 1;
        } else {
            offsets[pointer as usize] += 1;
        }
        pointer += offset;
    }
    unreachable!();
}
