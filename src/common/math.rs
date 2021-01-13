use std::iter::Product;

use num::Integer;

pub fn chinese_remainder<T: Integer + Copy + Product>(a: &[T], m: &[T]) -> Option<T> {
    let big_n = m.iter().copied().product::<T>();
    a.iter()
        .zip(m.iter())
        .try_fold(T::zero(), |acc, (&a_i, &m_i)| {
            let p = big_n / m_i;
            Some(acc + a_i * mod_inv(p, m_i)? * p)
        })
        .map(|r| (r % big_n + big_n) % big_n)
}

#[allow(clippy::many_single_char_names)]
pub fn egcd<T: Integer + Copy>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inv<T: Integer + Copy>(a: T, m: T) -> Option<T> {
    let (g, x, _) = egcd(a, m);
    if g == T::one() {
        Some((x % m + m) % m)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese() {
        assert_eq!(chinese_remainder(&[2, 3, 2], &[3, 5, 7]), Some(23))
    }
}
