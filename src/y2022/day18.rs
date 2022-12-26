use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    str::FromStr,
};

const ADJACENT_DIRECTIONS: [Vector3; 6] = [
    Vector3::new(-1, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(0, 0, -1),
    Vector3::new(0, 0, 1),
];

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let scan = input.trim().parse::<Scan>().unwrap();

    let part1 = scan
        .all_adjacent_pixels()
        .filter(|pixel| !scan.pixels.contains(pixel))
        .count();

    let part2 = scan.exterior_surface_size();

    (part1, part2)
}

#[derive(Debug)]
struct Scan {
    pixels: HashSet<Vector3>,
}

impl Scan {
    fn all_adjacent_pixels(&self) -> impl Iterator<Item = Vector3> + '_ {
        self.pixels
            .iter()
            .flat_map(|pixel| self.adjacent_pixels(*pixel))
    }

    fn adjacent_pixels(&self, position: Vector3) -> impl Iterator<Item = Vector3> + '_ {
        ADJACENT_DIRECTIONS.iter().map(move |dir| position + *dir)
    }

    fn exterior_surface_size(&self) -> usize {
        let mut all_adjacent_pixels = HashSet::new();
        let mut adjacent_pixel_map = self
            .pixels
            .iter()
            .map(|pixel| {
                let adjacent = self
                    .adjacent_pixels(*pixel)
                    .filter(|p| !self.pixels.contains(p))
                    .collect::<HashSet<_>>();
                all_adjacent_pixels.extend(adjacent.iter().cloned());
                (*pixel, adjacent)
            })
            .collect::<HashMap<_, _>>();

        let x_min = self.pixels.iter().map(|p| p.x).min().unwrap();
        let x_max = self.pixels.iter().map(|p| p.x).max().unwrap();
        let y_min = self.pixels.iter().map(|p| p.y).min().unwrap();
        let y_max = self.pixels.iter().map(|p| p.y).max().unwrap();
        let z_min = self.pixels.iter().map(|p| p.z).min().unwrap();
        let z_max = self.pixels.iter().map(|p| p.z).max().unwrap();

        'outer: for adjacent_pixel in all_adjacent_pixels.into_iter() {
            let mut seen = HashSet::new();
            let mut queue = vec![adjacent_pixel];

            while let Some(pixel) = queue.pop() {
                if !seen.insert(pixel) {
                    continue;
                }

                if pixel.x <= x_min
                    || pixel.x >= x_max
                    || pixel.y <= y_min
                    || pixel.y >= y_max
                    || pixel.z <= z_min
                    || pixel.z >= z_max
                {
                    continue 'outer;
                }

                queue.extend(
                    self.adjacent_pixels(pixel)
                        .filter(|p| !self.pixels.contains(p)),
                )
            }

            // No way out found. Inside drop
            adjacent_pixel_map.values_mut().for_each(|v| {
                v.remove(&adjacent_pixel);
            });
        }

        adjacent_pixel_map.values().map(|v| v.len()).sum::<usize>()
    }
}

impl FromStr for Scan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .trim()
            .lines()
            .map(|l| {
                let parts = l.split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>();
                Vector3::new(parts[0], parts[1], parts[2])
            })
            .collect();

        Ok(Scan { pixels })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    const fn new(x: i64, y: i64, z: i64) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 64.to_string());
        assert_eq!(result.1.to_string(), 58.to_string());
    }
}
