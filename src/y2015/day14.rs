use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let deers = input
        .lines()
        .map(|line| line.parse::<Deer>().unwrap())
        .collect::<Vec<_>>();
    let part1 = deers
        .iter()
        .map(|deer| deer.distance_at(2503))
        .max()
        .unwrap();

    let deers = input
        .lines()
        .map(|line| line.parse::<Deer>().unwrap())
        .collect::<Vec<_>>();
    let mut scores = vec![0; deers.len()];
    for time in 1..=2503 {
        let mut cur_best = 0;
        let mut cur_best_deers = Vec::new();
        for (i, deer) in deers.iter().enumerate() {
            let dist = deer.distance_at(time);
            match dist.cmp(&cur_best) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => cur_best_deers.push(i),
                std::cmp::Ordering::Greater => {
                    cur_best = dist;
                    cur_best_deers = vec![i];
                }
            }
        }
        cur_best_deers.into_iter().for_each(|i| scores[i] += 1);
    }
    let part2 = *scores.iter().max().unwrap();

    (part1, part2)
}

struct Deer {
    _name: String,
    speed: u32,
    speed_time: u32,
    rest_time: u32,
}

impl Deer {
    pub fn distance_at(&self, time: u32) -> u32 {
        let times_cycled = time / (self.speed_time + self.rest_time);
        let remaining = time % (self.speed_time + self.rest_time);
        let remaining = remaining.min(self.speed_time);
        (times_cycled * self.speed_time * self.speed) + (remaining * self.speed)
    }
}

impl FromStr for Deer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let name = parts[0].to_string();
        let speed = parts[3].parse()?;
        let speed_time = parts[6].parse()?;
        let rest_time = parts[13].parse()?;
        Ok(Self {
            _name: name,
            speed,
            speed_time,
            rest_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_known() {
        let comet = Deer {
            _name: "Comet".to_string(),
            speed: 14,
            speed_time: 10,
            rest_time: 127,
        };
        assert_eq!(comet.distance_at(1000), 1120);
    }
}
