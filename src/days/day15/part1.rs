use super::utils::{read_input, Grid};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (mut grid, commands) = read_input().expect("Failed to read input");

    grid.print_grid();
    grid.process_commands(commands);

    println!("Day 15 Part 1: {:?}", grid.gps());
    
    Ok(())
}