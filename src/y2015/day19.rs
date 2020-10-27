use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::Exercise;

pub struct Day19;

impl Exercise for Day19 {
    fn part1(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();
        let mut actions = HashMap::new();
        for line in &lines[..lines.len() - 2] {
            let mut parts = line.split(" => ");
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            actions.entry(from).or_insert_with(Vec::new).push(to);
        }
        let molecule = lines.last().unwrap();
        let mut new_molecules = HashSet::new();
        new_molecules.extend(get_new(molecule, &actions));
        new_molecules.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();
        let mut actions = HashMap::new();
        for line in &lines[..lines.len() - 2] {
            let mut parts = line.split(" => ");
            let from = parts.next().unwrap().as_bytes();
            let to = parts.next().unwrap().as_bytes();
            actions.entry(from).or_insert_with(Vec::new).push(to);
        }
        let target_molecule = lines.last().unwrap();
        let decoded = target_molecule
            .replace("Rn", "(")
            .replace("Ar", ")")
            .replace("Y", ",");
        let total_elements = decoded
            .chars()
            .filter(|&c| (c >= 'A' && c <= 'Z') || c == '(' || c == ')' || c == ',')
            .count();
        let bracket_count = decoded.chars().filter(|&c| c == '(' || c == ')').count();
        let comma_count = decoded.chars().filter(|&c| c == ',').count();
        println!("{:?} {} {}", total_elements, bracket_count, comma_count);
        (total_elements - 1 - bracket_count - (2 * comma_count)).to_string()
    }
}

fn get_new(molecule: &str, actions: &HashMap<&str, Vec<&str>>) -> Vec<String> {
    let mut new_molecules = Vec::new();
    for (key, replacements) in actions.iter() {
        let mut indexes = Vec::new();
        let mut offset = 0;
        let mut current = molecule;
        while let Some(index) = current.find(key) {
            indexes.push(offset + index);
            offset += index + 1;
            current = &current[index + 1..];
        }
        for index in indexes {
            for replacement in replacements {
                let new_value = format!(
                    "{}{}{}",
                    &molecule[..index],
                    replacement,
                    &molecule[index + key.len()..]
                );
                // println!(
                //     "{} {} {} {} {}",
                //     molecule, new_value, index, key, replacement
                // );
                new_molecules.push(new_value);
            }
        }
    }
    new_molecules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "H => HO
H => OH
O => HH

HOH";

        assert_eq!(Day19 {}.part1(input), "4");
    }
}
