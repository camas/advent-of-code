use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut nodes = input
        .lines()
        .skip(2)
        .map(|l| {
            let x = l[16..18].trim_end_matches('-').parse::<u64>().unwrap();
            let y = l[19..22]
                .trim_start_matches('y')
                .trim_end_matches(' ')
                .parse::<u64>()
                .unwrap();
            let size = l[24..27].trim_start_matches(' ').parse::<u64>().unwrap();
            let used = l[30..33].trim_start_matches(' ').parse::<u64>().unwrap();
            let available = l[37..40].trim_start_matches(' ').parse::<u64>().unwrap();
            assert_eq!(used + available, size);
            ((y, x), Node { size, used })
        })
        .collect::<HashMap<_, _>>();

    // Convert to grid
    let min_x = *nodes.keys().map(|(_, x)| x).min().unwrap();
    let max_x = *nodes.keys().map(|(_, x)| x).max().unwrap();
    let min_y = *nodes.keys().map(|(y, _)| y).min().unwrap();
    let max_y = *nodes.keys().map(|(y, _)| y).max().unwrap();
    let node_grid = (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| nodes.remove(&(y, x)).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(min_x, 0);
    assert_eq!(min_y, 0);
    assert!(nodes.is_empty());

    // Part 1: Count viable nodes
    let mut count = 0;
    for (y, x, node) in node_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, node)| (y, x, node)))
    {
        for (other_y, other_x, other_node) in node_grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, node)| (y, x, node)))
        {
            if x == other_x && y == other_y {
                continue;
            }
            if node.used > 0 && other_node.available() >= node.used {
                count += 1;
            }
        }
    }
    let part1 = count;

    // Part 2: Another move search thingy
    let empty = node_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, node)| {
                    if node.used == 0 {
                        Some((y, x))
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(empty.len(), 1);
    // true for large node, false otherwise
    let map = node_grid
        .iter()
        .map(|row| row.iter().map(|node| node.used > 200).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let initial_state = State {
        moves: 0,
        empty_pos: (empty[0].0 as u64, empty[0].1 as u64),
        data_pos: (0, node_grid[0].len() as u64 - 1),
    };
    let mut queue = vec![initial_state];
    let mut seen = HashSet::new();
    let part2 = loop {
        let state = queue.pop().unwrap();
        if state.is_final() {
            break state.moves;
        }

        macro_rules! do_dir {
            ($y:expr, $x:expr) => {
                let x = $x;
                let y = $y;
                if !map[y as usize][x as usize] {
                    let new = if state.data_pos == (y, x) {
                        State {
                            moves: state.moves + 1,
                            empty_pos: (y, x),
                            data_pos: state.empty_pos,
                        }
                    } else {
                        State {
                            moves: state.moves + 1,
                            empty_pos: (y, x),
                            data_pos: state.data_pos,
                        }
                    };
                    if seen.insert((new.empty_pos, new.data_pos)) {
                        let pos = queue
                            .binary_search_by(|s| {
                                (s.moves + s.h()).cmp(&(new.moves + new.h())).reverse()
                            })
                            .unwrap_or_else(|e| e);
                        queue.insert(pos, new);
                    }
                }
            };
        }

        if state.empty_pos.0 > 0 {
            do_dir!(state.empty_pos.0 - 1, state.empty_pos.1);
        }
        if state.empty_pos.0 < map.len() as u64 - 1 {
            do_dir!(state.empty_pos.0 + 1, state.empty_pos.1);
        }
        if state.empty_pos.1 > 0 {
            do_dir!(state.empty_pos.0, state.empty_pos.1 - 1);
        }
        if state.empty_pos.1 < map[0].len() as u64 - 1 {
            do_dir!(state.empty_pos.0, state.empty_pos.1 + 1);
        }
    };

    (part1, part2)
}

#[derive(Debug)]
struct State {
    moves: u64,
    empty_pos: (u64, u64),
    data_pos: (u64, u64),
}

impl State {
    fn is_final(&self) -> bool {
        self.data_pos == (0, 0)
    }

    fn h(&self) -> u64 {
        let dist_from_start = self.data_pos.0 + self.data_pos.1;
        let dist_from_empty = self
            .data_pos
            .0
            .checked_sub(self.empty_pos.0)
            .unwrap_or(self.empty_pos.0 - self.data_pos.0)
            + self
                .data_pos
                .1
                .checked_sub(self.empty_pos.1)
                .unwrap_or(self.empty_pos.1 - self.data_pos.1);
        dist_from_start << 32 | dist_from_empty
    }
}

#[derive(Debug)]
struct Node {
    size: u64,
    used: u64,
}

impl Node {
    fn available(&self) -> u64 {
        self.size - self.used
    }
}
