use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult, Parser,
};

struct Game {
    id: u64,
    subsets: Vec<GameSubset>,
}

struct GameSubset {
    red: u64,
    green: u64,
    blue: u64,
}

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    let part1 = games
        .iter()
        .filter(|game| {
            game.subsets
                .iter()
                .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u64>();

    let part2 = games
        .iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for subset in game.subsets.iter() {
                max_red = max_red.max(subset.red);
                max_green = max_green.max(subset.green);
                max_blue = max_blue.max(subset.blue);
            }

            max_red * max_green * max_blue
        })
        .sum::<u64>();

    (part1, part2)
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_game(s).unwrap().1)
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    all_consuming(map(
        (
            tag("Game "),
            digit1::<&str, _>,
            tag(": "),
            separated_list1(
                tag("; "),
                separated_list1(tag(", "), (digit1, tag(" "), alpha1)),
            ),
        ),
        |(_, id, _, subsets)| Game {
            id: id.parse().unwrap(),
            subsets: subsets
                .into_iter()
                .map(|subset| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for (count, _, color) in subset {
                        let count = count.parse().unwrap();
                        match color {
                            "red" => red = count,
                            "green" => green = count,
                            "blue" => blue = count,
                            _ => unreachable!(),
                        }
                    }
                    GameSubset { red, green, blue }
                })
                .collect(),
        },
    ))
    .parse(input)
}
