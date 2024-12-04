use winnow::{ascii::digit1, PResult, Parser};

pub fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1
        .map(|digits: &str| digits.parse::<u32>().unwrap())
        .parse_next(input)
}
