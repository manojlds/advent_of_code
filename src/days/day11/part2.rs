use super::utils::{read_input, blink};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input().expect("Failed to read input");

    let output = blink(input, 75);

    println!("Day 11 Part 2: {:?}", output);
    
    Ok(())
}