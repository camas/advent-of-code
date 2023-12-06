use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map},
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult,
};
use num::integer::Roots;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let races = parse_races(input).unwrap().1;

    let part1 = races
        .iter()
        .map(|race| race.count_ways_to_win())
        .product::<i64>();

    let race = parse_kerning_issue_race(input).unwrap().1;
    let part2 = race.count_ways_to_win();

    (part1, part2)
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn count_ways_to_win(&self) -> i64 {
        // h (time held), s (speed), d (distance), t (race time)
        // s = h
        // to win = d + 1
        // d + 1 = (t - h) * s
        // d + 1 = t * s - s**2
        // s**2 - t*s + d + 1 = 0
        let (a, b) = solve_quadratic(1, -self.time, self.distance + 1);
        1 + a.max(b) - a.min(b).max(0)
    }
}

fn solve_quadratic(a: i64, b: i64, c: i64) -> (i64, i64) {
    let square_root_part = (b.pow(2) - (4 * a * c)).sqrt();
    (
        ((-b + square_root_part).div_floor(2 * a)),
        ((-b - square_root_part).div_ceil(2 * a)),
    )
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    all_consuming(map(
        tuple((
            tag("Time:"),
            many1(tag(" ")),
            parse_numbers,
            tag("\nDistance:"),
            many1(tag(" ")),
            parse_numbers,
            many0(tag("\n")),
        )),
        |(_, _, times, _, _, distances, _)| {
            times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect::<Vec<_>>()
        },
    ))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    map(
        separated_list1(many1(tag(" ")), digit1::<&str, _>),
        |numbers| {
            numbers
                .into_iter()
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
        },
    )(input)
}

fn parse_kerning_issue_race(input: &str) -> IResult<&str, Race> {
    all_consuming(map(
        tuple((
            tag("Time:"),
            many0(tag(" ")),
            separated_list1(many1(tag(" ")), digit1::<&str, _>),
            tag("\nDistance:"),
            many0(tag(" ")),
            separated_list1(many1(tag(" ")), digit1),
            many0(tag("\n")),
        )),
        |(_, _, time, _, _, distance, _)| Race {
            time: time.concat().parse::<i64>().unwrap(),
            distance: distance.concat().parse::<i64>().unwrap(),
        },
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "288");
        assert_eq!(part2.to_string(), "71503");
    }
}
