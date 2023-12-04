#![feature(int_roundings)]

use std::fmt::Display;

use chrono::NaiveDate;

mod common;
macros::mod_years!();

macros::solutions!();

pub struct Solution {
    pub year: Year,
    pub day: Day,
}

impl Solution {
    pub fn date_released(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year.0 as i32, 12, self.day.0 as u32).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Year(i64);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Day(u8);

pub fn get_solution(year: Year, day: Day) -> Option<&'static Solution> {
    SOLUTIONS
        .iter()
        .find(|solution| solution.year == year && solution.day == day)
}

impl Year {
    pub const fn new(year: i64) -> Year {
        Year(year)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Day {
    pub const fn new(day: u8) -> Day {
        assert!(day >= 1 && day <= 25);
        Day(day)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
