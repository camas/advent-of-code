use std::collections::HashMap;

use crate::common::md5_string;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let salt = input.trim();

    let part1 = work(salt, false);
    let part2 = work(salt, true);

    (part1, part2)
}

fn work(salt: &str, part2: bool) -> u32 {
    let mut triple_indexes: HashMap<char, Vec<u32>> = HashMap::new();
    let mut key_count = 0;
    for i in 0.. {
        let hash = if part2 {
            (0..2016).fold(md5_string(&format!("{}{}", salt, i)), |acc, _| {
                md5_string(&acc)
            })
        } else {
            md5_string(&format!("{}{}", salt, i))
        };
        let chars = hash.chars().collect::<Vec<_>>();
        for to_check in chars.windows(5) {
            if to_check.iter().skip(1).all(|&other| other == to_check[0]) {
                if let Some(other_indexes) = triple_indexes.get_mut(&to_check[0]) {
                    let matching = other_indexes
                        .iter()
                        .enumerate()
                        .filter(|(_, other)| i - **other <= 1000)
                        .collect::<Vec<_>>();
                    for (_, index) in matching.iter() {
                        key_count += 1;
                        if key_count == 64 {
                            return **index;
                        }
                    }
                    let matching = matching.into_iter().map(|(i, _)| i).collect::<Vec<_>>();
                    for i in matching.into_iter().rev() {
                        other_indexes.swap_remove(i);
                    }
                }
            }
        }
        for to_check in chars.windows(3) {
            if to_check[0] == to_check[1] && to_check[0] == to_check[2] {
                triple_indexes.entry(to_check[0]).or_default().push(i);
                break;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let result = solve("abc");
        assert_eq!(result.0.to_string(), "22728");
    }
}
