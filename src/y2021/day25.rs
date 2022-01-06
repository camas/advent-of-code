use std::collections::HashSet;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut map = Map::from_str(input);
    let mut part1 = 0;
    for step in 1.. {
        let next_map = map.step();
        if next_map == map {
            part1 = step;
            break;
        }
        map = next_map;
    }

    (part1, "N/A")
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    right_moving: HashSet<Point>,
    down_moving: HashSet<Point>,
    width: u32,
    height: u32,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut right_moving = HashSet::new();
        let mut down_moving = HashSet::new();
        let mut last_y = 0;
        let mut last_x = 0;
        for (y, line) in s.lines().enumerate() {
            last_y = y;
            for (x, c) in line.chars().enumerate() {
                last_x = x;
                match c {
                    '>' => right_moving.insert(Point {
                        x: x as u32,
                        y: y as u32,
                    }),
                    'v' => down_moving.insert(Point {
                        x: x as u32,
                        y: y as u32,
                    }),
                    _ => false,
                };
            }
        }
        Self {
            right_moving,
            down_moving,
            width: last_x as u32 + 1,
            height: last_y as u32 + 1,
        }
    }

    fn step(&self) -> Self {
        let mut new_right = HashSet::new();
        let mut new_down = HashSet::new();
        for p in self.right_moving.iter() {
            let mut new_x = p.x + 1;
            if new_x >= self.width {
                new_x = 0;
            }
            let new_p = Point { x: new_x, y: p.y };
            if !self.right_moving.contains(&new_p) && !self.down_moving.contains(&new_p) {
                new_right.insert(new_p);
            } else {
                new_right.insert(*p);
            }
        }
        for p in self.down_moving.iter() {
            let mut new_y = p.y + 1;
            if new_y >= self.height {
                new_y = 0;
            }
            let new_p = Point { x: p.x, y: new_y };
            if !new_right.contains(&new_p) && !self.down_moving.contains(&new_p) {
                new_down.insert(new_p);
            } else {
                new_down.insert(*p);
            }
        }
        Self {
            right_moving: new_right,
            down_moving: new_down,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}
