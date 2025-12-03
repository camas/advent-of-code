use std::collections::HashMap;

use num::Integer;
use winnow::{
    ascii::{alphanumeric1, multispace0},
    combinator::{empty, fail, repeat, separated},
    prelude::*,
};
use winnow::{dispatch, token::any};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let (instructions, map) = parse_input(input);

    let mut position = map.root();
    let mut part1 = 0;
    for (moves, instruction) in instructions.iter().cycle().enumerate() {
        if position.is_zzz() {
            part1 = moves;
            break;
        }
        position = position.apply(*instruction);
    }

    let part2 = map
        .all_roots()
        .into_iter()
        .map(|root| {
            let mut position = root;
            let mut iterable = instructions.iter().cycle().enumerate();
            loop {
                let (moves, instruction) = iterable.next().unwrap();
                if position.ends_with_z() {
                    break moves;
                }
                position = position.apply(*instruction);
            }
        })
        .fold(1, |acc, other| acc.lcm(&other));

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    nodes: Vec<Node>,
    zzz_index: usize,
}

#[derive(Debug)]

struct Node {
    name: String,
    index: usize,
    left: usize,
    right: usize,
    ends_with_z: bool,
}

struct MapPosition<'a> {
    map: &'a Map,
    index: usize,
}

impl Map {
    fn new(node_data: Vec<(String, String, String)>) -> Map {
        let mut current_index = 0;
        let mut node_indexes = HashMap::new();

        for (name, _, _) in node_data.iter() {
            node_indexes.entry(name.clone()).or_insert_with(|| {
                let index = current_index;
                current_index += 1;
                index
            });
        }

        let nodes = node_data
            .into_iter()
            .map(|(name, left, right)| Node {
                index: *node_indexes.get(&name).unwrap(),
                ends_with_z: name.ends_with('Z'),
                name,
                left: *node_indexes.get(&left).unwrap(),
                right: *node_indexes.get(&right).unwrap(),
            })
            .collect::<Vec<_>>();

        #[cfg(debug_assertions)]
        for (i, node) in nodes.iter().enumerate() {
            assert_eq!(i, node.index);
        }

        let zzz_index = nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.name == "ZZZ")
            .unwrap()
            .0;

        Map { nodes, zzz_index }
    }

    fn root<'a>(&'a self) -> MapPosition<'a> {
        MapPosition {
            index: self
                .nodes
                .iter()
                .find(|node| node.name == "AAA")
                .unwrap()
                .index,
            map: self,
        }
    }

    fn all_roots<'a>(&'a self) -> Vec<MapPosition<'a>> {
        self.nodes
            .iter()
            .filter(|node| node.name.ends_with('A'))
            .map(|node| MapPosition {
                map: self,
                index: node.index,
            })
            .collect()
    }
}

impl<'a> MapPosition<'a> {
    fn apply(self, instruction: Instruction) -> MapPosition<'a> {
        match instruction {
            Instruction::Left => self.left(),
            Instruction::Right => self.right(),
        }
    }

    fn left(self) -> MapPosition<'a> {
        MapPosition {
            index: self.map.nodes[self.index].left,
            map: self.map,
        }
    }

    fn right(self) -> MapPosition<'a> {
        MapPosition {
            index: self.map.nodes[self.index].right,
            map: self.map,
        }
    }

    fn is_zzz(&self) -> bool {
        self.map.zzz_index == self.index
    }

    fn ends_with_z(&self) -> bool {
        self.map.nodes[self.index].ends_with_z
    }
}

fn parse_input(input: &str) -> (Vec<Instruction>, Map) {
    (
        parse_instructions,
        "\n\n",
        separated(1.., parse_node, "\n"),
        multispace0,
    )
        .map(|(instructions, _, nodes, _)| (instructions, Map::new(nodes)))
        .parse(input)
        .unwrap()
}

fn parse_instructions(input: &mut &str) -> winnow::Result<Vec<Instruction>> {
    repeat(
        1..,
        dispatch! {any;
            'R' => empty.value(Instruction::Right),
            'L' => empty.value(Instruction::Left),
            _ => fail,
        },
    )
    .parse_next(input)
}

fn parse_node(input: &mut &str) -> winnow::Result<(String, String, String)> {
    (
        alphanumeric1::<&str, _>,
        " = (",
        alphanumeric1,
        ", ",
        alphanumeric1,
        ")",
    )
        .map(|(name, _, left, _, right, _)| (name.to_string(), left.to_string(), right.to_string()))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let (part1, _) = solve(input);

        assert_eq!(part1.to_string(), "6");

        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
ZZZ = (ZZZ, ZZZ)
AAA = (ZZZ, ZZZ)
";

        let (_, part2) = solve(input);

        assert_eq!(part2.to_string(), "6");
    }
}
