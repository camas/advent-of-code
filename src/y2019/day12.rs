use std::{cmp::Ordering, collections::HashMap};

use num::integer::lcm;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let initial_moons = input
        .lines()
        .map(|l| Moon {
            position: Vector3::from_str(l),
            velocity: Vector3 { x: 0, y: 0, z: 0 },
        })
        .collect::<Vec<_>>();

    let mut moons = initial_moons.clone();
    for _ in 0..1000 {
        step(&mut moons);
    }
    let part1 = moons.iter().map(Moon::energy).sum::<i64>();

    let mut moons = initial_moons;
    let mut x_cycle = None;
    let mut y_cycle = None;
    let mut z_cycle = None;
    let mut x_seen = HashMap::<Vec<(i64, i64)>, i64>::new();
    let mut y_seen = HashMap::<Vec<(i64, i64)>, i64>::new();
    let mut z_seen = HashMap::<Vec<(i64, i64)>, i64>::new();
    for i in 0.. {
        macro_rules! do_dim {
            ($dim:ident, $seen:ident, $cycle:ident) => {
                if !$cycle.is_some() {
                    let dim_part = moons
                        .iter()
                        .map(|m| (m.position.$dim, m.velocity.$dim))
                        .collect::<Vec<_>>();
                    if let Some(index) = $seen.get(&dim_part) {
                        $cycle = Some((*index, i - *index));
                    } else {
                        $seen.insert(dim_part, i);
                    }
                }
            };
        }
        do_dim!(x, x_seen, x_cycle);
        do_dim!(y, y_seen, y_cycle);
        do_dim!(z, z_seen, z_cycle);
        if x_cycle.is_some() && y_cycle.is_some() && z_cycle.is_some() {
            break;
        }
        step(&mut moons);
    }
    let x_cycle = x_cycle.unwrap();
    let y_cycle = y_cycle.unwrap();
    let z_cycle = z_cycle.unwrap();
    assert!(x_cycle.0 == 0 && y_cycle.0 == 0 && z_cycle.0 == 0);
    let part2 = lcm(lcm(x_cycle.1, y_cycle.1), z_cycle.1);

    (part1, part2)
}

fn step(moons: &mut [Moon]) {
    for i in 0..(moons.len() - 1) {
        let (moon, others) = moons.split_at_mut(i + 1);
        let moon = moon.last_mut().unwrap();
        for other in others {
            macro_rules! apply_gravity {
                ($v:ident) => {
                    match moon.position.$v.cmp(&other.position.$v) {
                        Ordering::Less => {
                            moon.velocity.$v += 1;
                            other.velocity.$v -= 1
                        }
                        Ordering::Greater => {
                            moon.velocity.$v -= 1;
                            other.velocity.$v += 1
                        }
                        Ordering::Equal => {}
                    }
                };
            }
            apply_gravity!(x);
            apply_gravity!(y);
            apply_gravity!(z);
        }
    }
    for moon in moons {
        moon.position.x += moon.velocity.x;
        moon.position.y += moon.velocity.y;
        moon.position.z += moon.velocity.z;
    }
}

#[derive(Debug, Clone)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

impl Moon {
    fn energy(&self) -> i64 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

#[derive(Debug, Clone)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn from_str(data: &str) -> Self {
        let parts = data[1..(data.len() - 1)]
            .split(", ")
            .map(|s| s[2..].parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }
}
