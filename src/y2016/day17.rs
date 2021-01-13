use crate::common::md5_string;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let passcode = input.trim();

    let mut part1 = None;
    let mut part2 = 0;
    let initial_state = State {
        moves: Vec::new(),
        x: 0,
        y: 0,
    };
    let mut queue = vec![initial_state];
    while let Some(state) = queue.pop() {
        if state.reached_vault() {
            part2 = state.moves.len();
            if part1.is_none() {
                part1 = Some(state.moves.iter().map(|m| m.char()).collect::<String>())
            }
            continue;
        }

        let mut to_hash = passcode.to_string();
        to_hash.extend(state.moves.iter().map(|d| d.char()));
        let hash = md5_string(&to_hash);
        let mut hash_chars = hash.chars();

        let up_open = ('b'..='f').contains(&hash_chars.next().unwrap());
        let down_open = ('b'..='f').contains(&hash_chars.next().unwrap());
        let left_open = ('b'..='f').contains(&hash_chars.next().unwrap());
        let right_open = ('b'..='f').contains(&hash_chars.next().unwrap());

        macro_rules! new_state {
            ($state:expr) => {
                let new = $state;
                let pos = queue
                    .binary_search_by(|s| {
                        (s.moves.len() + s.h())
                            .cmp(&(new.moves.len() + new.h()))
                            .reverse()
                    })
                    .unwrap_or_else(|e| e);
                queue.insert(pos, new);
            };
        }

        if up_open && state.y > 0 {
            let mut new_moves = state.moves.clone();
            new_moves.push(Direction::Up);
            new_state!(State {
                moves: new_moves,
                x: state.x,
                y: state.y - 1,
            });
        }
        if right_open && state.x < 3 {
            let mut new_moves = state.moves.clone();
            new_moves.push(Direction::Right);
            new_state!(State {
                moves: new_moves,
                x: state.x + 1,
                y: state.y,
            });
        }
        if down_open && state.y < 3 {
            let mut new_moves = state.moves.clone();
            new_moves.push(Direction::Down);
            new_state!(State {
                moves: new_moves,
                x: state.x,
                y: state.y + 1,
            });
        }
        if left_open && state.x > 0 {
            let mut new_moves = state.moves.clone();
            new_moves.push(Direction::Left);
            new_state!(State {
                moves: new_moves,
                x: state.x - 1,
                y: state.y
            });
        }
    }

    (part1.unwrap(), part2)
}

#[derive(Debug)]
struct State {
    moves: Vec<Direction>,
    x: usize,
    y: usize,
}

impl State {
    fn h(&self) -> usize {
        3 - self.x + 3 - self.y
    }

    fn reached_vault(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn char(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
            Direction::Left => 'L',
        }
    }
}
