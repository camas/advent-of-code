use super::intcode::{Handler, Machine};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut machine = Machine::from_str(input);

    let mut handler = NumberHandler::new(1);
    machine.clone().run(Some(&mut handler));
    let part1 = *handler.outputs.last().unwrap();

    let mut handler = NumberHandler::new(5);
    machine.run(Some(&mut handler));
    let part2 = *handler.outputs.last().unwrap();

    (part1, part2)
}

struct NumberHandler {
    number: i64,
    outputs: Vec<i64>,
}

impl NumberHandler {
    fn new(number: i64) -> Self {
        Self {
            number,
            outputs: Vec::new(),
        }
    }
}

impl Handler for NumberHandler {
    fn input(&mut self, _: &Machine) -> i64 {
        self.number
    }

    fn output(&mut self, _: &Machine, value: i64) {
        self.outputs.push(value);
    }
}
