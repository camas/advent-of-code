#![feature(int_roundings)]

use std::{collections::HashMap, io::Write, time::Instant};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use log::{error, info};

mod common;
mod y2015;
mod y2016;
mod y2017;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;

type RunnableClosure = Box<dyn Fn(&str) -> (String, String)>;

fn main() {
    let runners = initialize_runners();

    // Initialize logging
    flexi_logger::Logger::try_with_str("warn, advent_of_code=trace")
        .unwrap()
        .start()
        .unwrap();

    println!();
    println!("        Advent Of Code Solutions");
    println!();

    // Read session
    let session = match std::fs::read_to_string(".session") {
        Ok(value) => value,
        Err(_) => {
            error!("Put session key in .session file");
            return;
        }
    };
    let session = session.trim();

    // Collect args, skipping filename
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();

    if args[0] == "--help" || args[0] == "-h" {
        println!();
        println!("    Usage: ./program [year] <days...>");
        println!();
        return;
    }

    let (year, days) = match args.len() {
        0 => (prompt("Year: "), vec![prompt("Day:  ")]),
        1 => {
            // Only year given. Run all days
            (args.remove(0), vec!["all".to_string()])
        }
        2.. => {
            // Individual numbers given
            if args[1] == "all" {
                (args.remove(0), vec!["all".to_string()])
            } else {
                (args.remove(0), args)
            }
        }
        _ => unreachable!(),
    };

    let year = year.parse::<u32>().unwrap();
    let days = if days.len() == 1 && days[0] == "all" {
        (1..=25).collect::<Vec<_>>()
    } else {
        days.into_iter()
            .map(|d| d.parse::<u32>().unwrap())
            .collect()
    };

    if days.is_empty() {
        error!("No exercises found");
        return;
    }

    for day in days {
        let runner = match runners.get(&(year, day)) {
            Some(runner) => runner,
            None => {
                info!("{} day {} not found. Skipping", year, day);
                continue;
            }
        };
        run_exercise(year, day, session, runner);
    }
}

