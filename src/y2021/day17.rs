pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let (xrange, yrange) = input
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();
    let (x_min, x_max) = xrange.split_once("..").unwrap();
    let (y_min, y_max) = yrange.split_once("..").unwrap();
    let (x_min, x_max) = (x_min.parse::<i64>().unwrap(), x_max.parse::<i64>().unwrap());
    let (y_min, y_max) = (y_min.parse::<i64>().unwrap(), y_max.parse::<i64>().unwrap());

    // Find minimum x velocity needed to reach target
    let mut best = i64::MIN;
    let mut count = 0;
    for initial_x_vel in 0..=x_max {
        for initial_y_vel in y_min..=(x_max * 10) {
            let mut x = 0;
            let mut y = 0;
            let mut x_vel = initial_x_vel;
            let mut y_vel = initial_y_vel;
            let mut highest = i64::MIN;
            let mut found = false;
            loop {
                x += x_vel;
                y += y_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
                if y > highest {
                    highest = y;
                }
                if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                    found = true;
                    if highest > best {
                        best = highest;
                    }
                }
                if x > x_max || (y_vel <= 0 && y < y_min) || (x < x_min && x_vel == 0) {
                    break;
                }
            }
            if found {
                count += 1;
            }
        }
    }

    (best, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"target area: x=20..30, y=-10..-5";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "45");
        assert_eq!(part2.to_string(), "112");
    }
}
