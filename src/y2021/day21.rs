use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let starts = input
        .lines()
        .map(|l| l.split(": ").last().unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut positions = (starts[0] - 1, starts[1] - 1);
    let mut scores = (0, 0);
    let mut dice = Dice::new();
    let part1;
    loop {
        let player1_roll = dice.roll() + dice.roll() + dice.roll();
        positions.0 = (positions.0 + player1_roll) % 10;
        scores.0 += positions.0 + 1;
        if scores.0 >= 1000 {
            part1 = scores.1 * dice.rolls;
            break;
        }
        let player2_roll = dice.roll() + dice.roll() + dice.roll();
        positions.1 = (positions.1 + player2_roll) % 10;
        scores.1 += positions.1 + 1;
        if scores.1 >= 1000 {
            part1 = scores.0 * dice.rolls;
            break;
        }
    }

    let mut states = HashMap::new();
    states.insert(
        State {
            positions: [starts[0] - 1, starts[1] - 1],
            scores: [0, 0],
        },
        1,
    );
    let mut wins = [0_i128, 0];
    let mut player2_turn = false;
    while !states.is_empty() {
        let mut new_states = HashMap::new();
        let player_index = player2_turn as usize;
        for (state, state_count) in states {
            for (roll, roll_count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let mut new_state = state.clone();
                new_state.positions[player_index] = (new_state.positions[player_index] + roll) % 10;
                new_state.scores[player_index] += new_state.positions[player_index] + 1;
                if new_state.scores[player_index] >= 21 {
                    wins[player_index] += state_count * roll_count;
                    continue;
                }
                *new_states.entry(new_state).or_default() += state_count * roll_count;
            }
        }

        states = new_states;
        player2_turn = !player2_turn;
    }
    let part2 = *wins.iter().max().unwrap();

    (part1, part2)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    positions: [usize; 2],
    scores: [usize; 2],
}

struct Dice {
    value: usize,
    rolls: usize,
}

impl Dice {
    fn new() -> Dice {
        Dice { value: 1, rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        let roll = self.value;
        self.rolls += 1;
        self.value += 1;
        if self.value > 100 {
            self.value = 1;
        }
        roll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"Player 1 starting position: 4
Player 2 starting position: 8
";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "739785");
        assert_eq!(part2.to_string(), "444356092776315");
    }
}
