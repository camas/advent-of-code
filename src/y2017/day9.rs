use crate::Exercise;

pub struct Day9;

impl Exercise for Day9 {
    fn part1(&self, input: &str) -> String {
        enum State {
            Normal,
            Ignore,
            Garbage,
        }
        let mut state = State::Normal;
        let mut total_score = 0;
        let mut curr_deep = 0;
        for c in input.trim().chars() {
            match state {
                State::Normal => match c {
                    '{' => {
                        curr_deep += 1;
                        total_score += curr_deep;
                    }
                    '}' => curr_deep -= 1,
                    '<' => state = State::Garbage,
                    ',' => (),
                    other => panic!("Unexpected char {:?}", other),
                },
                State::Garbage => match c {
                    '!' => state = State::Ignore,
                    '>' => state = State::Normal,
                    _ => (),
                },
                State::Ignore => state = State::Garbage,
            }
        }
        total_score.to_string()
    }

    fn part2(&self, input: &str) -> String {
        enum State {
            Normal,
            Ignore,
            Garbage,
        }
        let mut state = State::Normal;
        let mut total_score = 0;
        for c in input.trim().chars() {
            match state {
                State::Normal => match c {
                    '{' => (),
                    '}' => (),
                    '<' => state = State::Garbage,
                    ',' => (),
                    other => panic!("Unexpected char {:?}", other),
                },
                State::Garbage => match c {
                    '!' => state = State::Ignore,
                    '>' => state = State::Normal,
                    _ => total_score += 1,
                },
                State::Ignore => state = State::Garbage,
            }
        }
        total_score.to_string()
    }
}
