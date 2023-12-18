mod math;
mod md5;
mod vector2;
mod vector3;

pub(crate) use math::*;
pub(crate) use md5::*;
pub(crate) use vector2::*;
pub(crate) use vector3::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub(crate) fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub(crate) fn as_vector2(&self) -> Vector2 {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
        }
    }

    pub(crate) fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .cloned()
    }
}

/// Parses the block letters that AoC likes to output
///
/// Could just print it but this lets it fit in a single line
pub fn parse_letters(dots: &[Vec<bool>]) -> String {
    // Special case: last empty row not passed
    let mut dots_len = dots[0].len();
    if dots_len % 5 == 4 {
        dots_len += 1;
    }

    let mut letters = String::new();
    // Each letter is 4 wide followed by a space
    for i in 0..(dots_len / 5) {
        // Hash each letter for easier lookup
        // Each dot just a bit in the hash, so no data lost
        let mut hash = 0_u64;
        for (y, row) in dots.iter().enumerate() {
            let row_part = &row[(i * 5)..(i * 5 + 4)];
            for (x, v) in row_part.iter().enumerate() {
                if *v {
                    hash |= 1 << (y * 4 + x);
                }
            }
        }
        letters.push(match hash {
            1145239 => 'P',
            6885782 => 'C',
            10067865 => 'H',
            10090902 => 'A',
            9795991 => 'R',
            9786201 => 'K',
            15798545 => 'L',
            15323542 => 'G',
            15800095 => 'E',
            1120031 => 'F',
            15803535 => 'Z',
            6920601 => 'U',
            _ => {
                println!(
                    "{}",
                    dots.iter()
                        .map(|line| line
                            .iter()
                            .map(|v| if *v { '#' } else { '.' })
                            .collect::<String>())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                panic!("Unknown hash {}", hash);
            }
        });
    }
    letters
}
