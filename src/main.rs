use std::time::Instant;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

mod common;
mod y2015;
mod y2016;
mod y2017;
mod y2020;
mod y2021;

fn main() {
    // Read session
    let session = match std::fs::read_to_string(".session") {
        Ok(value) => value,
        Err(_) => {
            println!("Put session key in .session file");
            return;
        }
    };
    let session = session.trim();

    let args = std::env::args().collect::<Vec<_>>();
    let to_run: Vec<Exercise> = match args.len() {
        2 => {
            // Run entire year
            let year = args[1].parse::<u32>().expect("Expected a numerical year");
            (1..=25)
                .filter_map(|day| Exercise::new(day, year))
                .collect()
        }
        3..=usize::MAX => {
            // Run solutions
            let year = args[1].parse::<u32>().expect("Expected a year");
            args[2..]
                .iter()
                .filter_map(|day| {
                    let day = day.parse().expect("Expected a numerical day");
                    Exercise::new(day, year)
                })
                .collect()
        }
        _ => {
            // Print help and exit
            println!(
                r#"
        Advent Of Code Solutions

    Usage: ./program [year] <days...>
"#
            );
            return;
        }
    };

    if to_run.is_empty() {
        println!("No exercises found");
        return;
    }

    for exercise_info in to_run {
        exercise_info.run(&session);
    }
}

#[derive(Debug)]
struct Exercise {
    year: u32,
    day: u32,
}

impl Exercise {
    fn new(day: u32, year: u32) -> Option<Self> {
        Some(Self { day, year })
    }

    fn run(&self, session_key: &str) {
        println!("{} Day {}", self.year, self.day);
        let now = Utc::now();
        let d = NaiveDate::from_ymd(self.year as i32, 12, self.day);
        let t = NaiveTime::from_hms(0, 0, 0);
        let released = NaiveDateTime::new(d, t);
        if now.naive_local() < released {
            println!("Not released yet");
            return;
        }

        let input_path = format!("input/{}/day{}.txt", self.year, self.day);
        let input_path = std::path::Path::new(&input_path);
        let input = std::fs::read_to_string(input_path).unwrap_or_else(|_| {
            println!("Downloading input for {} {}", self.year, self.day);
            let url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                &self.year, &self.day
            );
            let input_response = ureq::get(&url)
                .set("Cookie", &format!("session={}", session_key))
                .call()
                .expect("Error getting day input");
            if input_response.status() != 200 {
                panic!("Server returned error");
            }
            let input = input_response
                .into_string()
                .expect("Error reading response text");
            std::fs::create_dir_all(input_path.parent().expect("Error getting parent dir"))
                .expect("Error creating input dir");
            std::fs::write(input_path, &input).expect("Error writing input");
            input
        });

