use std::{num::ParseIntError, str::FromStr};

use crate::Exercise;

pub struct Day2 {}

impl Exercise for Day2 {
    fn part1(&self, input: &str) -> String {
        let boxes = input
            .lines()
            .map(|line| line.parse::<Dimensions>().unwrap())
            .collect::<Vec<_>>();

        boxes
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
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let boxes = input
            .lines()
            .map(|line| line.parse::<Dimensions>().unwrap())
            .collect::<Vec<_>>();

        boxes
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
            .sum::<u32>()
            .to_string()
    }
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
