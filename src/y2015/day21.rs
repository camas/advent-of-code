pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();
    let hitpoints = lines
        .next()
        .unwrap()
        .trim_start_matches("Hit Points: ")
        .parse::<i32>()
        .unwrap();
    let damage = lines
        .next()
        .unwrap()
        .trim_start_matches("Damage: ")
        .parse::<i32>()
        .unwrap();
    let armor = lines
        .next()
        .unwrap()
        .trim_start_matches("Armor: ")
        .parse::<i32>()
        .unwrap();
    let boss = Character {
        hitpoints,
        damage,
        armor,
    };

    let mut armors = ARMORS.iter().map(Some).collect::<Vec<_>>();
    armors.push(None);
    let mut ring_combos = Vec::new();
    let mut rings = RINGS.iter().map(Some).collect::<Vec<_>>();
    rings.push(None);
    for (i, ring_a) in rings.iter().enumerate() {
        for (j, ring_b) in rings.iter().enumerate() {
            if i == j {
                continue;
            }
            ring_combos.push((ring_a, ring_b));
        }
    }

    let mut best_win = i32::MAX;
    for weapon in WEAPONS.iter() {
        for armor in armors.iter() {
            for (ring_a, ring_b) in ring_combos.iter() {
                let mut items = vec![weapon];
                if let Some(ring_a) = ring_a {
                    items.push(ring_a);
                }
                if let Some(ring_b) = ring_b {
                    items.push(ring_b);
                }
                if let Some(armor) = armor {
                    items.push(armor);
                }
                let player = Character::from_health_and_items(100, &items);
                if player.fight(&boss) {
                    let total_cost = items.iter().map(|item| item.cost).sum();
                    if total_cost < best_win {
                        best_win = total_cost;
                    }
                }
            }
        }
    }
    let part1 = best_win;

    let mut worst_loss = i32::MIN;
    for weapon in WEAPONS.iter() {
        for armor in armors.iter() {
            for (ring_a, ring_b) in ring_combos.iter() {
                let mut items = vec![weapon];
                if let Some(ring_a) = ring_a {
                    items.push(ring_a);
                }
                if let Some(ring_b) = ring_b {
                    items.push(ring_b);
                }
                if let Some(armor) = armor {
                    items.push(armor);
                }
                let player = Character::from_health_and_items(100, &items);
                if !player.fight(&boss) {
                    let total_cost = items.iter().map(|item| item.cost).sum();
                    if total_cost > worst_loss {
                        worst_loss = total_cost;
                    }
                }
            }
        }
    }
    let part2 = worst_loss;
    (part1, part2)
}

#[derive(Debug)]
struct Character {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    pub fn from_health_and_items(hitpoints: i32, items: &[&Item]) -> Self {
        let (damage, armor) = items.iter().fold((0, 0), |acc, item| {
            (acc.0 + item.damage, acc.1 + item.armor)
        });
        Self {
            hitpoints,
            damage,
            armor,
        }
    }

    pub fn fight(&self, other: &Character) -> bool {
        let mut own_health = self.hitpoints;
        let mut other_health = other.hitpoints;
        loop {
            let damage_done = (self.damage - other.armor).max(1) as i32;
            other_health -= damage_done;
            if other_health <= 0 {
                return true;
            }
            let damage_taken = (other.damage - self.armor).max(1) as i32;
            own_health -= damage_taken;
            if own_health <= 0 {
                return false;
            }
        }
    }
}

#[derive(Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
const ARMORS: [Item; 5] = [
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];
const RINGS: [Item; 6] = [
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        let player = Character {
            hitpoints: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Character {
            hitpoints: 12,
            damage: 7,
            armor: 2,
        };
        assert!(player.fight(&boss));
    }
}