fn prompt(text: &str) -> String {
    print!("{}", text);
    std::io::stdout().flush().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn run_exercise(year: u32, day: u32, session_key: &str, runner: &RunnableClosure) {
    println!("{} Day {}", year, day);
    let now = Utc::now();
    let d = NaiveDate::from_ymd_opt(year as i32, 12, day).unwrap();
    let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let released = NaiveDateTime::new(d, t);
    if now.naive_local() < released {
        error!("Not released yet");
        return;
    }

    let input_path = format!("input/{}/day{}.txt", year, day);
    let input_path = std::path::Path::new(&input_path);
    let input = std::fs::read_to_string(input_path).unwrap_or_else(|_| {
        println!("Downloading input for {} {}", year, day);
        let url = format!("https://adventofcode.com/{}/day/{}/input", &year, &day);
        let input_response = ureq::get(&url)
            .set("Cookie", &format!("session={}", session_key))
            .call()
            .expect("Error getting day input");
        if input_response.status() != 200 {
            error!("Server returned error");
        }
        let input = input_response
            .into_string()
            .expect("Error reading response text");
        std::fs::create_dir_all(input_path.parent().expect("Error getting parent dir"))
            .expect("Error creating input dir");
        std::fs::write(input_path, &input).expect("Error writing input");
        input
    });

    let before = Instant::now();
    let result = runner(&input);
    let time_diff = before.elapsed();

    println!("Part 1:  {}", result.0);
    println!("Part 2:  {}", result.1);
    println!("Elapsed: {:?}", time_diff);
}

fn initialize_runners() -> HashMap<(u32, u32), RunnableClosure> {
    macro_rules! run_day {
        ($year:ident, $day:ident) => {
            Box::new(|s: &str| {
                let result = $year::$day::solve(s);
                (result.0.to_string(), result.1.to_string())
            })
        };
    }

    let mut runners = HashMap::<(u32, u32), RunnableClosure>::new();
    runners.insert((2015, 1), run_day!(y2015, day1));
    runners.insert((2015, 2), run_day!(y2015, day2));
    runners.insert((2015, 3), run_day!(y2015, day3));
    runners.insert((2015, 4), run_day!(y2015, day4));
    runners.insert((2015, 5), run_day!(y2015, day5));
    runners.insert((2015, 6), run_day!(y2015, day6));
    runners.insert((2015, 7), run_day!(y2015, day7));
    runners.insert((2015, 8), run_day!(y2015, day8));
    runners.insert((2015, 9), run_day!(y2015, day9));
    runners.insert((2015, 10), run_day!(y2015, day10));
    runners.insert((2015, 11), run_day!(y2015, day11));
    runners.insert((2015, 12), run_day!(y2015, day12));
    runners.insert((2015, 13), run_day!(y2015, day13));
    runners.insert((2015, 14), run_day!(y2015, day14));
    runners.insert((2015, 15), run_day!(y2015, day15));
    runners.insert((2015, 16), run_day!(y2015, day16));
    runners.insert((2015, 17), run_day!(y2015, day17));
    runners.insert((2015, 18), run_day!(y2015, day18));
    runners.insert((2015, 19), run_day!(y2015, day19));
    runners.insert((2015, 20), run_day!(y2015, day20));
    runners.insert((2015, 21), run_day!(y2015, day21));
    runners.insert((2015, 22), run_day!(y2015, day22));
    runners.insert((2015, 23), run_day!(y2015, day23));
    runners.insert((2015, 24), run_day!(y2015, day24));
    runners.insert((2015, 25), run_day!(y2015, day25));
    runners.insert((2016, 1), run_day!(y2016, day1));
    runners.insert((2016, 2), run_day!(y2016, day2));
    runners.insert((2016, 3), run_day!(y2016, day3));
    runners.insert((2016, 4), run_day!(y2016, day4));
    runners.insert((2016, 5), run_day!(y2016, day5));
    runners.insert((2016, 6), run_day!(y2016, day6));
    runners.insert((2016, 7), run_day!(y2016, day7));
    runners.insert((2016, 8), run_day!(y2016, day8));
    runners.insert((2016, 9), run_day!(y2016, day9));
    runners.insert((2016, 10), run_day!(y2016, day10));
    runners.insert((2016, 11), run_day!(y2016, day11));
    runners.insert((2016, 12), run_day!(y2016, day12));
    runners.insert((2016, 13), run_day!(y2016, day13));
    runners.insert((2016, 14), run_day!(y2016, day14));
    runners.insert((2016, 15), run_day!(y2016, day15));
    runners.insert((2016, 16), run_day!(y2016, day16));
    runners.insert((2016, 17), run_day!(y2016, day17));
    runners.insert((2016, 18), run_day!(y2016, day18));
    runners.insert((2016, 19), run_day!(y2016, day19));
    runners.insert((2016, 20), run_day!(y2016, day20));
    runners.insert((2016, 21), run_day!(y2016, day21));
    runners.insert((2016, 22), run_day!(y2016, day22));
    runners.insert((2016, 23), run_day!(y2016, day23));
    runners.insert((2016, 24), run_day!(y2016, day24));
    runners.insert((2016, 25), run_day!(y2016, day25));
    runners.insert((2017, 1), run_day!(y2017, day1));
    runners.insert((2017, 2), run_day!(y2017, day2));
    runners.insert((2017, 3), run_day!(y2017, day3));
    runners.insert((2017, 4), run_day!(y2017, day4));
    runners.insert((2017, 5), run_day!(y2017, day5));
    runners.insert((2017, 6), run_day!(y2017, day6));
    runners.insert((2017, 7), run_day!(y2017, day7));
    runners.insert((2017, 8), run_day!(y2017, day8));
    runners.insert((2017, 9), run_day!(y2017, day9));
    runners.insert((2017, 10), run_day!(y2017, day10));
    runners.insert((2017, 11), run_day!(y2017, day11));
    runners.insert((2017, 12), run_day!(y2017, day12));
    runners.insert((2017, 13), run_day!(y2017, day13));
    runners.insert((2017, 14), run_day!(y2017, day14));
    runners.insert((2017, 15), run_day!(y2017, day15));
    runners.insert((2017, 16), run_day!(y2017, day16));
    runners.insert((2017, 17), run_day!(y2017, day17));
    runners.insert((2017, 18), run_day!(y2017, day18));
    runners.insert((2017, 19), run_day!(y2017, day19));
    runners.insert((2017, 20), run_day!(y2017, day20));
    runners.insert((2017, 21), run_day!(y2017, day21));
    runners.insert((2017, 22), run_day!(y2017, day22));
    runners.insert((2017, 23), run_day!(y2017, day23));
    runners.insert((2017, 24), run_day!(y2017, day24));
    runners.insert((2017, 25), run_day!(y2017, day25));
    runners.insert((2019, 1), run_day!(y2019, day1));
    runners.insert((2019, 2), run_day!(y2019, day2));
    runners.insert((2019, 3), run_day!(y2019, day3));
    runners.insert((2019, 4), run_day!(y2019, day4));
    runners.insert((2019, 5), run_day!(y2019, day5));
    runners.insert((2019, 6), run_day!(y2019, day6));
    runners.insert((2019, 7), run_day!(y2019, day7));
    runners.insert((2019, 8), run_day!(y2019, day8));
    runners.insert((2019, 9), run_day!(y2019, day9));
    runners.insert((2019, 10), run_day!(y2019, day10));
    runners.insert((2019, 11), run_day!(y2019, day11));
    runners.insert((2019, 12), run_day!(y2019, day12));
    runners.insert((2019, 13), run_day!(y2019, day13));
    runners.insert((2019, 14), run_day!(y2019, day14));
    // runners.insert((2019, 15), run_day!(y2019, day15));
    // runners.insert((2019, 16), run_day!(y2019, day16));
    // runners.insert((2019, 17), run_day!(y2019, day17));
    // runners.insert((2019, 18), run_day!(y2019, day18));
    // runners.insert((2019, 19), run_day!(y2019, day19));
    // runners.insert((2019, 20), run_day!(y2019, day20));
    // runners.insert((2019, 21), run_day!(y2019, day21));
    // runners.insert((2019, 22), run_day!(y2019, day22));
    // runners.insert((2019, 23), run_day!(y2019, day23));
    // runners.insert((2019, 24), run_day!(y2019, day24));
    // runners.insert((2019, 25), run_day!(y2019, day25));
    runners.insert((2020, 1), run_day!(y2020, day1));
    runners.insert((2020, 2), run_day!(y2020, day2));
    runners.insert((2020, 3), run_day!(y2020, day3));
    runners.insert((2020, 4), run_day!(y2020, day4));
    runners.insert((2020, 5), run_day!(y2020, day5));
    runners.insert((2020, 6), run_day!(y2020, day6));
    runners.insert((2020, 7), run_day!(y2020, day7));
    runners.insert((2020, 8), run_day!(y2020, day8));
    runners.insert((2020, 9), run_day!(y2020, day9));
    runners.insert((2020, 10), run_day!(y2020, day10));
    runners.insert((2020, 11), run_day!(y2020, day11));
    runners.insert((2020, 12), run_day!(y2020, day12));
    runners.insert((2020, 13), run_day!(y2020, day13));
    runners.insert((2020, 14), run_day!(y2020, day14));
    runners.insert((2020, 15), run_day!(y2020, day15));
    runners.insert((2020, 16), run_day!(y2020, day16));
    runners.insert((2020, 17), run_day!(y2020, day17));
    runners.insert((2020, 18), run_day!(y2020, day18));
    runners.insert((2020, 19), run_day!(y2020, day19));
    runners.insert((2020, 20), run_day!(y2020, day20));
    runners.insert((2020, 21), run_day!(y2020, day21));
    runners.insert((2020, 22), run_day!(y2020, day22));
    runners.insert((2020, 23), run_day!(y2020, day23));
    runners.insert((2020, 24), run_day!(y2020, day24));
    runners.insert((2020, 25), run_day!(y2020, day25));
    runners.insert((2021, 1), run_day!(y2021, day1));
    runners.insert((2021, 2), run_day!(y2021, day2));
    runners.insert((2021, 3), run_day!(y2021, day3));
    runners.insert((2021, 4), run_day!(y2021, day4));
    runners.insert((2021, 5), run_day!(y2021, day5));
    runners.insert((2021, 6), run_day!(y2021, day6));
    runners.insert((2021, 7), run_day!(y2021, day7));
    runners.insert((2021, 8), run_day!(y2021, day8));
    runners.insert((2021, 9), run_day!(y2021, day9));
    runners.insert((2021, 10), run_day!(y2021, day10));
    runners.insert((2021, 11), run_day!(y2021, day11));
    runners.insert((2021, 12), run_day!(y2021, day12));
    runners.insert((2021, 13), run_day!(y2021, day13));
    runners.insert((2021, 14), run_day!(y2021, day14));
    runners.insert((2021, 15), run_day!(y2021, day15));
    runners.insert((2021, 16), run_day!(y2021, day16));
    runners.insert((2021, 17), run_day!(y2021, day17));
    runners.insert((2021, 18), run_day!(y2021, day18));
    runners.insert((2021, 19), run_day!(y2021, day19));
    runners.insert((2021, 20), run_day!(y2021, day20));
    runners.insert((2021, 21), run_day!(y2021, day21));
    runners.insert((2021, 22), run_day!(y2021, day22));
    runners.insert((2021, 23), run_day!(y2021, day23));
    runners.insert((2021, 24), run_day!(y2021, day24));
    runners.insert((2021, 25), run_day!(y2021, day25));
    runners.insert((2022, 1), run_day!(y2022, day1));
    runners.insert((2022, 2), run_day!(y2022, day2));
    runners.insert((2022, 3), run_day!(y2022, day3));
    runners.insert((2022, 4), run_day!(y2022, day4));
    runners.insert((2022, 5), run_day!(y2022, day5));
    runners.insert((2022, 6), run_day!(y2022, day6));
    runners.insert((2022, 7), run_day!(y2022, day7));
    runners.insert((2022, 8), run_day!(y2022, day8));
    runners.insert((2022, 9), run_day!(y2022, day9));
    runners.insert((2022, 10), run_day!(y2022, day10));
    runners.insert((2022, 11), run_day!(y2022, day11));
    runners.insert((2022, 12), run_day!(y2022, day12));
    runners.insert((2022, 13), run_day!(y2022, day13));
    runners.insert((2022, 14), run_day!(y2022, day14));
    runners.insert((2022, 15), run_day!(y2022, day15));
    runners.insert((2022, 16), run_day!(y2022, day16));
    runners.insert((2022, 17), run_day!(y2022, day17));
    runners.insert((2022, 18), run_day!(y2022, day18));
    runners.insert((2022, 19), run_day!(y2022, day19));
    runners.insert((2022, 20), run_day!(y2022, day20));
    runners.insert((2022, 21), run_day!(y2022, day21));
    runners.insert((2022, 22), run_day!(y2022, day22));
    runners.insert((2022, 23), run_day!(y2022, day23));
    runners.insert((2022, 24), run_day!(y2022, day24));
    runners.insert((2022, 25), run_day!(y2022, day25));
    runners.insert((2023, 1), run_day!(y2023, day1));
    runners.insert((2023, 2), run_day!(y2023, day2));
    // runners.insert((2023, 3), run_day!(y2023, day3));
    // runners.insert((2023, 4), run_day!(y2023, day4));
    // runners.insert((2023, 5), run_day!(y2023, day5));
    // runners.insert((2023, 6), run_day!(y2023, day6));
    // runners.insert((2023, 7), run_day!(y2023, day7));
    // runners.insert((2023, 8), run_day!(y2023, day8));
    // runners.insert((2023, 9), run_day!(y2023, day9));
    // runners.insert((2023, 10), run_day!(y2023, day10));
    // runners.insert((2023, 11), run_day!(y2023, day11));
    // runners.insert((2023, 12), run_day!(y2023, day12));
    // runners.insert((2023, 13), run_day!(y2023, day13));
    // runners.insert((2023, 14), run_day!(y2023, day14));
    // runners.insert((2023, 15), run_day!(y2023, day15));
    // runners.insert((2023, 16), run_day!(y2023, day16));
    // runners.insert((2023, 17), run_day!(y2023, day17));
    // runners.insert((2023, 18), run_day!(y2023, day18));
    // runners.insert((2023, 19), run_day!(y2023, day19));
    // runners.insert((2023, 20), run_day!(y2023, day20));
    // runners.insert((2023, 21), run_day!(y2023, day21));
    // runners.insert((2023, 22), run_day!(y2023, day22));
    // runners.insert((2023, 23), run_day!(y2023, day23));
    // runners.insert((2023, 24), run_day!(y2023, day24));
    // runners.insert((2023, 25), run_day!(y2023, day25));
    runners
}
