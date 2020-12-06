use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let boxes = input
        .lines()
        .map(|line| line.parse::<Dimensions>().unwrap())
        .collect::<Vec<_>>();

    let part_1 = boxes
        .iter()
        .map(|b| {
            let surface_area = b.surface_area();
            let slack = if b.length < b.width {
                if b.width < b.height {
                    b.length * b.width
                } else {
                    b.length * b.height
                }
            } else if b.length < b.height {
                b.width * b.length
            } else {
                b.width * b.height
            };
            surface_area + slack
        })
        .sum::<u32>();

    let part_2 = boxes
        .iter()
        .map(|b| {
            let volume = b.volume();
            let bow = if b.length < b.width {
                if b.width < b.height {
                    2 * (b.length + b.width)
                } else {
                    2 * (b.length + b.height)
                }
            } else if b.length < b.height {
                2 * (b.width + b.length)
            } else {
                2 * (b.width + b.height)
            };
            volume + bow
        })
        .sum::<u32>();

    (part_1, part_2)
}

struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimensions {
    fn surface_area(&self) -> u32 {
        2 * (self.length * self.width + self.length * self.height + self.width * self.height)
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

impl FromStr for Dimensions {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split('x')
            .map(|sub| sub.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Dimensions {
            length: nums[0],
            width: nums[1],
            height: nums[2],
        })
    }
}
