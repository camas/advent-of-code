use std::collections::HashSet;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let heightmap = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();
    let width = heightmap[0].len();
    let height = heightmap.len();

    let mut part1 = 0;
    let mut basin_sizes = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let value = heightmap[y][x];
            if x > 0 && heightmap[y][x - 1] <= value {
                continue;
            }
            if x < width - 1 && heightmap[y][x + 1] <= value {
                continue;
            }
            if y > 0 && heightmap[y - 1][x] <= value {
                continue;
            }
            if y < height - 1 && heightmap[y + 1][x] <= value {
                continue;
            }
            part1 += value + 1;

            let mut queue = vec![(x, y)];
            let mut seen = HashSet::new();
            while !queue.is_empty() {
                let curr = queue.pop().unwrap();
                if seen.contains(&curr) {
                    continue;
                }
                if heightmap[curr.1][curr.0] == 9 {
                    continue;
                }
                seen.insert(curr);
                if curr.0 > 0 {
                    queue.push((curr.0 - 1, curr.1));
                }
                if curr.0 < width - 1 {
                    queue.push((curr.0 + 1, curr.1));
                }
                if curr.1 > 0 {
                    queue.push((curr.0, curr.1 - 1));
                }
                if curr.1 < height - 1 {
                    queue.push((curr.0, curr.1 + 1));
                }
            }
            basin_sizes.push(seen.len());
        }
    }

    basin_sizes.sort_unstable();
    let part2 = basin_sizes[basin_sizes.len() - 3..]
        .iter()
        .product::<usize>();

    (part1, part2)
}
