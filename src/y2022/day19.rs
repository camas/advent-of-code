use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res, opt, recognize},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let blueprints = input
        .trim()
        .lines()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    for blueprint in blueprints.iter() {
        let expected_outputs = [
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geode,
        ];
        let expected_inputs = [
            vec![Resource::Ore],
            vec![Resource::Ore],
            vec![Resource::Ore, Resource::Clay],
            vec![Resource::Ore, Resource::Obsidian],
        ];
        for ((recipie, expected_output), expected_inputs) in blueprint
            .recipies
            .iter()
            .zip(expected_outputs.iter())
            .zip(expected_inputs.iter())
        {
            assert_eq!(&recipie.output, expected_output);
            assert_eq!(recipie.inputs.len(), expected_inputs.len());
            for (input, expected_input) in
                recipie.inputs.iter().map(|(r, _)| r).zip(expected_inputs)
            {
                assert_eq!(input, expected_input);
            }
        }
    }

    let part1 = blueprints
        .iter()
        .map(|b| b.quality_level(24) * b.id)
        .sum::<i64>();

    let part2 = blueprints
        .iter()
        .take(3)
        .map(|b| b.quality_level(32))
        .product::<i64>();

    (part1, part2)
}

#[derive(Debug)]
struct Blueprint {
    id: i64,
    recipies: Vec<Recipie>,
}

#[derive(Debug)]
struct Recipie {
    output: Resource,
    inputs: Vec<(Resource, i64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct State {
    income: [i64; 4],
    resources: [i64; 4],
    time: i64,
}

impl Blueprint {
    fn quality_level(&self, max_time: i64) -> i64 {
        let initial_state = State {
            income: [1, 0, 0, 0],
            resources: [0; 4],
            time: 0,
        };

        let mut max_incomes = [Resource::Ore, Resource::Clay, Resource::Obsidian]
            .into_iter()
            .map(|resource| {
                self.recipies
                    .iter()
                    .filter_map(|r| {
                        r.inputs
                            .iter()
                            .find(|(r, _)| r == &resource)
                            .map(|(_, v)| *v)
                    })
                    .max()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        max_incomes.push(i64::MAX);

        let mut best = i64::MIN;
        let mut queue = vec![initial_state];
        while let Some(state) = queue.pop() {
            // If time up test score
            if state.time == max_time {
                let score = state.resources[Resource::Geode.as_index()];
                if score > best {
                    best = score;
                }
                continue;
            }

            if state.time > max_time {
                continue;
            }

            // If no further changes test score
            let score = state.income[Resource::Geode.as_index()] * (max_time - state.time);
            if score > best {
                best = score;
            }

            queue.extend(state.moves(&self.recipies, &max_incomes));
        }

        best
    }
}

impl State {
    fn moves(&self, recipies: &[Recipie], max_incomes: &[i64]) -> Vec<State> {
        let mut moves = Vec::new();

        for recipie in recipies {
            if self.income[recipie.output.as_index()] >= max_incomes[recipie.output.as_index()] {
                continue;
            }

            // Check income to reach recipie
            if !recipie
                .inputs
                .iter()
                .all(|(resource, _)| self.income[resource.as_index()] > 0)
            {
                continue;
            }

            // Calculate time needed to afford recipie
            let time_taken = recipie
                .inputs
                .iter()
                .map(|(resource, amount)| {
                    (amount - self.resources[resource.as_index()])
                        .div_ceil(self.income[resource.as_index()])
                })
                .max()
                .unwrap()
                .max(0)
                + 1;

            let mut new_state = self.clone();

            new_state.time += time_taken;

            // Update resources
            new_state
                .resources
                .iter_mut()
                .zip(self.income.iter())
                .for_each(|(res, inc)| *res += inc * time_taken);

            // Subtract amounts
            recipie.inputs.iter().for_each(|(resource, amount)| {
                new_state.resources[resource.as_index()] -= amount;
            });

            // Add income
            new_state.income[recipie.output.as_index()] += 1;

            moves.push(new_state);
        }

        moves
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, blueprint) = parse_blueprint(s).unwrap();

        assert!(s.is_empty());

        Ok(blueprint)
    }
}

impl Resource {
    fn as_index(&self) -> usize {
        match self {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        }
    }
}

fn parse_blueprint(s: &str) -> IResult<&str, Blueprint> {
    let (s, id) = preceded(tag("Blueprint "), parse_i64)(s)?;
    let (s, recipies) = preceded(tag(": "), many1(parse_recipie))(s)?;

    Ok((s, Blueprint { id, recipies }))
}

fn parse_recipie(s: &str) -> IResult<&str, Recipie> {
    let (s, output) = preceded(preceded(opt(tag(" ")), tag("Each ")), parse_resource)(s)?;
    let (s, inputs) = delimited(
        tag(" robot costs "),
        separated_list0(
            tag(" and "),
            separated_pair(parse_i64, tag(" "), parse_resource),
        ),
        tag("."),
    )(s)?;

    let inputs = inputs.into_iter().map(|(a, b)| (b, a)).collect();

    Ok((s, Recipie { output, inputs }))
}

fn parse_resource(s: &str) -> IResult<&str, Resource> {
    map(alpha1, |s| match s {
        "ore" => Resource::Ore,
        "clay" => Resource::Clay,
        "obsidian" => Resource::Obsidian,
        "geode" => Resource::Geode,
        _ => unreachable!(),
    })(s)
}

fn parse_i64(s: &str) -> IResult<&str, i64> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), i64::from_str)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 33.to_string());
        assert_eq!(result.1.to_string(), (56 * 62).to_string());
    }
}
