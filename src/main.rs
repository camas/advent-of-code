use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use advent_of_code::{get_solution, Day, Solution, Year};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use log::{error, info};

fn main() -> Result<()> {
    // Initialize logging
    flexi_logger::Logger::try_with_str("warn, advent_of_code=trace")?.start()?;

    println!();
    println!("        Advent Of Code Solutions");
    println!();

    let session = read_session()?;

    // skip filename
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!();
        println!("    Usage: ./program [year] <days...>");
        println!();
        return Ok(());
    }

    let (year, days) = match args.len() {
        0 => (prompt("Year: ")?, vec![prompt("Day:  ")?]),
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
    };

    let year = year.parse::<i64>()?;
    let days = if days.len() == 1 && days[0] == "all" {
        (1..=25).collect::<Vec<_>>()
    } else {
        days.into_iter()
            .map(|d| d.parse::<u8>())
            .collect::<std::result::Result<Vec<u8>, _>>()?
    };

    if days.is_empty() {
        error!("No exercises found");
        return Ok(());
    }

    for day in days {
        let Some(solution) = get_solution(Year::new(year), Day::new(day)) else {
            info!("{} day {} not found. Skipping", year, day);
            continue;
        };
        run_solution(solution, &session)?;
    }

    Ok(())
}

fn prompt(text: &str) -> Result<String> {
    print!("{}", text);
    std::io::stdout().flush()?;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn read_session() -> Result<String> {
    Ok(std::fs::read_to_string(".session")
        .context("Couldn't read session file. Is there a session key in .session?")?
        .trim()
        .to_string())
}

fn run_solution(solution: &Solution, session_key: &str) -> Result<()> {
    println!("{} Day {}", solution.year, solution.day);
    if Utc::now().naive_utc().date() < solution.date_released() {
        error!("Not released yet");
        return Ok(());
    }

    let input_path = PathBuf::from(format!("input/{}/day{}.txt", solution.year, solution.day));
    let input = match std::fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(_) => download_input(&input_path, solution, session_key)?,
    };

    let before = Instant::now();
    let result = solution.run(&input);
    let time_diff = before.elapsed();

    println!("Part 1:  {}", result.0);
    println!("Part 2:  {}", result.1);
    println!("Elapsed: {:?}", time_diff);

    Ok(())
}

fn download_input(input_path: &Path, solution: &Solution, session_key: &str) -> Result<String> {
    println!("Downloading input for {} {}", solution.year, solution.day);

    let input_url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        solution.year, solution.day
    );
    let mut input_response = ureq::get(&input_url)
        .header("Cookie", &format!("session={}", session_key))
        .call()
        .context("Error getting day input")?;
    if input_response.status() != 200 {
        return Err(anyhow!("Server returned error"));
    }

    let input = input_response
        .body_mut()
        .read_to_string()
        .context("Error reading response text")?;

    std::fs::create_dir_all(input_path.parent().context("Error getting parent dir")?)
        .context("Error creating input dir")?;
    std::fs::write(input_path, &input).context("Error writing input")?;

    Ok(input)
}
