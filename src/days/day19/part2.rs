
use super::utils2::{read_input, check_patterns};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (available_towels, patterns) = read_input().expect("Input not found");

    let count = check_patterns(available_towels, patterns);

    println!("Day 19 Part 2: {:?}", count);
    
    Ok(())
}