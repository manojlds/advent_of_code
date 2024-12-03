use regex::Regex;
use std::fs;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let input = fs::read_to_string("inputs/day3/input.txt")?;

    let mut result = 0;

    for line in input.lines() {
        let mut sum = 0;
        
        for capture in re.captures_iter(&line) {
            let num1: i32 = capture[1].parse().unwrap();
            let num2: i32 = capture[2].parse().unwrap();
            sum += num1 * num2;
        }

        result += sum

    }

    println!("Day 3 Part 1: {}", result);

    Ok(())
}