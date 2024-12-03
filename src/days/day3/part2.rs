use regex::Regex;
use std::fs;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let re = Regex::new(r"(?P<mul>mul\((\d+),(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();

    let input = fs::read_to_string("inputs/day3/input.txt")?;

    let mut result = 0;

    let mut is_enabled = true;

    for line in input.lines() {
        let mut sum = 0;
        
        
        for capture in re.captures_iter(&line) {
            if let Some(_) = capture.name("mul") {
                if is_enabled {
                    let num1: i32 = capture[2].parse().unwrap();
                    let num2: i32 = capture[3].parse().unwrap();
                    sum += num1 * num2;
                }
            }
            else if let Some(_) = capture.name("do") {
                is_enabled = true;
            }
            else if let Some(_) = capture.name("dont") {
                is_enabled = false;
            }
        }

        result += sum

    }

    println!("Day 3 Part 2: {}", result);

    Ok(())
}