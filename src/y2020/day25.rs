pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let nums = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut curr = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        curr = mul_mod64(curr, 7, 20201227);
        if curr == nums[0] {
            break;
        }
    }
    let part1 = pow_mod64(nums[1], loop_size, 20201227);

    (part1, "")
}

fn pow_mod64(x: u64, y: u64, m: u64) -> u64 {
    (1..y).fold(x, |acc, _| mul_mod64(acc, x, m))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod64(2, 4, 10), 6);
    }
}
