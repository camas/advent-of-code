use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    str::FromStr,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let foods = input
        .lines()
        .map(|l| l.parse::<Food>().unwrap())
        .collect::<Vec<_>>();

    let all_ingredients = foods
        .iter()
        .flat_map(|f| &f.ingredients)
        .collect::<HashSet<_>>();
    let all_allergens = foods
        .iter()
        .flat_map(|f| &f.allergens)
        .collect::<HashSet<_>>();

    let possible_matches = all_allergens
        .iter()
        .map(|allergen| {
            let mut common = foods.iter().filter(|f| f.allergens.contains(allergen));
            let first = common.next().unwrap();
            let common = common.fold(
                HashSet::<&String>::from_iter(&first.ingredients),
                |acc, curr| {
                    acc.intersection(&HashSet::from_iter(&curr.ingredients))
                        .cloned()
                        .collect()
                },
            );
            (*allergen, common)
        })
        .collect::<HashMap<_, _>>();

    let all_possible = possible_matches
        .values()
        .flatten()
        .cloned()
        .collect::<HashSet<_>>();
    let no_allergen_ingredients = all_ingredients
        .difference(&all_possible)
        .collect::<HashSet<_>>();

    let part1 = no_allergen_ingredients
        .iter()
        .map(|&&ingredient| {
            foods
                .iter()
                .filter(|f| f.ingredients.contains(ingredient))
                .count()
        })
        .sum::<usize>();

    let mut known = HashMap::new();
    let mut possible_matches = possible_matches;
    while !possible_matches.is_empty() {
        let allergen = *possible_matches
            .iter()
            .find(|(_, v)| v.len() == 1)
            .unwrap()
            .0;
        let ingredient = possible_matches
            .remove(allergen)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        known.insert(allergen, ingredient);
        possible_matches.values_mut().for_each(|ingredients| {
            ingredients.remove(ingredient);
        });
    }

    let mut known_sorted = known.into_iter().collect::<Vec<_>>();
    known_sorted.sort_by_key(|(allergen, _)| *allergen);

    let part2 = known_sorted
        .into_iter()
        .map(|(_, i)| i.clone())
        .collect::<Vec<_>>()
        .join(",");

    (part1, part2)
}

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let allergens = parts
            .next()
            .unwrap()
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Ok(Self {
            ingredients,
            allergens,
        })
    }
}