        macro_rules! run {
            ($year:ident, $day:ident) => {{
                // Hack to stop stack overflowing (but only when using `--release`)
                #[allow(clippy::redundant_closure_call)]
                (|| {
                    let res = $year::$day::solve(&input);
                    (res.0.to_string(), res.1.to_string())
                })()
            }};
        }
        let before = Instant::now();
        let (part_1, part_2) = match (self.year, self.day) {
            (2015, 1) => run!(y2015, day1),
            (2015, 2) => run!(y2015, day2),
            (2015, 3) => run!(y2015, day3),
            (2015, 4) => run!(y2015, day4),
            (2015, 5) => run!(y2015, day5),
            (2015, 6) => run!(y2015, day6),
            (2015, 7) => run!(y2015, day7),
            (2015, 8) => run!(y2015, day8),
            (2015, 9) => run!(y2015, day9),
            (2015, 10) => run!(y2015, day10),
            (2015, 11) => run!(y2015, day11),
            (2015, 12) => run!(y2015, day12),
            (2015, 13) => run!(y2015, day13),
            (2015, 14) => run!(y2015, day14),
            (2015, 15) => run!(y2015, day15),
            (2015, 16) => run!(y2015, day16),
            (2015, 17) => run!(y2015, day17),
            (2015, 18) => run!(y2015, day18),
            (2015, 19) => run!(y2015, day19),
            (2015, 20) => run!(y2015, day20),
            (2015, 21) => run!(y2015, day21),
            (2015, 22) => run!(y2015, day22),
            (2015, 23) => run!(y2015, day23),
            (2015, 24) => run!(y2015, day24),
            (2015, 25) => run!(y2015, day25),
            (2016, 1) => run!(y2016, day1),
            (2016, 2) => run!(y2016, day2),
            (2016, 3) => run!(y2016, day3),
            (2016, 4) => run!(y2016, day4),
            (2016, 5) => run!(y2016, day5),
            (2016, 6) => run!(y2016, day6),
            (2016, 7) => run!(y2016, day7),
            (2016, 8) => run!(y2016, day8),
            (2016, 9) => run!(y2016, day9),
            (2016, 10) => run!(y2016, day10),
            (2016, 11) => run!(y2016, day11),
            (2016, 12) => run!(y2016, day12),
            (2016, 13) => run!(y2016, day13),
            (2016, 14) => run!(y2016, day14),
            (2016, 15) => run!(y2016, day15),
            (2016, 16) => run!(y2016, day16),
            (2016, 17) => run!(y2016, day17),
            (2016, 18) => run!(y2016, day18),
            (2016, 19) => run!(y2016, day19),
            (2016, 20) => run!(y2016, day20),
            (2016, 21) => run!(y2016, day21),
            (2016, 22) => run!(y2016, day22),
            (2016, 23) => run!(y2016, day23),
            (2016, 24) => run!(y2016, day24),
            (2016, 25) => run!(y2016, day25),
            (2017, 1) => run!(y2017, day1),
            (2017, 2) => run!(y2017, day2),
            (2017, 3) => run!(y2017, day3),
            (2017, 4) => run!(y2017, day4),
            (2017, 5) => run!(y2017, day5),
            (2017, 6) => run!(y2017, day6),
            (2017, 7) => run!(y2017, day7),
            (2017, 8) => run!(y2017, day8),
            (2017, 9) => run!(y2017, day9),
            (2017, 10) => run!(y2017, day10),
            (2017, 11) => run!(y2017, day11),
            (2017, 12) => run!(y2017, day12),
            (2017, 13) => run!(y2017, day13),
            (2017, 14) => run!(y2017, day14),
            (2017, 15) => run!(y2017, day15),
            (2017, 16) => run!(y2017, day16),
            (2017, 17) => run!(y2017, day17),
            (2017, 18) => run!(y2017, day18),
            (2017, 19) => run!(y2017, day19),
            (2017, 20) => run!(y2017, day20),
            (2017, 21) => run!(y2017, day21),
            (2017, 22) => run!(y2017, day22),
            (2017, 23) => run!(y2017, day23),
            (2017, 24) => run!(y2017, day24),
            (2017, 25) => run!(y2017, day25),
            (2020, 1) => run!(y2020, day1),
            (2020, 2) => run!(y2020, day2),
            (2020, 3) => run!(y2020, day3),
            (2020, 4) => run!(y2020, day4),
            (2020, 5) => run!(y2020, day5),
            (2020, 6) => run!(y2020, day6),
            (2020, 7) => run!(y2020, day7),
            (2020, 8) => run!(y2020, day8),
            (2020, 9) => run!(y2020, day9),
            (2020, 10) => run!(y2020, day10),
            (2020, 11) => run!(y2020, day11),
            (2020, 12) => run!(y2020, day12),
            (2020, 13) => run!(y2020, day13),
            (2020, 14) => run!(y2020, day14),
            (2020, 15) => run!(y2020, day15),
            (2020, 16) => run!(y2020, day16),
            (2020, 17) => run!(y2020, day17),
            (2020, 18) => run!(y2020, day18),
            (2020, 19) => run!(y2020, day19),
            (2020, 20) => run!(y2020, day20),
            (2020, 21) => run!(y2020, day21),
            (2020, 22) => run!(y2020, day22),
            (2020, 23) => run!(y2020, day23),
            (2020, 24) => run!(y2020, day24),
            (2020, 25) => run!(y2020, day25),
            (2021, 1) => run!(y2021, day1),
            // (2021, 2) => run!(y2021, day2),
            // (2021, 3) => run!(y2021, day3),
            // (2021, 4) => run!(y2021, day4),
            // (2021, 5) => run!(y2021, day5),
            // (2021, 6) => run!(y2021, day6),
            // (2021, 7) => run!(y2021, day7),
            // (2021, 8) => run!(y2021, day8),
            // (2021, 9) => run!(y2021, day9),
            // (2021, 10) => run!(y2021, day10),
            // (2021, 11) => run!(y2021, day11),
            // (2021, 12) => run!(y2021, day12),
            // (2021, 13) => run!(y2021, day13),
            // (2021, 14) => run!(y2021, day14),
            // (2021, 15) => run!(y2021, day15),
            // (2021, 16) => run!(y2021, day16),
            // (2021, 17) => run!(y2021, day17),
            // (2021, 18) => run!(y2021, day18),
            // (2021, 19) => run!(y2021, day19),
            // (2021, 20) => run!(y2021, day20),
            // (2021, 21) => run!(y2021, day21),
            // (2021, 22) => run!(y2021, day22),
            // (2021, 23) => run!(y2021, day23),
            // (2021, 24) => run!(y2021, day24),
            // (2021, 25) => run!(y2021, day25),
            _ => panic!(),
        };
        let time_diff = before.elapsed();

        println!("Part 1:  {}", part_1);
        println!("Part 2:  {}", part_2);
        println!("Elapsed: {:?}", time_diff);
    }
}
