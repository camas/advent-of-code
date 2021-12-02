use std::str::FromStr;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input
    let commands = input
        .lines()
        .map(|l| l.parse::<Command>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    let mut depth = 0;
    let mut horizontal = 0;
    for command in commands.iter() {
        match command {
            Command::Forward(s) => horizontal += s,
            Command::Up(s) => depth -= s,
            Command::Down(s) => depth += s,
        }
    }
    let part1 = depth * horizontal;

    // Part 2, now with aim
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for command in commands.iter() {
        match command {
            Command::Forward(s) => {
                horizontal += s;
                depth += aim * s;
            }
            Command::Up(s) => aim -= s,
            Command::Down(s) => aim += s,
        }
    }
    let part2 = depth * horizontal;

    (part1, part2)
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, size) = s.split_once(' ').unwrap();
        let size = size.parse::<i32>().unwrap();
        Ok(match command {
            "forward" => Self::Forward(size),
            "up" => Self::Up(size),
            "down" => Self::Down(size),
            _ => unreachable!(),
        })
    }
}
