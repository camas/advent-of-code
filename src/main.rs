mod y2015;
mod y2017;

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
    let to_run: Vec<ExerciseInfo> = match args.len() {
        2 => {
            // Run entire year
            let year = args[1].parse::<u32>().expect("Expected a numerical year");
            (1..=25).filter_map(|day| get_day(year, day)).collect()
        }
        3..=usize::MAX => {
            // Run solutions
            let year = args[1].parse::<u32>().expect("Expected a year");
            args[2..]
                .iter()
                .filter_map(|day| {
                    let day = day.parse().expect("Expected a numerical day");
                    get_day(year, day)
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

fn get_day(year: u32, day: u32) -> Option<ExerciseInfo> {
    let exercise: Box<dyn Exercise> = match (year, day) {
        (2015, 1) => Box::new(y2015::day1::Day1 {}),
        (2015, 2) => Box::new(y2015::day2::Day2 {}),
        (2015, 3) => Box::new(y2015::day3::Day3 {}),
        (2015, 4) => Box::new(y2015::day4::Day4 {}),
        (2015, 5) => Box::new(y2015::day5::Day5 {}),
        (2015, 6) => Box::new(y2015::day6::Day6 {}),
        (2015, 7) => Box::new(y2015::day7::Day7 {}),
        (2015, 8) => Box::new(y2015::day8::Day8 {}),
        (2015, 9) => Box::new(y2015::day9::Day9 {}),
        (2015, 10) => Box::new(y2015::day10::Day10 {}),
        (2015, 11) => Box::new(y2015::day11::Day11 {}),
        (2015, 12) => Box::new(y2015::day12::Day12 {}),
        (2015, 13) => Box::new(y2015::day13::Day13 {}),
        (2015, 14) => Box::new(y2015::day14::Day14 {}),
        (2015, 15) => Box::new(y2015::day15::Day15 {}),
        (2015, 16) => Box::new(y2015::day16::Day16 {}),
        (2015, 17) => Box::new(y2015::day17::Day17 {}),
        (2015, 18) => Box::new(y2015::day18::Day18 {}),
        (2015, 19) => Box::new(y2015::day19::Day19 {}),
        (2015, 20) => Box::new(y2015::day20::Day20 {}),
        (2015, 21) => Box::new(y2015::day21::Day21 {}),
        (2015, 22) => Box::new(y2015::day22::Day22 {}),
        (2015, 23) => Box::new(y2015::day23::Day23 {}),
        (2015, 24) => Box::new(y2015::day24::Day24 {}),
        (2015, 25) => Box::new(y2015::day25::Day25 {}),
        (2017, 1) => Box::new(y2017::day1::Day1 {}),
        (2017, 2) => Box::new(y2017::day2::Day2 {}),
        (2017, 3) => Box::new(y2017::day3::Day3 {}),
        (2017, 4) => Box::new(y2017::day4::Day4 {}),
        (2017, 5) => Box::new(y2017::day5::Day5 {}),
        (2017, 6) => Box::new(y2017::day6::Day6 {}),
        (2017, 7) => Box::new(y2017::day7::Day7 {}),
        (2017, 8) => Box::new(y2017::day8::Day8 {}),
        (2017, 9) => Box::new(y2017::day9::Day9 {}),
        (2017, 10) => Box::new(y2017::day10::Day10 {}),
        (2017, 11) => Box::new(y2017::day11::Day11 {}),
        (2017, 12) => Box::new(y2017::day12::Day12 {}),
        (2017, 13) => Box::new(y2017::day13::Day13 {}),
        (2017, 14) => Box::new(y2017::day14::Day14 {}),
        (2017, 15) => Box::new(y2017::day15::Day15 {}),
        (2017, 16) => Box::new(y2017::day16::Day16 {}),
        (2017, 17) => Box::new(y2017::day17::Day17 {}),
        (2017, 18) => Box::new(y2017::day18::Day18 {}),
        (2017, 19) => Box::new(y2017::day19::Day19 {}),
        (2017, 20) => Box::new(y2017::day20::Day20 {}),
        (2017, 21) => Box::new(y2017::day21::Day21 {}),
        (2017, 22) => Box::new(y2017::day22::Day22 {}),
        (2017, 23) => Box::new(y2017::day23::Day23 {}),
        (2017, 24) => Box::new(y2017::day24::Day24 {}),
        (2017, 25) => Box::new(y2017::day25::Day25 {}),
        _ => return None,
    };
    Some(ExerciseInfo {
        year,
        day,
        exercise,
    })
}

trait Exercise {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

struct ExerciseInfo {
    year: u32,
    day: u32,
    exercise: Box<dyn Exercise>,
}

impl ExerciseInfo {
    fn run(&self, session_key: &str) {
        println!("{} Day {}", self.year, self.day);

        let input_path = format!("input/{}/day{}.txt", self.year, self.day);
        let input_path = std::path::Path::new(&input_path);
        let input = std::fs::read_to_string(input_path).unwrap_or_else(|_| {
            println!("Downloading input for {} {}", self.year, self.day);
            let url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                &self.year, &self.day
            );
            let client = reqwest::blocking::Client::new();
            let input = client
                .get(&url)
                .header("Cookie", format!("session={}", session_key))
                .send()
                .expect("Error getting day input")
                .text()
                .expect("Error reading response text");
            std::fs::create_dir_all(input_path.parent().expect("Error getting parent dir"))
                .expect("Error creating input dir");
            std::fs::write(input_path, &input).expect("Error writing input");
            input
        });

        println!("Part 1: {}", self.exercise.part1(&input));
        println!("Part 2: {}", self.exercise.part2(&input));
    }
}
