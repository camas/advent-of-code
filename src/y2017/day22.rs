use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut map = parse_map(input);
    let mut position = Position { x: 0, y: 0 };
    let mut dir = Direction::Up;
    let mut infections = 0_u64;
    for _ in 0..10_000 {
        let curr_infected = map.contains(&position);
        dir = if curr_infected {
            map.remove(&position);
            dir.right_turn()
        } else {
            map.insert(position.clone());
            infections += 1;
            dir.left_turn()
        };
        position = match dir {
            Direction::Up => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
            Direction::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
        };
    }
    let part1 = infections;

    let infected_map = parse_map(input);
    let mut map = infected_map
        .into_iter()
        .map(|pos| (pos, NodeState::Infected))
        .collect::<HashMap<_, _>>();
    let mut dir = Direction::Up;
    let mut position = Position { x: 0, y: 0 };
    let mut infections = 0_u64;
    for _ in 0..10_000_000 {
        let state = map.entry(position.clone()).or_insert(NodeState::Clean);
        dir = match state {
            NodeState::Clean => dir.left_turn(),
            NodeState::Weakened => dir,
            NodeState::Infected => dir.right_turn(),
            NodeState::Flagged => dir.reverse(),
        };
        let new_state = state.next();
        if new_state == NodeState::Infected {
            infections += 1;
        }
        map.insert(position.clone(), new_state);

        position = match dir {
            Direction::Up => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
            Direction::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
        };
    }
    let part2 = infections;

    (part1, part2)
}

fn parse_map(input: &str) -> HashSet<Position> {
    let height = input.lines().count() as i64;
    let width = input.lines().next().unwrap().chars().count() as i64;
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let real_y = y as i64 - height / 2;
            let real_x = x as i64 - width / 2;
            if c == '#' {
                map.insert(Position {
                    y: real_y,
                    x: real_x,
                });
            }
        }
    }
    map
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left_turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right_turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn next(&self) -> NodeState {
        match self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}
