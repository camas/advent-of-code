use std::str::FromStr;

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let forest = input.parse::<Forest>().unwrap();

    let part1 = forest
        .calculate_visible()
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum::<usize>();

    let part2 = forest.find_best_place_for_treehouse();

    (part1, part2)
}

struct Forest {
    tree_heights: Vec<Vec<u8>>,
}

impl Forest {
    fn height(&self) -> usize {
        self.tree_heights.len()
    }

    fn width(&self) -> usize {
        self.tree_heights[0].len()
    }

    fn height_at(&self, position: Vector2) -> u8 {
        assert!(position.y >= 0 && position.x >= 0);
        self.tree_heights[position.y as usize][position.x as usize]
    }

    fn in_bounds(&self, position: Vector2) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.width() as i64
            && position.y < self.height() as i64
    }

    fn calculate_visible(&self) -> Vec<Vec<bool>> {
        let mut visible_map = (0..self.height())
            .map(|_| (0..self.width()).map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Set edges visible
        visible_map[0].iter_mut().for_each(|v| *v = true);
        visible_map
            .last_mut()
            .unwrap()
            .iter_mut()
            .for_each(|v| *v = true);
        (1..(self.height() - 1)).for_each(|y| {
            visible_map[y][0] = true;
            *visible_map[y].last_mut().unwrap() = true;
        });

        struct QueueItem {
            start: Vector2,
            direction: Vector2,
        }

        let mut queue = Vec::new();
        for x in 1..(self.width() - 1) {
            queue.push(QueueItem {
                start: Vector2::new(x as i64, 0),
                direction: Vector2::new(0, 1),
            });
            queue.push(QueueItem {
                start: Vector2::new(x as i64, self.height() as i64 - 1),
                direction: Vector2::new(0, -1),
            });
        }
        for y in 1..(self.height() - 1) {
            queue.push(QueueItem {
                start: Vector2::new(0, y as i64),
                direction: Vector2::new(1, 0),
            });
            queue.push(QueueItem {
                start: Vector2::new(self.width() as i64 - 1, y as i64),
                direction: Vector2::new(-1, 0),
            });
        }

        while let Some(item) = queue.pop() {
            let mut position = item.start;
            let mut highest_seen = self.height_at(position);
            loop {
                position += item.direction;

                if !self.in_bounds(position) {
                    break;
                }

                let cur_height = self.height_at(position);
                if cur_height <= highest_seen {
                    continue;
                }

                visible_map[position.y as usize][position.x as usize] = true;
                highest_seen = cur_height;
            }
        }

        visible_map
    }

    fn find_best_place_for_treehouse(&self) -> u64 {
        let mut best = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let treehouse_height = self.height_at(Vector2::new(x as i64, y as i64));
                // for x in [3] {
                // for y in [2] {
                let mut score = 1;
                for direction in [
                    Vector2::new(0, 1),
                    Vector2::new(0, -1),
                    Vector2::new(1, 0),
                    Vector2::new(-1, 0),
                ] {
                    let mut distance = 0;
                    let mut position = Vector2::new(x as i64, y as i64);
                    loop {
                        position += direction;
                        if !self.in_bounds(position) {
                            break;
                        }
                        distance += 1;
                        if self.height_at(position) >= treehouse_height {
                            break;
                        }
                    }
                    score *= distance;
                }
                if score > best {
                    best = score;
                }
            }
        }

        best
    }
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tree_heights = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        assert!((b'0'..=b'9').contains(&(c as u8)));
                        c as u8 - b'0'
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert!(tree_heights
            .iter()
            .all(|row| row.len() == tree_heights[0].len()));

        Ok(Forest { tree_heights })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "30373
25512
65332
33549
35390";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 21.to_string());
        assert_eq!(result.1.to_string(), 8.to_string());
    }
}
