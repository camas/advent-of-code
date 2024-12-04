use std::collections::BinaryHeap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut levels = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = search(levels.clone());
    let rows_len = levels[0].len();
    // Extend down
    let initial_levels = levels.clone();
    for i in 1..5 {
        for row in initial_levels.iter() {
            let row = row
                .iter()
                .map(|x| {
                    let mut x = x + i;
                    while x >= 10 {
                        x -= 9;
                    }
                    x
                })
                .collect::<Vec<_>>();
            levels.push(row);
        }
    }
    // Extend right
    let levels = levels
        .into_iter()
        .map(|row| {
            let mut new_row = row.clone();
            for i in 1..5 {
                let extra = row[..rows_len].iter().map(|x| {
                    let mut x = x + i;
                    while x >= 10 {
                        x -= 9;
                    }
                    x
                });
                new_row.extend(extra);
            }
            new_row
        })
        .collect::<Vec<_>>();
    let part2 = search(levels);

    (part1, part2)
}

fn search(levels: Vec<Vec<i64>>) -> i64 {
    let width = levels[0].len() as i64;
    let height = levels.len() as i64;

    let end_point = Point {
        x: width - 1,
        y: height - 1,
    };

    let initial_state = State {
        position: Point { x: 0, y: 0 },
        curr_risk: 0,
    };
    let mut queue = BinaryHeap::new();
    queue.push(initial_state);
    let mut point_bests = vec![vec![i64::max_value(); width as usize]; height as usize];
    loop {
        let curr = queue.pop().unwrap();
        if curr.position == end_point {
            return curr.curr_risk;
        }
        for (m_x, m_y) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = Point {
                x: curr.position.x + m_x,
                y: curr.position.y + m_y,
            };
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= width || new_pos.y >= height {
                continue;
            }
            let new_risk = curr.curr_risk + levels[new_pos.y as usize][new_pos.x as usize];
            let point_best = &mut point_bests[new_pos.y as usize][new_pos.x as usize];
            if new_risk >= *point_best {
                continue;
            }
            *point_best = new_risk;
            let new_state = State {
                position: new_pos,
                curr_risk: new_risk,
            };
            queue.push(new_state);
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    position: Point,
    curr_risk: i64,
    // h: i64,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.curr_risk == other.curr_risk
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.curr_risk.cmp(&self.curr_risk)
        // (other.h + other.curr_risk).cmp(&(self.h + self.curr_risk))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "40");
        assert_eq!(part2.to_string(), "315");
    }
}
