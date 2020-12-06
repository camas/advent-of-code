use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let actions = input
        .trim()
        .split(',')
        .map(|line| Action::from_str(line))
        .collect::<Vec<_>>();
    let mut programs = ('a'..='p').collect::<VecDeque<_>>();
    for action in actions {
        match action {
            Action::Spin { amount } => programs.rotate_right(amount),
            Action::Swap { a, b } => programs.swap(a, b),
            Action::Partner { a, b } => {
                let a_pos = programs.iter().position(|&e| e == a).unwrap();
                let b_pos = programs.iter().position(|&e| e == b).unwrap();
                programs.swap(a_pos, b_pos);
            }
        }
    }
    let part1 = programs.iter().collect::<String>();
    let actions = input
        .trim()
        .split(',')
        .map(|line| Action::from_str(line))
        .collect::<Vec<_>>();
    let mut programs = ('a'..='p').collect::<VecDeque<_>>();
    let mut seen = HashSet::new();
    seen.insert(programs.clone());
    for step in 1.. {
        for action in actions.iter() {
            match action {
                Action::Spin { amount } => programs.rotate_right(*amount),
                Action::Swap { a, b } => programs.swap(*a, *b),
                Action::Partner { a, b } => {
                    let a_pos = programs.iter().position(|e| e == a).unwrap();
                    let b_pos = programs.iter().position(|e| e == b).unwrap();
                    programs.swap(a_pos, b_pos);
                }
            }
        }
        if !seen.insert(programs.clone()) {
            let remaining_steps = 1_000_000_000 % step;
            for _ in 0..remaining_steps {
                for action in actions.iter() {
                    match action {
                        Action::Spin { amount } => programs.rotate_right(*amount),
                        Action::Swap { a, b } => programs.swap(*a, *b),
                        Action::Partner { a, b } => {
                            let a_pos = programs.iter().position(|e| e == a).unwrap();
                            let b_pos = programs.iter().position(|e| e == b).unwrap();
                            programs.swap(a_pos, b_pos);
                        }
                    }
                }
            }
            return (part1, programs.iter().collect::<String>());
        }
    }
    unreachable!()
}

enum Action {
    Spin { amount: usize },
    Swap { a: usize, b: usize },
    Partner { a: char, b: char },
}

impl Action {
    fn from_str(value: &str) -> Self {
        match value.chars().next().unwrap() {
            's' => Action::Spin {
                amount: value[1..].parse::<usize>().unwrap(),
            },
            'x' => {
                let mut parts = value[1..]
                    .split('/')
                    .map(|part| part.parse::<usize>().unwrap());
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                Action::Swap { a, b }
            }
            'p' => {
                let mut parts = value[1..]
                    .split('/')
                    .map(|part| part.chars().next().unwrap());
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                Action::Partner { a, b }
            }
            _ => panic!(),
        }
    }
}
