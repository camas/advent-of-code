use std::collections::HashMap;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Clever answer
    let target = input.trim().parse::<i32>().unwrap();
    let n = ((target as f64).sqrt() - 1.) / 2.;
    let n = n.ceil() as i32;
    let level_min = 4 * (n - 1) * (n - 1) + 4 * (n - 1) + 1;
    let quad_offset = (target - level_min) % (2 * n);
    let answer = n + (quad_offset - n).abs();
    let part1 = answer;

    let mut values: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    values.entry(0).or_insert_with(HashMap::new).insert(0, 1);
    let mut x = 0;
    let mut y = 0;
    for n in 1.. {
        x += 1;
        y += 1;
        for direction in &[(0, -1), (-1, 0), (0, 1), (1, 0)] {
            for _step in 0..(2 * n) {
                x += direction.0;
                y += direction.1;
                let value: i32 = (-1..=1)
                    .map(|y_off| {
                        let check_y = y + y_off;
                        (-1..=1)
                            .map(|x_off| {
                                let check_x = x + x_off;
                                if check_x == x && check_y == y {
                                    0
                                } else {
                                    *values
                                        .entry(check_y)
                                        .or_insert_with(HashMap::new)
                                        .get(&check_x)
                                        .unwrap_or(&0)
                                }
                            })
                            .sum::<i32>()
                    })
                    .sum();
                if value > target {
                    return (part1, value);
                }
                values
                    .entry(y)
                    .or_insert_with(HashMap::new)
                    .insert(x, value);
            }
        }
    }
    unreachable!()
}
