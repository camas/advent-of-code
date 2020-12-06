
pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let map = input
        .lines()
        .map(|line| line.chars().map(MapSection::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_x = map[0]
        .iter()
        .position(|s| matches!(s, MapSection::LineDown))
        .unwrap();
    let start_y = 0;

    let mut x = start_x;
    let mut y = start_y;
    let mut chars_found = Vec::new();
    let mut dir = Direction::Down;
    loop {
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Right => x += 1,
            Direction::Left => x -= 1,
        }
        match map[y][x] {
            MapSection::Intersection => match dir {
                Direction::Up | Direction::Down => {
                    if !map[y][x - 1].is_empty() {
                        dir = Direction::Left;
                    } else if !map[y][x + 1].is_empty() {
                        dir = Direction::Right;
                    } else {
                        break;
                    }
                }
                Direction::Left | Direction::Right => {
                    if !map[y - 1][x].is_empty() {
                        dir = Direction::Up;
                    } else if !map[y + 1][x].is_empty() {
                        dir = Direction::Down;
                    } else {
                        break;
                    }
                }
            },
            MapSection::Letter(c) => chars_found.push(c),
            MapSection::LineAcross | MapSection::LineDown => (),
            MapSection::Empty => break,
        }
    }
    let part1 = chars_found.iter().collect::<String>();

    let mut x = start_x;
    let mut y = start_y;
    let mut dir = Direction::Down;
    let mut steps = 1;
    loop {
        steps += 1;
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Right => x += 1,
            Direction::Left => x -= 1,
        }
        match map[y][x] {
            MapSection::Intersection => match dir {
                Direction::Up | Direction::Down => {
                    if !map[y][x - 1].is_empty() {
                        dir = Direction::Left;
                    } else if !map[y][x + 1].is_empty() {
                        dir = Direction::Right;
                    } else {
                        break;
                    }
                }
                Direction::Left | Direction::Right => {
                    if !map[y - 1][x].is_empty() {
                        dir = Direction::Up;
                    } else if !map[y + 1][x].is_empty() {
                        dir = Direction::Down;
                    } else {
                        break;
                    }
                }
            },
            MapSection::LineAcross | MapSection::LineDown | MapSection::Letter(_) => (),
            MapSection::Empty => {
                steps -= 1;
                break;
            }
        }
    }
    let part2 = steps;

    (part1, part2)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum MapSection {
    Empty,
    LineDown,
    LineAcross,
    Intersection,
    Letter(char),
}

impl MapSection {
    fn from_char(c: char) -> Self {
        match c {
            ' ' => MapSection::Empty,
            '|' => MapSection::LineDown,
            '-' => MapSection::LineAcross,
            '+' => MapSection::Intersection,
            'A'..='Z' => MapSection::Letter(c),
            _ => unreachable!(),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, MapSection::Empty)
    }
}
