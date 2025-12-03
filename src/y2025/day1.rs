use itertools::Itertools;
use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::separated,
    error::ParserError,
    Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input).unwrap();

    ("", "")
}

struct Input {}

impl Input {
    fn parse(input: &str) -> winnow::Result<Self> {
        todo!()
    }
}
