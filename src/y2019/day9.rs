use super::intcode::{Handler, Machine};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut machine = Machine::from_str(input);

    let mut handler = DayHandler {
        input: 1,
        output: None,
    };
    machine.clone().run(Some(&mut handler));
    let part1 = handler.output.unwrap();

    let mut handler = DayHandler {
        input: 2,
        output: None,
    };
    machine.run(Some(&mut handler));
    let part2 = handler.output.unwrap();

    (part1, part2)
}

struct DayHandler {
    input: i64,
    output: Option<i64>,
}

impl Handler for DayHandler {
    fn input(&mut self, _: &Machine) -> i64 {
        self.input
    }

    fn output(&mut self, _: &Machine, value: i64) {
        self.output = Some(value);
    }
}
