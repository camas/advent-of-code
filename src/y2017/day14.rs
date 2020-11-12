use crate::Exercise;

pub struct Day14;

impl Exercise for Day14 {
    fn part1(&self, input: &str) -> String {
        let key = input.trim();
        (0..128)
            .map(|num| {
                let row_string = format!("{}-{}", key, num);
                let hash = crate::y2017::day10::knot_hash(
                    &row_string.chars().map(|c| c as usize).collect::<Vec<_>>(),
                );
                hash.into_iter().map(|byte| byte.count_ones()).sum::<u32>()
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let key = input.trim();
        let used = (0..128)
            .map(|num| {
                let row_string = format!("{}-{}", key, num);
                let hash = crate::y2017::day10::knot_hash(
                    &row_string.chars().map(|c| c as usize).collect::<Vec<_>>(),
                );
                hash.into_iter()
                    .flat_map(|byte| (0..8).rev().map(move |shift| byte & (1 << shift) > 0))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut groups = [[0; 128]; 128];
        let mut cur_group = 0;

        for y in 0..128 {
            for x in 0..128 {
                // Continue if not a used square or already in a group
                if !used[y][x] || groups[y][x] > 0 {
                    continue;
                }

                cur_group += 1;
                let mut queue = vec![(y as i32, x as i32)];
                while !queue.is_empty() {
                    let (test_y, test_x) = queue.pop().unwrap();
                    if test_x < 0 || test_y < 0 || test_x >= 128 || test_y >= 128 {
                        continue;
                    }
                    if !used[test_y as usize][test_x as usize] {
                        continue;
                    }
                    if groups[test_y as usize][test_x as usize] > 0 {
                        continue;
                    }
                    groups[test_y as usize][test_x as usize] = cur_group;
                    queue.push((test_y, test_x + 1));
                    queue.push((test_y, test_x - 1));
                    queue.push((test_y + 1, test_x));
                    queue.push((test_y - 1, test_x));
                }
            }
        }

        cur_group.to_string()
    }
}
