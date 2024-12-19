
use super::utils::{read_input, check_patterns};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (available_towels, patterns) = read_input().expect("Input not found");

    let count = check_patterns(available_towels, patterns);

    println!("Day 19 Part 1: {:?}", count);
    
    Ok(())
}