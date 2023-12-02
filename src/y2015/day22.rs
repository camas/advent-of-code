pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();
    let boss_hitpoints = lines
        .next()
        .unwrap()
        .trim_start_matches("Hit Points: ")
        .parse::<i32>()
        .unwrap();
    let boss_damage = lines
        .next()
        .unwrap()
        .trim_start_matches("Damage: ")
        .parse::<u32>()
        .unwrap();

    let initial_state = FightState {
        turn: Turn::Player,
        hitpoints: 50,
        mana: 500,
        mana_spent: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        boss_hitpoints,
    };
    let mut states = vec![initial_state];
    let mut lowest_mana_win = u32::MAX;
    while let Some(mut state) = states.pop() {
        if state.mana_spent > lowest_mana_win {
            continue;
        }
        if state.boss_hitpoints <= 0 {
            if state.mana_spent < lowest_mana_win {
                lowest_mana_win = state.mana_spent;
            }
            continue;
        }

        if state.shield_timer > 0 {
            state.shield_timer -= 1;
        }
        if state.poison_timer > 0 {
            state.poison_timer -= 1;
            state.boss_hitpoints -= 3;
        }
        if state.recharge_timer > 0 {
            state.recharge_timer -= 1;
            state.mana += 101;
        }

        if state.boss_hitpoints <= 0 {
            if state.mana_spent < lowest_mana_win {
                lowest_mana_win = state.mana_spent;
            }
            continue;
        }

        match state.turn {
            Turn::Player => {
                state.turn = Turn::Boss;
                if state.mana < 53 {
                    continue;
                }
                if state.mana >= 53 {
                    let mut new_state = state.clone();
                    new_state.boss_hitpoints -= 4;
                    new_state.mana -= 53;
                    new_state.mana_spent += 53;
                    states.push(new_state);
                }
                if state.mana >= 73 {
                    let mut new_state = state.clone();
                    new_state.boss_hitpoints -= 2;
                    new_state.hitpoints += 2;
                    new_state.mana -= 73;
                    new_state.mana_spent += 73;
                    states.push(new_state);
                }
                if state.mana >= 113 && state.shield_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.shield_timer = 6;
                    new_state.mana -= 113;
                    new_state.mana_spent += 113;
                    states.push(new_state);
                }
                if state.mana >= 173 && state.poison_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.poison_timer = 6;
                    new_state.mana -= 173;
                    new_state.mana_spent += 173;
                    states.push(new_state);
                }
                if state.mana >= 229 && state.recharge_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.recharge_timer = 5;
                    new_state.mana -= 229;
                    new_state.mana_spent += 229;
                    states.push(new_state);
                }
            }
            Turn::Boss => {
                let damage = if state.shield_timer > 0 {
                    (boss_damage - 7).max(1)
                } else {
                    boss_damage
                };
                state.hitpoints -= damage as i32;
                if state.hitpoints <= 0 {
                    continue;
                }
                state.turn = Turn::Player;
                states.push(state);
            }
        }
    }
    let part1 = lowest_mana_win;

    let mut lines = input.lines();
    let boss_hitpoints = lines
        .next()
        .unwrap()
        .trim_start_matches("Hit Points: ")
        .parse::<i32>()
        .unwrap();
    let boss_damage = lines
        .next()
        .unwrap()
        .trim_start_matches("Damage: ")
        .parse::<u32>()
        .unwrap();

    let initial_state = FightState {
        turn: Turn::Player,
        hitpoints: 50,
        mana: 500,
        mana_spent: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        boss_hitpoints,
    };
    let mut states = vec![initial_state];
    let mut lowest_mana_win = u32::MAX;
    while let Some(mut state) = states.pop() {
        if state.mana_spent >= lowest_mana_win {
            continue;
        }
        if state.boss_hitpoints <= 0 {
            if state.mana_spent < lowest_mana_win {
                lowest_mana_win = state.mana_spent;
            }
            continue;
        }
        if state.turn == Turn::Player {
            state.hitpoints -= 1;
            if state.hitpoints <= 0 {
                continue;
            }
        }

        if state.shield_timer > 0 {
            state.shield_timer -= 1;
        }
        if state.poison_timer > 0 {
            state.poison_timer -= 1;
            state.boss_hitpoints -= 3;
        }
        if state.recharge_timer > 0 {
            state.recharge_timer -= 1;
            state.mana += 101;
        }

        if state.boss_hitpoints <= 0 {
            if state.mana_spent < lowest_mana_win {
                lowest_mana_win = state.mana_spent;
            }
            continue;
        }

        match state.turn {
            Turn::Player => {
                state.turn = Turn::Boss;
                if state.mana < 53 {
                    continue;
                }
                if state.mana >= 53 {
                    let mut new_state = state.clone();
                    new_state.boss_hitpoints -= 4;
                    new_state.mana -= 53;
                    new_state.mana_spent += 53;
                    states.push(new_state);
                }
                if state.mana >= 73 {
                    let mut new_state = state.clone();
                    new_state.boss_hitpoints -= 2;
                    new_state.hitpoints += 2;
                    new_state.mana -= 73;
                    new_state.mana_spent += 73;
                    states.push(new_state);
                }
                if state.mana >= 113 && state.shield_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.shield_timer = 6;
                    new_state.mana -= 113;
                    new_state.mana_spent += 113;
                    states.push(new_state);
                }
                if state.mana >= 173 && state.poison_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.poison_timer = 6;
                    new_state.mana -= 173;
                    new_state.mana_spent += 173;
                    states.push(new_state);
                }
                if state.mana >= 229 && state.recharge_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.recharge_timer = 5;
                    new_state.mana -= 229;
                    new_state.mana_spent += 229;
                    states.push(new_state);
                }
            }
            Turn::Boss => {
                let damage = if state.shield_timer > 0 {
                    (boss_damage - 7).max(1)
                } else {
                    boss_damage
                };
                state.hitpoints -= damage as i32;
                if state.hitpoints <= 0 {
                    continue;
                }
                state.turn = Turn::Player;
                states.push(state);
            }
        }
    }
    let part2 = lowest_mana_win;

    (part1, part2)
}

#[derive(Debug, Clone)]
struct FightState {
    turn: Turn,

    // Player
    hitpoints: i32,
    mana: u32,
    mana_spent: u32,
    shield_timer: u32,
    poison_timer: u32,
    recharge_timer: u32,

    // Enemy
    boss_hitpoints: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum Turn {
    Player,
    Boss,
}
