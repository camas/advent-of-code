use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut passports = Vec::new();
    let mut current = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(current);
            current = HashMap::new();
            continue;
        }
        for parts in line.split(' ') {
            let mut inner_parts = parts.split(':');
            current.insert(inner_parts.next().unwrap(), inner_parts.next().unwrap());
        }
    }
    if !current.is_empty() {
        passports.push(current);
    }

    let mut valid_count = 0;
    let mut valid_2_count = 0;
    for passport in passports.iter() {
        let expected = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let valid = expected.iter().all(|k| passport.contains_key(k));
        if valid {
            valid_count += 1;
        } else {
            continue;
        }

        let byr = passport["byr"].parse::<u32>().unwrap();
        if !(1920..=2002).contains(&byr) {
            continue;
        }
        let iyr = passport["iyr"].parse::<u32>().unwrap();
        if !(2010..=2020).contains(&iyr) {
            continue;
        }
        let eyr = passport["eyr"].parse::<u32>().unwrap();
        if !(2020..=2030).contains(&eyr) {
            continue;
        }
        let height = passport["hgt"];
        let is_cm = &height[height.len() - 2..] == "cm";
        let is_in = &height[height.len() - 2..] == "in";
        if !is_cm && !is_in {
            continue;
        }
        let height = height[0..height.len() - 2].parse::<u32>().unwrap();
        if (is_cm && !(150..=193).contains(&height)) || (is_in && !(59..=76).contains(&height)) {
            continue;
        }
        let hair = passport["hcl"];
        if !hair.starts_with('#')
            || !hair[1..]
                .chars()
                .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
        {
            continue;
        }
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"]) {
            continue;
        }
        let id = passport["pid"];
        if id.chars().count() != 9 || id.chars().any(|c| !('0'..='9').contains(&c)) {
            continue;
        }

        valid_2_count += 1;
    }

    (valid_count, valid_2_count)
}
