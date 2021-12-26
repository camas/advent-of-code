use std::{
    collections::{BinaryHeap, HashMap},
    convert::TryInto,
    fmt::Display,
};

const ROOM_XS: [usize; 4] = [2, 4, 6, 8];

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rooms: [Vec<Amphipod>; 4] = (0..4)
        .map(|i| {
            let mut room = Vec::new();
            for j in 0.. {
                let x = ROOM_XS[i] + 1;
                let y = j + 2;
                match lines[y][x] {
                    '.' | '#' => break,
                    'A' => room.push(Amphipod::Amber),
                    'B' => room.push(Amphipod::Bronze),
                    'C' => room.push(Amphipod::Copper),
                    'D' => room.push(Amphipod::Desert),
                    _ => unreachable!(),
                }
            }
            room.reverse();
            room
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let part1 = solve_inner(rooms.clone(), 2);

    let extended_rooms = rooms
        .iter()
        .enumerate()
        .map(|(i, room)| {
            let mut room = room.clone();
            match i {
                0 => {
                    room.insert(1, Amphipod::Desert);
                    room.insert(2, Amphipod::Desert);
                }
                1 => {
                    room.insert(1, Amphipod::Bronze);
                    room.insert(2, Amphipod::Copper);
                }
                2 => {
                    room.insert(1, Amphipod::Amber);
                    room.insert(2, Amphipod::Bronze);
                }
                3 => {
                    room.insert(1, Amphipod::Copper);
                    room.insert(2, Amphipod::Amber);
                }
                _ => unreachable!(),
            }
            room
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let part2 = solve_inner(extended_rooms, 4);

    (part1, part2)
}

fn solve_inner(rooms: [Vec<Amphipod>; 4], room_size: usize) -> i64 {
    // A* search
    let initial_state = State::new(rooms, [None; 11], 0, room_size);
    let mut queue = BinaryHeap::new();
    queue.push(initial_state);
    let mut seen = HashMap::new();
    loop {
        let curr = queue.pop().unwrap();
        match seen.get_mut(&(curr.rooms.clone(), curr.hallway)) {
            Some(e) => {
                if curr.energy_used >= *e {
                    continue;
                } else {
                    *e = curr.energy_used;
                }
            }
            None => {
                seen.insert((curr.rooms.clone(), curr.hallway), curr.energy_used);
            }
        };
        if curr.won() {
            return curr.energy_used;
        }
        queue.extend(curr.moves());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn room_x(&self) -> usize {
        match self {
            Amphipod::Amber => ROOM_XS[0],
            Amphipod::Bronze => ROOM_XS[1],
            Amphipod::Copper => ROOM_XS[2],
            Amphipod::Desert => ROOM_XS[3],
        }
    }

    fn energy_cost(&self) -> i64 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn char(&self) -> char {
        match self {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    room_size: usize,
    rooms: [Vec<Amphipod>; 4],
    hallway: [Option<Amphipod>; 11],
    energy_used: i64,
    h: i64,
}

impl State {
    fn new(
        rooms: [Vec<Amphipod>; 4],
        hallway: [Option<Amphipod>; 11],
        energy_used: i64,
        room_size: usize,
    ) -> Self {
        // // Minimum distance needed to move up, to goal and down one, then multiplied by it's energy cost
        let room_h = rooms
            .iter()
            .enumerate()
            .map(|(room_i, r)| {
                r.iter()
                    .enumerate()
                    .map(|(i, a)| {
                        let moves_up = room_size as i64 - i as i64;
                        let moves_x = (a.room_x() as i64 - ROOM_XS[room_i] as i64).abs();
                        if moves_x == 0 {
                            return 0;
                        }
                        (moves_up + moves_x + 1) * a.energy_cost()
                    })
                    .sum::<i64>()
            })
            .sum::<i64>();
        let hallway_h = hallway
            .iter()
            .enumerate()
            .map(|(x, a)| {
                if let Some(a) = a {
                    let moves_x = (a.room_x() as i64 - x as i64).abs();
                    if moves_x == 0 {
                        return 0;
                    }
                    (moves_x + 1) * a.energy_cost()
                } else {
                    0
                }
            })
            .sum::<i64>();
        Self {
            room_size,
            rooms,
            hallway,
            energy_used,
            h: room_h + hallway_h,
        }
    }

    fn won(&self) -> bool {
        self.hallway.iter().all(|a| a.is_none())
            && self.rooms[0].iter().all(|a| *a == Amphipod::Amber)
            && self.rooms[1].iter().all(|a| *a == Amphipod::Bronze)
            && self.rooms[2].iter().all(|a| *a == Amphipod::Copper)
            && self.rooms[3].iter().all(|a| *a == Amphipod::Desert)
    }

    fn moves(&self) -> Vec<State> {
        let mut new_states = Vec::new();

        // Moves from the hallway
        for (x, a) in self.hallway.iter().enumerate() {
            if a.is_none() {
                continue;
            }
            let a = a.as_ref().unwrap();

            macro_rules! test_x_range {
                ($range:expr) => {
                    for test_x in $range {
                        if self.hallway[test_x].is_some() {
                            break;
                        }
                        // Skip if not a room or not own room
                        if !ROOM_XS.contains(&test_x) || test_x != a.room_x() {
                            continue;
                        }
                        let room_index = (test_x - 2) / 2;
                        let room = &self.rooms[room_index];
                        // Skip if room has other types of amphipods
                        if !room.iter().all(|other_a| *other_a == *a) {
                            continue;
                        }
                        // Add room move
                        let moves = (x as i64 - test_x as i64).abs() as usize
                            + (self.room_size - room.len());
                        let mut rooms = self.rooms.clone();
                        rooms[room_index].push(*a);
                        let mut hallway = self.hallway.clone();
                        hallway[x] = None;
                        new_states.push(State::new(
                            rooms,
                            hallway,
                            self.energy_used + (moves as i64 * a.energy_cost()),
                            self.room_size,
                        ));
                    }
                };
            }

            // Find left moves
            if x > 0 {
                test_x_range!((0..x).rev());
            }
            // Find right moves
            test_x_range!((x + 1)..self.hallway.len());
        }
        // Moves from rooms
        for (room_from_i, room_from) in self.rooms.iter().enumerate() {
            // Skip empty rooms
            if room_from.is_empty() {
                continue;
            }
            let a = room_from.last().unwrap();
            // Skip
            let moves_up = (self.room_size - room_from.len()) as i64 + 1;
            let x = ROOM_XS[room_from_i];

            macro_rules! test_x_range {
                ($range:expr) => {
                    for test_x in $range {
                        if self.hallway[test_x].is_some() {
                            break;
                        }
                        if ROOM_XS.contains(&test_x) {
                            // Skip if not own room
                            if test_x != a.room_x() {
                                continue;
                            }
                            let room_index = (test_x - 2) / 2;
                            let room = &self.rooms[room_index];
                            // Skip if room has other types of amphipods
                            if !room.iter().all(|other_a| *other_a == *a) {
                                continue;
                            }
                            // Add room move
                            let moves = (x as i64 - test_x as i64).abs()
                                + (self.room_size - room.len()) as i64
                                + moves_up;
                            let mut rooms = self.rooms.clone();
                            rooms[room_from_i].pop().unwrap();
                            rooms[room_index].push(*a);
                            let hallway = self.hallway.clone();
                            new_states.push(State::new(
                                rooms,
                                hallway,
                                self.energy_used + (moves * a.energy_cost()),
                                self.room_size,
                            ));
                        } else {
                            // Add corridor move
                            let moves = (x as i64 - test_x as i64).abs() + moves_up;
                            let mut rooms = self.rooms.clone();
                            rooms[room_from_i].pop().unwrap();
                            let mut hallway = self.hallway.clone();
                            hallway[test_x] = Some(*a);
                            new_states.push(State::new(
                                rooms,
                                hallway,
                                self.energy_used + (moves as i64 * a.energy_cost()),
                                self.room_size,
                            ));
                        }
                    }
                };
            }

            // Find left moves
            test_x_range!((0..x).rev());
            // Find right moves
            test_x_range!((x + 1)..self.hallway.len());
        }
        new_states
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.h + self.energy_used).eq(&(other.h + other.energy_used))
        // self.energy_used.eq(&other.energy_used)
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
        (other.h + other.energy_used).cmp(&(self.h + self.energy_used))
        // other.energy_used.cmp(&self.energy_used)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}#",
            self.hallway
                .iter()
                .map(|x| {
                    match x {
                        Some(a) => a.char(),
                        None => '.',
                    }
                })
                .collect::<String>()
        )?;
        for i in 0..self.room_size {
            let room_i = self.room_size - 1 - i;
            writeln!(
                f,
                "  #{}#{}#{}#{}#  ",
                self.rooms[0].get(room_i).map_or('.', |a| a.char()),
                self.rooms[1].get(room_i).map_or('.', |a| a.char()),
                self.rooms[2].get(room_i).map_or('.', |a| a.char()),
                self.rooms[3].get(room_i).map_or('.', |a| a.char()),
            )?;
        }
        write!(f, "  #########  ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "12521");
        assert_eq!(part2.to_string(), "44169");
    }
}
