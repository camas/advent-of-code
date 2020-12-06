use std::{ops::AddAssign, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut particles = input
        .lines()
        .map(|line| line.parse::<Particle>().unwrap())
        .collect::<Vec<_>>();

    let part1 = particles
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.acceleration.square_product())
        .unwrap()
        .0;

    let mut steps_without_collision = 0;
    loop {
        steps_without_collision += 1;
        if steps_without_collision > 10_000 {
            break;
        }

        // Move all particles
        for particle in particles.iter_mut() {
            particle.velocity += &particle.acceleration;
            particle.position += &particle.velocity;
        }

        // Remove particles in same space
        let to_remove = particles
            .iter()
            .enumerate()
            .filter(|(i, particle)| {
                particles.iter().enumerate().any(|(j, other)| {
                    if *i == j {
                        return false;
                    }
                    particle.position == other.position
                })
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if !to_remove.is_empty() {
            steps_without_collision = 0;
            particles = particles
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !to_remove.contains(i))
                .map(|(_, p)| p)
                .collect();
        }
    }
    let part2 = particles.len();

    (part1, part2)
}

#[derive(Debug, PartialEq)]
struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
}

impl FromStr for Particle {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(", ").collect::<Vec<_>>();
        let position = parts[0][2..].parse()?;
        let velocity = parts[1][2..].parse()?;
        let acceleration = parts[2][2..].parse()?;
        Ok(Self {
            position,
            velocity,
            acceleration,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    fn square_product(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl FromStr for Vector3 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s[1..(s.len() - 1)].split(',').collect::<Vec<_>>();
        let x = parts[0].parse()?;
        let y = parts[1].parse()?;
        let z = parts[2].parse()?;
        Ok(Self { x, y, z })
    }
}

impl AddAssign<&Self> for Vector3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
