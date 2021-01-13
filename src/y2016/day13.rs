use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let fav_number = input.trim().parse::<u64>().unwrap();

    let part1 = a_star(31, 39, fav_number);
    let part2 = breadth(50, fav_number);

    (part1, part2)
}

fn a_star(target_x: u64, target_y: u64, fav_number: u64) -> u64 {
    let initial_state = State {
        moves: 0,
        x: 1,
        y: 1,
    };
    let mut queue = vec![initial_state];
    let mut seen = HashSet::new();
    loop {
        let state = queue.pop().unwrap();
        if state.x == target_x && state.y == target_y {
            return state.moves;
        }

        macro_rules! insert_new {
            ($state:expr) => {
                let new = $state;
                if !new.is_wall(fav_number) && seen.insert((new.x, new.y)) {
                    let pos = queue
                        .binary_search_by(|s| {
                            (s.moves + s.h(target_x, target_y))
                                .cmp(&(new.moves + new.h(target_x, target_y)))
                                .reverse()
                        })
                        .unwrap_or_else(|e| e);
                    queue.insert(pos, new);
                }
            };
        }

        insert_new!(State {
            moves: state.moves + 1,
            x: state.x + 1,
            y: state.y
        });
        if state.x > 0 {
            insert_new!(State {
                moves: state.moves + 1,
                x: state.x - 1,
                y: state.y
            });
        }
        insert_new!(State {
            moves: state.moves + 1,
            x: state.x,
            y: state.y + 1,
        });
        if state.y > 0 {
            insert_new!(State {
                moves: state.moves + 1,
                x: state.x,
                y: state.y - 1,
            });
        }
    }
}

fn breadth(move_limit: u64, fav_number: u64) -> usize {
    let initial_state = State {
        moves: 0,
        x: 1,
        y: 1,
    };
    let mut queue = VecDeque::new();
    queue.push_front(initial_state);
    let mut seen = HashSet::new();
    while let Some(state) = queue.pop_front() {
        macro_rules! insert_new {
            ($state:expr) => {
                let new = $state;
                if new.moves <= move_limit
                    && !new.is_wall(fav_number)
                    && seen.insert((new.x, new.y))
                {
                    queue.push_back(new);
                }
            };
        }

        insert_new!(State {
            moves: state.moves + 1,
            x: state.x + 1,
            y: state.y
        });
        if state.x > 0 {
            insert_new!(State {
                moves: state.moves + 1,
                x: state.x - 1,
                y: state.y
            });
        }
        insert_new!(State {
            moves: state.moves + 1,
            x: state.x,
            y: state.y + 1,
        });
        if state.y > 0 {
            insert_new!(State {
                moves: state.moves + 1,
                x: state.x,
                y: state.y - 1,
            });
        }
    }
    seen.len()
}

#[derive(Debug)]
struct State {
    moves: u64,
    x: u64,
    y: u64,
}

impl State {
    fn h(&self, target_x: u64, target_y: u64) -> u64 {
        ((self.x as i64 - target_x as i64).abs() + (self.y as i64 - target_y as i64).abs()) as u64
    }

    fn is_wall(&self, fav_number: u64) -> bool {
        let num = self.x * self.x
            + 3 * self.x
            + 2 * self.x * self.y
            + self.y
            + self.y * self.y
            + fav_number;
        num.count_ones() % 2 == 1
    }
}
