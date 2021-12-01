use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let initial = input
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y, _)| Vector3 {
                    x: x as i64,
                    y: y as i64,
                    z: 0,
                })
        })
        .collect::<HashSet<_>>();

    let mut state = initial.clone();
    for _ in 0..6 {
        let count = count_neighbours(&state);
        state = count
            .into_iter()
            .filter(|(v, c)| *c == 3 || (*c == 2 && state.contains(v)))
            .map(|(v, _)| v)
            .collect::<HashSet<_>>();
    }
    let part1 = state.len();

    let mut state = initial
        .into_iter()
        .map(|v| v.into_vec4())
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        state = count_neighbours4(&state)
            .into_iter()
            .filter(|(v, c)| *c == 3 || (*c == 2 && state.contains(v)))
            .map(|(v, _)| v)
            .collect::<HashSet<_>>();
    }
    let part2 = state.len();

    (part1, part2)
}

fn count_neighbours(state: &HashSet<Vector3>) -> HashMap<Vector3, u64> {
    let mut result = HashMap::new();
    for v in state.iter() {
        for neighbour in v.neighbours() {
            *result.entry(neighbour).or_insert(0) += 1;
        }
    }
    result
}

fn count_neighbours4(state: &HashSet<Vector4>) -> HashMap<Vector4, u64> {
    let mut result = HashMap::new();
    for v in state.iter() {
        for neighbour in v.neighbours() {
            *result.entry(neighbour).or_insert(0) += 1;
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn neighbours(&self) -> impl Iterator<Item = Vector3> + '_ {
        (-1..=1).flat_map(move |x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1)
                    .filter(move |z| !(x == 0 && y == 0 && *z == 0))
                    .map(move |z| Vector3 {
                        x: self.x + x,
                        y: self.y + y,
                        z: self.z + z,
                    })
            })
        })
    }

    fn into_vec4(self) -> Vector4 {
        Vector4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vector4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Vector4 {
    fn neighbours(&self) -> impl Iterator<Item = Vector4> + '_ {
        (-1..=1).flat_map(move |x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).flat_map(move |z| {
                    (-1..=1)
                        .filter(move |w| !(x == 0 && y == 0 && z == 0 && *w == 0))
                        .map(move |w| Vector4 {
                            x: self.x + x,
                            y: self.y + y,
                            z: self.z + z,
                            w: self.w + w,
                        })
                })
            })
        })
    }
}
