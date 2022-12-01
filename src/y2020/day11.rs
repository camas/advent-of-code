pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let seating = Seating::from(input);

    // Part 1: Step until no change
    let mut state = seating.clone();
    let mut last_state = Seating { data: Vec::new() };
    loop {
        if state == last_state {
            break;
        }
        last_state = state;
        state = last_state.step();
    }
    let part1 = state.occupied();

    // Part 2: Alt-step until no change
    let mut state = seating;
    let mut last_state = Seating { data: Vec::new() };
    loop {
        if state == last_state {
            break;
        }
        last_state = state;
        state = last_state.step2();
    }
    let part2 = state.occupied();

    (part1, part2)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Seating {
    data: Vec<Vec<Tile>>,
}

impl Seating {
    fn step(&self) -> Seating {
        let data = (0..self.data.len())
            .map(|y| {
                (0..self.data[0].len())
                    .map(|x| {
                        let curr = &self.data[y][x];
                        match curr {
                            Tile::Empty => {
                                if self.count_adjacent(x, y, Tile::Occupied) == 0 {
                                    Tile::Occupied
                                } else {
                                    Tile::Empty
                                }
                            }
                            Tile::Occupied => {
                                if self.count_adjacent(x, y, Tile::Occupied) >= 4 {
                                    Tile::Empty
                                } else {
                                    Tile::Occupied
                                }
                            }
                            Tile::Floor => Tile::Floor,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { data }
    }

    fn step2(&self) -> Self {
        let data = (0..self.data.len())
            .map(|y| {
                (0..self.data[0].len())
                    .map(|x| {
                        let curr = &self.data[y][x];
                        match curr {
                            Tile::Empty => {
                                if self.count_occupied_by_sight(x, y) == 0 {
                                    Tile::Occupied
                                } else {
                                    Tile::Empty
                                }
                            }
                            Tile::Occupied => {
                                if self.count_occupied_by_sight(x, y) >= 5 {
                                    Tile::Empty
                                } else {
                                    Tile::Occupied
                                }
                            }
                            Tile::Floor => Tile::Floor,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { data }
    }

    fn count_adjacent(&self, x: usize, y: usize, tile_type: Tile) -> usize {
        let mut count = 0;
        let width = self.data[0].len();
        let height = self.data.len();
        for j in -1..=1 {
            for i in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if i == -1 && x == 0 {
                    continue;
                }
                if i == 1 && x == width - 1 {
                    continue;
                }
                if j == -1 && y == 0 {
                    continue;
                }
                if j == 1 && y == height - 1 {
                    continue;
                }
                if self.data[(y as i64 + j) as usize][(x as i64 + i) as usize] == tile_type {
                    count += 1;
                }
            }
        }
        count
    }

    /// Counts the number of visible occupied seats
    ///
    /// Visible meaning straight or diagonally from `(x, y)`
    fn count_occupied_by_sight(&self, x: usize, y: usize) -> usize {
        const DIRS: &[(i64, i64)] = &[
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        DIRS.iter()
            .filter(|(dir_y, dir_x)| {
                let mut result = false;
                for i in 1.. {
                    let cur_x = x as i64 + (*dir_x * i);
                    let cur_y = y as i64 + (*dir_y * i);
                    if cur_x < 0
                        || cur_y < 0
                        || cur_x as usize >= self.data[0].len()
                        || cur_y as usize >= self.data.len()
                    {
                        result = false;
                        break;
                    }
                    let tile = &self.data[cur_y as usize][cur_x as usize];
                    match tile {
                        Tile::Occupied => {
                            result = true;
                            break;
                        }
                        Tile::Empty => {
                            result = false;
                            break;
                        }
                        Tile::Floor => continue,
                    }
                }
                result
            })
            .count()
    }

    fn occupied(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&t| t == &Tile::Occupied).count())
            .sum()
    }
}

impl<S: AsRef<str>> From<S> for Seating {
    fn from(s: S) -> Self {
        let data = s
            .as_ref()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Floor,
                        'L' => Tile::Empty,
                        '#' => Tile::Occupied,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { data }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}
