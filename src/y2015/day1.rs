use std::str::FromStr;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let directions: Vec<Direction> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let floor = directions.iter().fold(0, |acc, next| match next {
        Direction::Up => acc + 1,
        Direction::Down => acc - 1,
    });
    let part_1 = floor;

    let mut floor = 0;
    for (i, dir) in directions.iter().enumerate() {
        floor += match dir {
            Direction::Up => 1,
            Direction::Down => -1,
        };
        if floor == -1 {
            let part_2 = i + 1;
            return (part_1, part_2);
        }
    }
    unreachable!()
}

enum Direction {
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Direction::Up),
            ")" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}
