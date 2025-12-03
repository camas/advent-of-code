use winnow::{ascii::digit1, Parser};

pub fn parse_u32(input: &mut &str) -> winnow::Result<u32> {
    digit1
        .map(|digits: &str| digits.parse::<u32>().unwrap())
        .parse_next(input)
}
