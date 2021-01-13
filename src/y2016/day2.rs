pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let moves = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'U' => Move::Up,
                    'R' => Move::Right,
                    'D' => Move::Down,
                    'L' => Move::Left,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = moves
        .iter()
        .map(|m| {
            let mut key = 5;
            for dir in m.iter() {
                match (key, dir) {
                    (1, Move::Right) => key = 2,
                    (1, Move::Down) => key = 4,
                    (2, Move::Right) => key = 3,
                    (2, Move::Down) => key = 5,
                    (2, Move::Left) => key = 1,
                    (3, Move::Down) => key = 6,
                    (3, Move::Left) => key = 2,
                    (4, Move::Up) => key = 1,
                    (4, Move::Right) => key = 5,
                    (4, Move::Down) => key = 7,
                    (5, Move::Up) => key = 2,
                    (5, Move::Right) => key = 6,
                    (5, Move::Down) => key = 8,
                    (5, Move::Left) => key = 4,
                    (6, Move::Up) => key = 3,
                    (6, Move::Down) => key = 9,
                    (6, Move::Left) => key = 5,
                    (7, Move::Up) => key = 4,
                    (7, Move::Right) => key = 8,
                    (8, Move::Up) => key = 5,
                    (8, Move::Right) => key = 9,
                    (8, Move::Left) => key = 7,
                    (9, Move::Up) => key = 6,
                    (9, Move::Left) => key = 8,
                    _ => {}
                }
            }
            key
        })
        .map(|v| v.to_string())
        .collect::<String>();

    let part2 = moves
        .iter()
        .map(|m| {
            let mut key = 5;
            for dir in m.iter() {
                match (key, dir) {
                    (1, Move::Down) => key = 3,
                    (2, Move::Right) => key = 3,
                    (2, Move::Down) => key = 6,
                    (3, Move::Up) => key = 1,
                    (3, Move::Right) => key = 4,
                    (3, Move::Down) => key = 7,
                    (3, Move::Left) => key = 2,
                    (4, Move::Down) => key = 8,
                    (4, Move::Left) => key = 3,
                    (5, Move::Right) => key = 6,
                    (6, Move::Up) => key = 2,
                    (6, Move::Right) => key = 7,
                    (6, Move::Down) => key = 0xa,
                    (6, Move::Left) => key = 5,
                    (7, Move::Up) => key = 3,
                    (7, Move::Right) => key = 8,
                    (7, Move::Down) => key = 0xb,
                    (7, Move::Left) => key = 6,
                    (8, Move::Up) => key = 4,
                    (8, Move::Right) => key = 9,
                    (8, Move::Down) => key = 0xc,
                    (8, Move::Left) => key = 7,
                    (9, Move::Left) => key = 8,
                    (0xa, Move::Up) => key = 6,
                    (0xa, Move::Right) => key = 0xb,
                    (0xb, Move::Up) => key = 7,
                    (0xb, Move::Right) => key = 0xc,
                    (0xb, Move::Down) => key = 0xd,
                    (0xb, Move::Left) => key = 0xa,
                    (0xc, Move::Up) => key = 8,
                    (0xc, Move::Left) => key = 0xb,
                    (0xd, Move::Up) => key = 0xb,
                    _ => {}
                }
            }
            key
        })
        .map(|v| format!("{:X}", v))
        .collect::<String>();

    (part1, part2)
}

enum Move {
    Up,
    Right,
    Down,
    Left,
}
