#[allow(clippy::needless_range_loop)]
pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lights = [[false; 1000]; 1000];
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        if parts[0] == "toggle" {
            let mut start = parts[1].split(',').map(|val| val.parse::<usize>().unwrap());
            let (start_x, start_y) = (start.next().unwrap(), start.next().unwrap());
            let mut end = parts[3].split(',').map(|val| val.parse::<usize>().unwrap());
            let (end_x, end_y) = (end.next().unwrap(), end.next().unwrap());

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lights[y][x] = !lights[y][x];
                }
            }
        } else if parts[0] == "turn" && parts[1] == "off" {
            let mut start = parts[2].split(',').map(|val| val.parse::<usize>().unwrap());
            let (start_x, start_y) = (start.next().unwrap(), start.next().unwrap());
            let mut end = parts[4].split(',').map(|val| val.parse::<usize>().unwrap());
            let (end_x, end_y) = (end.next().unwrap(), end.next().unwrap());

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lights[y][x] = false;
                }
            }
        } else if parts[0] == "turn" && parts[1] == "on" {
            let mut start = parts[2].split(',').map(|val| val.parse::<usize>().unwrap());
            let (start_x, start_y) = (start.next().unwrap(), start.next().unwrap());
            let mut end = parts[4].split(',').map(|val| val.parse::<usize>().unwrap());
            let (end_x, end_y) = (end.next().unwrap(), end.next().unwrap());

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lights[y][x] = true;
                }
            }
        }
    }

    let part1 = lights.iter().fold(0, |count, row| {
        count + row.iter().filter(|&&val| val).count()
    });

    let mut lights = [[0_u32; 1000]; 1000];
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        if parts[0] == "toggle" {
            let mut start = parts[1].split(',').map(|val| val.parse::<usize>().unwrap());
            let (start_x, start_y) = (start.next().unwrap(), start.next().unwrap());
            let mut end = parts[3].split(',').map(|val| val.parse::<usize>().unwrap());
            let (end_x, end_y) = (end.next().unwrap(), end.next().unwrap());

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lights[y][x] += 2;
                }
            }
        } else if parts[0] == "turn" && parts[1] == "off" {
            let mut start = parts[2].split(',').map(|val| val.parse::<usize>().unwrap());
            let (start_x, start_y) = (start.next().unwrap(), start.next().unwrap());
            let mut end = parts[4].split(',').map(|val| val.parse::<usize>().unwrap());
            let (end_x, end_y) = (end.next().unwrap(), end.next().unwrap());

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    if lights[y][x] > 0 {
                        lights[y][x] -= 1;
                    }
                }
            }
        } else if parts[0] == "turn" && parts[1] == "on" {
            let mut start = parts[2].split(',').map(|val| val.parse::<usize>().unwrap());
            let (start_x, start_y) = (start.next().unwrap(), start.next().unwrap());
            let mut end = parts[4].split(',').map(|val| val.parse::<usize>().unwrap());
            let (end_x, end_y) = (end.next().unwrap(), end.next().unwrap());

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    lights[y][x] += 1;
                }
            }
        }
    }

    let part2 = lights
        .iter()
        .fold(0_u32, |count, row| count + row.iter().sum::<u32>());

    (part1, part2)
}
