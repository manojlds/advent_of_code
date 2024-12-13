use super::utils::{read_input, cost};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_input().expect("Failed to read input");

    let output = cost(grid);

    println!("Day 12 Part 1: {:?}", output);
    
    Ok(())
}