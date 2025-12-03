use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let ingredients = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();

    fn do_loop(
        index: u32,
        current_total: u32,
        mut amounts: Vec<u32>,
        ingredients: &[Ingredient],
        ingredients_len: u32,
    ) -> i32 {
        if index == ingredients_len - 1 {
            let last_amount = 100 - current_total;
            amounts.push(last_amount);
            cookie_value(ingredients, &amounts)
        } else {
            (0..(100 - current_total))
                .map(|amount| {
                    let mut amounts = amounts.clone();
                    amounts.push(amount);
                    do_loop(
                        index + 1,
                        current_total + amount,
                        amounts,
                        ingredients,
                        ingredients_len,
                    )
                })
                .max()
                .unwrap()
        }
    }
    let best = do_loop(0, 0, Vec::new(), &ingredients, ingredients.len() as u32);
    let part1 = best;

    fn do_loop2(
        index: u32,
        current_calories: u32,
        mut amounts: Vec<u32>,
        ingredients: &[Ingredient],
        ingredients_len: u32,
    ) -> i32 {
        let ingredient = &ingredients[index as usize];
        if index == ingredients_len - 1 {
            let last_calories = 500 - current_calories;
            if !last_calories.is_multiple_of(ingredient.calories as u32) {
                i32::MIN
            } else {
                amounts.push(last_calories / ingredient.calories as u32);
                if amounts.iter().sum::<u32>() != 100 {
                    i32::MIN
                } else {
                    cookie_value(ingredients, &amounts)
                }
            }
        } else {
            (0..=(500 - current_calories))
                .step_by(ingredient.calories as usize)
                .map(|calories| {
                    let mut amounts = amounts.clone();
                    amounts.push(calories / ingredient.calories as u32);
                    do_loop2(
                        index + 1,
                        current_calories + calories,
                        amounts,
                        ingredients,
                        ingredients_len,
                    )
                })
                .max()
                .unwrap()
        }
    }
    let best = do_loop2(0, 0, Vec::new(), &ingredients, ingredients.len() as u32);
    let part2 = best;

    (part1, part2)
}

fn cookie_value(ingredients: &[Ingredient], amounts: &[u32]) -> i32 {
    let amounts =
        ingredients
            .iter()
            .zip(amounts.iter())
            .fold((0, 0, 0, 0), |curr, (ing, &amount)| {
                let amount = amount as i32;
                (
                    curr.0 + ing.capacity * amount,
                    curr.1 + ing.durability * amount,
                    curr.2 + ing.flavour * amount,
                    curr.3 + ing.texture * amount,
                )
            });
    if amounts.0 <= 0 || amounts.1 <= 0 || amounts.2 <= 0 || amounts.3 <= 0 {
        return 0;
    }
    amounts.0 * amounts.1 * amounts.2 * amounts.3
}

struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let name = parts[0].trim_end_matches(':').to_string();
        let capacity = parts[2].trim_end_matches(',').parse()?;
        let durability = parts[4].trim_end_matches(',').parse()?;
        let flavour = parts[6].trim_end_matches(',').parse()?;
        let texture = parts[8].trim_end_matches(',').parse()?;
        let calories = parts[10].trim_end_matches(',').parse()?;
        Ok(Self {
            _name: name,
            capacity,
            durability,
            flavour,
            texture,
            calories,
        })
    }
}
