use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();

    let winning_number_counts = cards
        .iter()
        .map(|card| {
            card.picked_numbers
                .iter()
                .filter(|picked| card.winning_numbers.contains(picked))
                .count() as u64
        })
        .collect::<Vec<_>>();

    let part1 = winning_number_counts
        .iter()
        .map(|&winning_number_count| {
            if winning_number_count == 0 {
                0
            } else {
                2_u64.pow(winning_number_count as u32 - 1)
            }
        })
        .sum::<u64>();

    let mut counts = vec![1; cards.len()];
    for i in 0..counts.len() {
        let winning_number_count = winning_number_counts[i];
        let to_add = counts[i];

        counts
            .iter_mut()
            .skip(i + 1)
            .take(winning_number_count as usize)
            .for_each(|count| *count += to_add);
    }
    let part2 = counts.iter().sum::<u64>();

    (part1, part2)
}

struct Card {
    winning_numbers: Vec<u32>,
    picked_numbers: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    all_consuming(map(
        tuple((
            tag("Card"),
            multispace1,
            digit1,
            tag(":"),
            multispace1,
            parse_numbers,
            tag(" |"),
            multispace1,
            parse_numbers,
        )),
        |(_, _, _, _, _, winning_numbers, _, _, picked_numbers)| Card {
            winning_numbers,
            picked_numbers,
        },
    ))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    map(separated_list1(multispace1, digit1), |a| {
        a.into_iter()
            .map(|b: &str| b.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    })(input)
}
