use winnow::{
    combinator::{repeat, separated},
    error::ContextError,
    token::one_of,
    Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = Input::parse(input);

    let mut part1 = 0_i64;
    let mut part2 = 0_i64;

    for bank in input.banks.iter() {
        let (i, first_digit) = find_max_initial(bank, 1);
        let second_digit = bank.iter().skip(i + 1).max().unwrap();

        // let line = bank.iter().map(|v| v.to_string()).collect::<String>();
        // println!("{line} {first_digit}{second_digit}");
        part1 += (first_digit * 10 + second_digit) as i64;

        let (i0, d0) = find_max_initial(bank, 11);
        let (i1, d1) = find_max(bank, i0 + 1, 10);
        let (i2, d2) = find_max(bank, i1 + 1, 9);
        let (i3, d3) = find_max(bank, i2 + 1, 8);
        let (i4, d4) = find_max(bank, i3 + 1, 7);
        let (i5, d5) = find_max(bank, i4 + 1, 6);
        let (i6, d6) = find_max(bank, i5 + 1, 5);
        let (i7, d7) = find_max(bank, i6 + 1, 4);
        let (i8_, d8) = find_max(bank, i7 + 1, 3);
        let (i9, d9) = find_max(bank, i8_ + 1, 2);
        let (i10, d10) = find_max(bank, i9 + 1, 1);
        let d11 = bank.iter().skip(i10 + 1).max().unwrap();

        part2 += (*d0 as i64 * 10_i64.pow(11))
            + (*d1 as i64 * 10_i64.pow(10))
            + (*d2 as i64 * 10_i64.pow(9))
            + (*d3 as i64 * 10_i64.pow(8))
            + (*d4 as i64 * 10_i64.pow(7))
            + (*d5 as i64 * 10_i64.pow(6))
            + (*d6 as i64 * 10_i64.pow(5))
            + (*d7 as i64 * 10_i64.pow(4))
            + (*d8 as i64 * 10_i64.pow(3))
            + (*d9 as i64 * 10_i64.pow(2))
            + (*d10 as i64 * 10_i64)
            + (*d11 as i64);
    }

    (part1.to_string(), part2.to_string())
}

fn find_max_initial(bank: &[u8], limit: usize) -> (usize, &u8) {
    bank[0..(bank.len() - limit)]
        .iter()
        .enumerate()
        .max_by(|(a_i, a_v), (b_i, b_v)| a_v.cmp(b_v).then(a_i.cmp(b_i).reverse()))
        .unwrap()
}

fn find_max(bank: &[u8], skip: usize, limit: usize) -> (usize, &u8) {
    bank[0..(bank.len() - limit)]
        .iter()
        .enumerate()
        .skip(skip)
        .max_by(|(a_i, a_v), (b_i, b_v)| a_v.cmp(b_v).then(a_i.cmp(b_i).reverse()))
        .unwrap()
}

struct Input {
    banks: Vec<Vec<u8>>,
}

impl Input {
    fn parse(input: &str) -> Self {
        separated(
            1..,
            repeat::<_, _, Vec<u8>, _, _>(
                1..,
                one_of::<_, _, ContextError<&str>>('0'..='9')
                    .map(|c: char| c.to_digit(10).unwrap() as u8),
            ),
            '\n',
        )
        .parse(input.trim_ascii_end())
        .map(|banks| Self { banks })
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let (result1, result2) = solve(input);
        assert_eq!(result1.to_string(), "357");
        assert_eq!(result2.to_string(), "3121910778619");
    }
}
