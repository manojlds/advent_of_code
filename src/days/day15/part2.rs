use super::utils::{read_input, Grid};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (mut grid, commands) = read_input().expect("Failed to read input");

    grid.process_commands(commands);
    
    println!("Day 14 Part 1: {:?}", 1);
    
    Ok(())
}