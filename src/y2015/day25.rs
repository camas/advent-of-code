use crate::Exercise;

pub struct Day25;

impl Exercise for Day25 {
    fn part1(&self, input: &str) -> String {
        let input = input.trim().trim_start_matches(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        );
        let parts = input.split(' ').collect::<Vec<_>>();
        let target_row = parts[0].trim_end_matches(',').parse::<u64>().unwrap();
        let target_column = parts[2].trim_end_matches('.').parse::<u64>().unwrap();
        const BASE_VALUE: u64 = 20151125;
        let real_index = get_real_index(target_row, target_column);
        get_code(BASE_VALUE, real_index).to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Finished!".to_string()
    }
}

fn get_real_index(row: u64, column: u64) -> u64 {
    let a = ((column + 1) * column) / 2;
    let b = ((column + column + row - 2) * (row - 1)) / 2;
    a + b
}

// Indexes start at 1
fn get_code(base_value: u64, index: u64) -> u64 {
    // Could be made quicker
    let mut value = base_value;
    for _ in 0..(index - 1) {
        value = mul_mod64(value, 252533, 33554393);
    }
    value
}

// https://stackoverflow.com/a/45924957
fn mul_mod64(mut x: u64, mut y: u64, m: u64) -> u64 {
    let msb = 0x8000_0000_0000_0000;
    let mut d = 0;
    let mp2 = m >> 1;
    x %= m;
    y %= m;

    if m & msb == 0 {
        for _ in 0..64 {
            d = if d > mp2 { (d << 1) - m } else { d << 1 };
            if x & msb != 0 {
                d += y;
            }
            if d >= m {
                d -= m;
            }
            x <<= 1;
        }
        d
    } else {
        for _ in 0..64 {
            d = if d > mp2 {
                d.wrapping_shl(1).wrapping_sub(m)
            } else {
                // the case d == m && x == 0 is taken care of
                // after the end of the loop
                d << 1
            };
            if x & msb != 0 {
                let (mut d1, overflow) = d.overflowing_add(y);
                if overflow {
                    d1 = d1.wrapping_sub(m);
                }
                d = if d1 >= m { d1 - m } else { d1 };
            }
            x <<= 1;
        }
        if d >= m {
            d - m
        } else {
            d
        }
    }
}
