use super::intcode::Machine;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let initial_machine = Machine::from_str(input);

    let mut machine = initial_machine.clone();
    machine.memory[1] = 12;
    machine.memory[2] = 2;
    machine.run(None);
    let part1 = machine.memory[0];

    let mut part2 = None;
    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = initial_machine.clone();
            machine.memory[1] = noun;
            machine.memory[2] = verb;
            machine.run(None);
            if machine.memory[0] == 19690720 {
                part2 = Some(100 * noun + verb);
                break 'outer;
            }
        }
    }
    (part1, part2.unwrap())
}
