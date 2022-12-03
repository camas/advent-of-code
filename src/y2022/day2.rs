use std::str::FromStr;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let simple_strategy = input
        .trim()
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|i| Item::from_str(i).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = simple_strategy
        .iter()
        .map(|round| {
            let selection_score = round[1].score();
            let result_score = round[1].result(round[0]).score();

            selection_score + result_score
        })
        .sum::<u64>();

    let part2 = input
        .trim()
        .lines()
        .map(|l| {
            let (opponent, result_needed) = l.split_once(' ').unwrap();
            let opponent = Item::from_str(opponent).unwrap();
            let result_needed = GameResult::from_str(result_needed).unwrap();
            let move_picked = opponent.to_get_result(result_needed);

            move_picked.score() + result_needed.score()
        })
        .sum::<u64>();

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl Item {
    fn score(self) -> u64 {
        match self {
            Item::Rock => 1,
            Item::Paper => 2,
            Item::Scissors => 3,
        }
    }

    fn result(self, other: Item) -> GameResult {
        match (self, other) {
            (Item::Rock, Item::Rock) => GameResult::Draw,
            (Item::Rock, Item::Paper) => GameResult::Lose,
            (Item::Rock, Item::Scissors) => GameResult::Win,
            (Item::Paper, Item::Rock) => GameResult::Win,
            (Item::Paper, Item::Paper) => GameResult::Draw,
            (Item::Paper, Item::Scissors) => GameResult::Lose,
            (Item::Scissors, Item::Rock) => GameResult::Lose,
            (Item::Scissors, Item::Paper) => GameResult::Win,
            (Item::Scissors, Item::Scissors) => GameResult::Draw,
        }
    }

    fn to_get_result(self, result: GameResult) -> Item {
        match (self, result) {
            (Item::Rock, GameResult::Win) => Item::Paper,
            (Item::Rock, GameResult::Lose) => Item::Scissors,
            (Item::Paper, GameResult::Win) => Item::Scissors,
            (Item::Paper, GameResult::Lose) => Item::Rock,
            (Item::Scissors, GameResult::Win) => Item::Rock,
            (Item::Scissors, GameResult::Lose) => Item::Paper,
            (_, GameResult::Draw) => self,
        }
    }
}

impl GameResult {
    fn score(self) -> u64 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Item::Rock,
            "B" | "Y" => Item::Paper,
            "C" | "Z" => Item::Scissors,
            _ => unreachable!(),
        })
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => unreachable!(),
        })
    }
}
