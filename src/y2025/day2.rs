use num::Integer;
use winnow::{
    ascii::dec_int,
    combinator::separated,
    error::{ContextError, ParserError},
    Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let mut part1 = 0_i64;
    let mut part2 = 0_i64;

    for range in input.ranges.iter() {
        for number in range.start..=range.end {
            let digits = number.ilog10() + 1;
            // println!("n: {number} digits: {digits}");
            if digits.is_even() {
                let low = number % 10_i64.pow(digits / 2);
                let high = number / 10_i64.pow(digits / 2);
                // println!("n: {number} l: {low} h: {high}");
                if low == high {
                    part1 += number;
                }
            }

            let invalid = all_digits_same(number, digits)
                || match digits {
                    1 | 3 | 5 | 7 => false,
                    2 => number / 10 == number % 10,
                    4 => number / 100 == number % 100,
                    6 => {
                        (number / 1000 == number % 1000) || {
                            let low = number % 100;
                            (low == number / 10000) && (low == (number / 100) % 100)
                        }
                    }
                    8 => number / 10000 == number % 10000,
                    9 => {
                        let low = number % 1000;
                        (low == number / 1000000) && (low == (number / 1000) % 1000)
                    }
                    10 => {
                        (number / 100000 == number % 100000) || {
                            let low = number % 100;
                            (low == number / 100000000)
                                && (low == (number / 1000000) % 100)
                                && (low == (number / 10000) % 100)
                                && (low == (number / 100) % 100)
                        }
                    }
                    _ => {
                        panic!("haven't checked every factor for digits this high. 2-10 unrolled above")
                    }
                };
            if invalid {
                part2 += number;
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

fn all_digits_same(number: i64, digits: u32) -> bool {
    if digits == 1 {
        return false;
    }

    let low = number % 10;
    (1..digits).all(|i| ((number / 10_i64.pow(i)) % 10) == low)
}

struct Input {
    ranges: Vec<Range>,
}

impl Input {
    fn parse(input: &str) -> Self {
        separated(1.., Range::parse::<ContextError<&str>>, ',')
            .parse(input.trim_end())
            .map(|ranges| Self { ranges })
            .unwrap()
    }
}

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn parse<'a, E: ParserError<&'a str>>(input: &mut &'a str) -> winnow::Result<Self, E> {
        (dec_int, '-', dec_int)
            .parse_next(input)
            .map(|(start, _, end)| Self { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known() {
        assert_eq!(solve("11-22").0.to_string(), "33");
        assert_eq!(solve("998-1012").0.to_string(), "1010");

        assert_eq!(solve("11-22").1.to_string(), "33");
        assert_eq!(solve("95-115").1.to_string(), "210");
        assert_eq!(solve("998-1012").1.to_string(), "2009");
        assert_eq!(solve("1188511880-1188511890").1.to_string(), "1188511885");
        assert_eq!(solve("222220-222224").1.to_string(), "222222");
        assert_eq!(solve("1698522-1698528").1.to_string(), "0");
        assert_eq!(solve("446443-446449").1.to_string(), "446446");
        assert_eq!(solve("38593856-38593862").1.to_string(), "38593859");
        assert_eq!(solve("565653-565659").1.to_string(), "565656");
        assert_eq!(solve("824824821-824824827").1.to_string(), "824824824");
        assert_eq!(solve("2121212118-2121212124").1.to_string(), "2121212121");

        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (result1, result2) = solve(input);
        assert_eq!(result1.to_string(), "1227775554");
        assert_eq!(result2.to_string(), "4174379265");
    }
}
