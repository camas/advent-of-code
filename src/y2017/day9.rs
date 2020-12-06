pub fn solve(input: &str) -> (impl ToString, impl ToString) {
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
    let part1 = total_score;

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
    let part2 = total_score;

    (part1, part2)
}
