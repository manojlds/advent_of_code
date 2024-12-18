use std::env;
use std::time::Instant;
mod days;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <part>");
        return Ok(());
    }

    let day = &args[1];
    let part = &args[2];

    let start = Instant::now();

    let result = match (day.as_str(), part.as_str()) {
        ("day1", "part1") => days::day1::part1::solve(),
        ("day1", "part2") => days::day1::part2::solve(),
        ("day2", "part1") => days::day2::part1::solve(),
        ("day2", "part1_optimized") => days::day2::part1_optimized::solve(),
        ("day2", "part2") => days::day2::part2::solve(),
        ("day2", "part2_optimized") => days::day2::part2_optimized::solve(),
        ("day3", "part1") => days::day3::part1::solve(),
        ("day3", "part2") => days::day3::part2::solve(),
        ("day4", "part1") => days::day4::part1::solve(),
        ("day4", "part2") => days::day4::part2::solve(),
        ("day5", "part1") => days::day5::part1::solve(),
        ("day5", "part2") => days::day5::part2::solve(),
        ("day6", "part1") => days::day6::part1::solve(),
        ("day6", "part2") => days::day6::part2::solve(),
        ("day7", "part1") => days::day7::part1::solve(),
        ("day7", "part2") => days::day7::part2::solve(),
        ("day8", "part1") => days::day8::part1::solve(),
        ("day8", "part2") => days::day8::part2::solve(),
        ("day9", "part1") => days::day9::part1::solve(),
        ("day9", "part2") => days::day9::part2::solve(),
        ("day10", "part1") => days::day10::part1::solve(),
        ("day10", "part2") => days::day10::part2::solve(),
        ("day11", "part1") => days::day11::part1::solve(),
        ("day11", "part2") => days::day11::part2::solve(),
        ("day12", "part1") => days::day12::part1::solve(),
        ("day12", "part2") => days::day12::part2::solve(),
        ("day13", "part1") => days::day13::part1::solve(),
        ("day13", "part2") => days::day13::part2::solve(),
        ("day14", "part1") => days::day14::part1::solve(),
        ("day14", "part2") => days::day14::part2::solve(),
        ("day15", "part1") => days::day15::part1::solve(),
        ("day15", "part2") => days::day15::part2::solve(),
        ("day16", "part1") => days::day16::part1::solve(),
        ("day16", "part2") => days::day16::part2::solve(),
        ("day17", "part1") => days::day17::part1::solve(),
        ("day17", "part2") => days::day17::part2::solve(),
        ("day18", "part1") => days::day18::part1::solve(),
        ("day18", "part2") => days::day18::part2::solve(),
        _ => {
            eprintln!("Invalid day or part. Usage: cargo run <day> <part>");
            Ok(())
        }
    };

    let duration = start.elapsed();

    match &result {
        Ok(_) => println!("Execution time: {:.2?}", duration),
        Err(err) => eprintln!("Error occurred: {} (Execution time: {:.2?})", err, duration),
    }

    result
}