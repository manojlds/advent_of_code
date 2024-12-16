use super::utils2::{read_input};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (mut grid, commands) = read_input().expect("Failed to read input");

    grid.process_commands(commands, false);
    
    println!("Day 15 Part 2: {:?}", grid.gps());
    
    Ok(())
}