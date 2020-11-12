use std::str::FromStr;

use crate::Exercise;

pub struct Day24;

impl Exercise for Day24 {
    fn part1(&self, input: &str) -> String {
        let components = input
            .lines()
            .map(|line| line.parse::<Component>().unwrap())
            .collect::<Vec<_>>();

        struct SearchState {
            current_strength: u32,
            last_port: u32,
            remaining: Vec<Component>,
        }

        let mut best = 0;
        let mut queue = components
            .iter()
            .filter(|c| c.a == 0 || c.b == 0)
            .map(|start| {
                let last_port = if start.a == 0 { start.b } else { start.a };
                let mut remaining = components.clone();
                remaining.remove(remaining.iter().position(|other| other == start).unwrap());
                SearchState {
                    current_strength: start.strength(),
                    last_port,
                    remaining,
                }
            })
            .collect::<Vec<_>>();
        while !queue.is_empty() {
            let curr = queue.pop().unwrap();
            if curr.current_strength > best {
                best = curr.current_strength;
            }
            let matches = curr
                .remaining
                .iter()
                .filter(|c| c.a == curr.last_port || c.b == curr.last_port);
            for next in matches {
                let last_port = if next.a == curr.last_port {
                    next.b
                } else {
                    next.a
                };
                let mut remaining = curr.remaining.clone();
                remaining.remove(remaining.iter().position(|c| c == next).unwrap());
                queue.push(SearchState {
                    current_strength: curr.current_strength + next.strength(),
                    last_port,
                    remaining,
                });
            }
        }
        best.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let components = input
            .lines()
            .map(|line| line.parse::<Component>().unwrap())
            .collect::<Vec<_>>();

        struct SearchState {
            strength: u32,
            length: u32,
            last_port: u32,
            remaining: Vec<Component>,
        }

        let mut best_length = 0;
        let mut best_strength = 0;
        let mut queue = components
            .iter()
            .filter(|c| c.a == 0 || c.b == 0)
            .map(|start| {
                let last_port = if start.a == 0 { start.b } else { start.a };
                let mut remaining = components.clone();
                remaining.remove(remaining.iter().position(|other| other == start).unwrap());
                SearchState {
                    length: 1,
                    strength: start.strength(),
                    last_port,
                    remaining,
                }
            })
            .collect::<Vec<_>>();
        while !queue.is_empty() {
            let curr = queue.pop().unwrap();
            if curr.length >= best_length && curr.strength > best_strength {
                best_length = curr.length;
                best_strength = curr.strength;
            }
            let matches = curr
                .remaining
                .iter()
                .filter(|c| c.a == curr.last_port || c.b == curr.last_port);
            for next in matches {
                let last_port = if next.a == curr.last_port {
                    next.b
                } else {
                    next.a
                };
                let mut remaining = curr.remaining.clone();
                remaining.remove(remaining.iter().position(|c| c == next).unwrap());
                queue.push(SearchState {
                    length: curr.length + 1,
                    strength: curr.strength + next.strength(),
                    last_port,
                    remaining,
                });
            }
        }
        best_strength.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Component {
    a: u32,
    b: u32,
}

impl Component {
    fn strength(&self) -> u32 {
        self.a + self.b
    }
}

impl FromStr for Component {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/').collect::<Vec<_>>();
        let a = parts[0].parse()?;
        let b = parts[1].parse()?;
        Ok(Component { a, b })
    }
}
