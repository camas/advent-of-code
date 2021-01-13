use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();
    let player1 = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let player2 = lines
        .skip(1)
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = play(&player1, &player2)
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| v * (i as u64 + 1))
        .sum::<u64>();

    let part2 = play_recursive(player1.into_iter().collect(), player2.into_iter().collect())
        .1
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| v * (i as u64 + 1))
        .sum::<u64>();

    (part1, part2)
}

// Plays a game and returns the winning players deck
fn play(player1: &[u64], player2: &[u64]) -> VecDeque<u64> {
    let mut player1 = player1.iter().cloned().collect::<VecDeque<_>>();
    let mut player2 = player2.iter().cloned().collect::<VecDeque<_>>();

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    if !player1.is_empty() {
        player1
    } else {
        player2
    }
}

// Plays a recursive game, returning true if player1 won and the winning players deck
fn play_recursive(mut player1: VecDeque<u64>, mut player2: VecDeque<u64>) -> (bool, VecDeque<u64>) {
    let mut seen = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        let player1_won = if card1 <= player1.len() as u64 && card2 <= player2.len() as u64 {
            play_recursive(
                player1.iter().take(card1 as usize).cloned().collect(),
                player2.iter().take(card2 as usize).cloned().collect(),
            )
            .0
        } else {
            card1 > card2
        };
        if player1_won {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }

        if !seen.insert((player1.clone(), player2.clone())) {
            return (true, player1);
        }
    }

    if !player1.is_empty() {
        (true, player1)
    } else {
        (false, player2)
    }
}
