use std::str::FromStr;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
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
    while let Some(curr) = queue.pop() {
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
    let part1 = best;

    struct SearchState2 {
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
            SearchState2 {
                length: 1,
                strength: start.strength(),
                last_port,
                remaining,
            }
        })
        .collect::<Vec<_>>();
    while let Some(curr) = queue.pop() {
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
            queue.push(SearchState2 {
                length: curr.length + 1,
                strength: curr.strength + next.strength(),
                last_port,
                remaining,
            });
        }
    }
    let part2 = best_strength;

    (part1, part2)
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
