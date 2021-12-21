use std::collections::HashSet;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm.chars().map(|c| c == '#').collect::<Vec<_>>();
    let mut image = Image::from_str(image);

    for _ in 0..2 {
        image.enhance(&algorithm);
    }
    let part1 = image.pixels.len();
    for _ in 0..(50 - 2) {
        image.enhance(&algorithm);
    }
    let part2 = image.pixels.len();

    (part1, part2)
}

#[derive(Debug, Clone)]
struct Image {
    pixels: HashSet<(i64, i64)>,
    default_pixel: bool,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Image {
    fn from_str(data: &str) -> Self {
        let pixels = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .map(|c| c == '#')
                    .enumerate()
                    .filter_map(move |(x, b)| if b { Some((x as i64, y as i64)) } else { None })
            })
            .flatten()
            .collect::<HashSet<_>>();
        let min_x = pixels.iter().map(|&(x, _)| x).min().unwrap();
        let max_x = pixels.iter().map(|&(x, _)| x).max().unwrap();
        let min_y = pixels.iter().map(|&(_, y)| y).min().unwrap();
        let max_y = pixels.iter().map(|&(_, y)| y).max().unwrap();
        Self {
            pixels,
            default_pixel: false,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn enhance(&mut self, algo: &[bool]) {
        let mut new_pixels = HashSet::new();
        let next_default = if self.default_pixel {
            algo[0b111111111]
        } else {
            algo[0b0]
        };
        for y in (self.min_y - 1)..=(self.max_y + 1) {
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                let mut n = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let pixel = self.pixels.contains(&(x + dx, y + dy)) ^ self.default_pixel;
                        n <<= 1;
                        n |= pixel as i64;
                    }
                }
                if algo[n as usize] ^ next_default {
                    new_pixels.insert((x, y));
                }
            }
        }
        self.min_x -= 1;
        self.min_y -= 1;
        self.max_x += 1;
        self.max_y += 1;
        self.default_pixel = next_default;
        self.pixels = new_pixels;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "35");
        assert_eq!(part2.to_string(), "3351");
    }
}
