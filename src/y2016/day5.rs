use std::collections::HashMap;

use crate::common::md5_string;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let door_id = input.trim();

    let mut part1 = Vec::new();
    let mut part2 = HashMap::new();
    for i in 0.. {
        let test = format!("{}{}", door_id, i);
        let hash = md5_string(&test);
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap();
            let value = hash.chars().nth(6).unwrap();
            if part1.len() < 8 {
                part1.push(pos);
            }
            if !('0'..='7').contains(&pos) {
                continue;
            }
            let pos = pos as u8 - b'0';
            #[allow(clippy::map_entry)]
            if !part2.contains_key(&pos) {
                part2.insert(pos, value);
                if part2.len() == 8 {
                    break;
                }
            }
        }
    }

    let part1 = part1.iter().collect::<String>();
    let part2 = (0..8).map(|i| part2[&i]).collect::<String>();

    (part1, part2)
}
