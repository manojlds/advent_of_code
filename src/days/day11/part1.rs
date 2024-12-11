use super::utils::{read_input, blink};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input().expect("Failed to read input");

    let output = blink(input, 25);

    println!("Day 11 Part 1: {:?}", output);
    
    Ok(())
}