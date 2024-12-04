use std::{convert::TryInto, str::FromStr};

const BOARD_SIZE: usize = 5;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input
    let lines = input.lines().collect::<Vec<_>>();
    let picks = lines[0]
        .split(',')
        .map(|v| i32::from_str(v).unwrap())
        .collect::<Vec<_>>();
    let mut boards = lines[1..]
        .chunks(6)
        .map(Board::from_chunk)
        .collect::<Vec<_>>();

    // Loop through every number, taking out boards as they win
    // The first and last winning scores are saved to part1 and part2
    let mut part1 = None;
    let mut part2 = None;
    for n in picks {
        let mut to_remove = Vec::new();
        for (i, board) in boards.iter_mut().enumerate() {
            board.pick(n);
            if board.won() {
                if part1.is_none() {
                    part1 = Some(n * board.score());
                }
                part2 = Some(n * board.score());
                to_remove.push(i);
            }
        }
        for i in to_remove.iter().rev() {
            boards.remove(*i);
        }
    }

    (part1.unwrap(), part2.unwrap())
}

#[derive(Debug)]
struct Board {
    numbers: [[i32; BOARD_SIZE]; BOARD_SIZE],
    picked: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn from_chunk(chunk: &[&str]) -> Board {
        let numbers = chunk[1..]
            .iter()
            .map(|line| {
                line.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| i32::from_str(v).unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Board {
            numbers,
            picked: [[false; 5]; 5],
        }
    }

    fn pick(&mut self, n: i32) {
        for (y, row) in self.numbers.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if n == *v {
                    self.picked[y][x] = true;
                }
            }
        }
    }

    // Check if won and returns score if it has
    fn won(&self) -> bool {
        // Rows
        for row in self.picked.iter() {
            if row.iter().all(|&v| v) {
                return true;
            }
        }
        // Columns
        for x in 0..BOARD_SIZE {
            if self.picked.iter().all(|row| row[x]) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for (y, picked_row) in self.picked.iter().enumerate() {
            for (x, p) in picked_row.iter().enumerate() {
                if !p {
                    score += self.numbers[y][x];
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let data = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (part1, part2) = solve(data);
        assert_eq!(part1.to_string(), "4512");
        assert_eq!(part2.to_string(), "1924");
    }
}
