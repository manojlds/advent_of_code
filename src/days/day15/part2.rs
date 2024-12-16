use super::utils2::{read_input};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (mut grid, commands) = read_input().expect("Failed to read input");

    grid.print_grid();

    grid.process_commands(commands);
    
    println!("Day 15 Part 2: {:?}", grid.gps());
    
    Ok(())
}