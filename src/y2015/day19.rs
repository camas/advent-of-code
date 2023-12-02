use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
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
    let part1 = new_molecules.len();

    let target_molecule = lines.last().unwrap();
    let decoded = target_molecule
        .replace("Rn", "(")
        .replace("Ar", ")")
        .replace('Y', ",");
    let total_elements = decoded
        .chars()
        .filter(|&c| c.is_ascii_uppercase() || c == '(' || c == ')' || c == ',')
        .count();
    let bracket_count = decoded.chars().filter(|&c| c == '(' || c == ')').count();
    let comma_count = decoded.chars().filter(|&c| c == ',').count();
    // println!("{:?} {} {}", total_elements, bracket_count, comma_count);
    let part2 = total_elements - 1 - bracket_count - (2 * comma_count);

    (part1, part2)
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
