use crate::Exercise;

pub struct Day13;

impl Exercise for Day13 {
    fn part1(&self, input: &str) -> String {
        let scanners = input
            .lines()
            .map(|line| {
                let parts = line.split(": ").collect::<Vec<_>>();
                let depth = parts[0].parse::<u32>().unwrap();
                let range = parts[1].parse::<u32>().unwrap();
                (depth, range)
            })
            .collect::<Vec<_>>();

        let severity = scanners
            .iter()
            .map(|(depth, range)| {
                if depth % (2 * range - 2) == 0 {
                    depth * range
                } else {
                    0
                }
            })
            .sum::<u32>();
        severity.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let scanners = input
            .lines()
            .map(|line| {
                let parts = line.split(": ").collect::<Vec<_>>();
                let depth = parts[0].parse::<u32>().unwrap();
                let range = parts[1].parse::<u32>().unwrap();
                (depth, range)
            })
            .collect::<Vec<_>>();

        for wait in 0.. {
            if scanners
                .iter()
                .find(|(depth, range)| (wait + depth) % (2 * range - 2) == 0)
                .is_none()
            {
                return wait.to_string();
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(Day13 {}.part2(input), "10");
    }
}
