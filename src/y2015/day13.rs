use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut rels = HashMap::new();
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let person_a = parts[0];
        let is_neg = parts[2] == "lose";
        let value = parts[3].parse::<i32>().unwrap();
        let person_b = parts[10].trim_end_matches('.');
        let value = if is_neg { -value } else { value };
        rels.entry(person_a)
            .or_insert_with(HashMap::new)
            .insert(person_b, value);
    }

    let mut best = i32::MIN;
    let names = rels.keys().collect::<Vec<_>>();
    let names_len = names.len();
    for comb in names.into_iter().permutations(names_len) {
        let mut last_person = comb[0];
        let end_person = comb[comb.len() - 1];
        //println!("{} {}", end_person, last_person);
        let mut happiness = rels[end_person][last_person] + rels[last_person][end_person];
        for &person in &comb[1..] {
            //println!("{} {}", person, last_person);
            happiness += rels[person][last_person] + rels[last_person][person];
            last_person = person;
        }
        if happiness > best {
            best = happiness;
        }
    }
    let part1 = best;

    let mut rels = HashMap::new();
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let person_a = parts[0];
        let is_neg = parts[2] == "lose";
        let value = parts[3].parse::<i32>().unwrap();
        let person_b = parts[10].trim_end_matches('.');
        let value = if is_neg { -value } else { value };
        rels.entry(person_a)
            .or_insert_with(HashMap::new)
            .insert(person_b, value);
    }

    // Add self
    rels.insert("me", HashMap::new());
    let others = rels
        .keys()
        .filter(|&name| name != &"me")
        .cloned()
        .collect::<Vec<_>>();
    for name in others {
        rels.get_mut("me").unwrap().insert(name, 0);
        rels.get_mut(name).unwrap().insert("me", 0);
    }

    let mut best = i32::MIN;
    let names = rels.keys().collect::<Vec<_>>();
    let names_len = names.len();
    for comb in names.into_iter().permutations(names_len) {
        let mut last_person = comb[0];
        let end_person = comb[comb.len() - 1];
        //println!("{} {}", end_person, last_person);
        let mut happiness = rels[end_person][last_person] + rels[last_person][end_person];
        for &person in &comb[1..] {
            //println!("{} {}", person, last_person);
            happiness += rels[person][last_person] + rels[last_person][person];
            last_person = person;
        }
        if happiness > best {
            best = happiness;
        }
    }
    let part2 = best;

    (part1, part2)
}
