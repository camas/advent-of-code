mod math;
mod md5;
mod vector;

pub use math::*;
pub use md5::*;
pub use vector::*;

pub fn parse_letters(dots: &[Vec<bool>]) -> String {
    let mut letters = String::new();
    // Special case: last empty row not passed
    let mut dots_len = dots[0].len();
    if dots_len % 5 == 4 {
        dots_len += 1;
    }
    // Each letter is 4 wide followed by an empty line
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
