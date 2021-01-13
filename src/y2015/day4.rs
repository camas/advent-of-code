use crate::common::md5_string;
use rayon::prelude::*;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let input = input.trim();

    let part1 = (1..i32::MAX)
        .into_par_iter()
        .find_first(|i| {
            let key = format!("{}{}", input, i);
            let hash = md5_string(&key);
            &hash[0..5] == "00000"
        })
        .unwrap();

    let part2 = (1..i32::MAX)
        .into_par_iter()
        .find_first(|i| {
            let key = format!("{}{}", input, i);
            let hash = md5_string(&key);
            &hash[0..6] == "000000"
        })
        .unwrap();

    (part1, part2)
}
