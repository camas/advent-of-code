use std::str::FromStr;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let slope = input
        .lines()
        .map(|line| line.parse::<Row>().unwrap())
        .collect::<Vec<_>>();

    let mut x = 0;
    let mut trees_hit = 0;
    for row in slope.iter().skip(1) {
        x += 3;
        if row.get(x) == '#' {
            trees_hit += 1;
        }
    }

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = slopes
        .iter()
        .map(|(right, down)| {
            let mut x = 0;
            let mut trees_hit = 0;
            for row in slope.iter().step_by(*down as usize).skip(1) {
                x += right;
                if row.get(x) == '#' {
                    trees_hit += 1;
                }
            }
            trees_hit
        })
        .product::<u64>();

    (trees_hit, part2)
}

struct Row {
    pattern: Vec<char>,
}

impl Row {
    fn get(&self, offset: usize) -> char {
        self.pattern[offset % self.pattern.len()]
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pattern: s.chars().collect(),
        })
    }
}
