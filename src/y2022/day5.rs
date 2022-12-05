pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();

    let stack_lines = lines
        .by_ref()
        .take_while(|l| l.trim() != "")
        .collect::<Vec<_>>();
    let stack_count = (stack_lines[stack_lines.len() - 1].len() + 1) / 4;
    let stacks = (0..stack_count)
        .map(|i| {
            let str_offset = 4 * i + 1;
            (0..(stack_lines.len() - 1))
                .rev()
                .filter_map(|vert_offset| {
                    match stack_lines[vert_offset].chars().nth(str_offset).unwrap() {
                        ' ' => None,
                        c => Some(c),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let moves = lines
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<_>>();

            let count = parts[1].parse().unwrap();
            let from = parts[3].parse().unwrap();
            let to = parts[5].parse().unwrap();

            Move { from, to, count }
        })
        .collect::<Vec<_>>();

    let mut ship = Ship {
        stacks: stacks.clone(),
    };
    for m in moves.iter() {
        ship.apply_move(m);
    }
    let part1 = ship.top_chars();

    let mut ship = Ship { stacks };
    for m in moves.iter() {
        ship.apply_move_alt(m);
    }
    let part2 = ship.top_chars();

    (part1, part2)
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.count {
            let c = self.stacks[m.from - 1].pop().unwrap();
            self.stacks[m.to - 1].push(c);
        }
    }

    fn apply_move_alt(&mut self, m: &Move) {
        let split_pos = self.stacks[m.from - 1].len() - m.count;
        let chars = self.stacks[m.from - 1].split_off(split_pos);
        self.stacks[m.to - 1].extend(chars);
    }

    fn top_chars(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s[s.len() - 1])
            .collect::<String>()
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let result = solve(input);

        assert_eq!(result.0.to_string(), "CMZ".to_string());
        assert_eq!(result.1.to_string(), "MCD".to_string());
    }
}
