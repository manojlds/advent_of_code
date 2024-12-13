use super::utils::{read_input, bulk_cost};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_input().expect("Failed to read input");

    let output = bulk_cost(grid);

    println!("Day 12 Part 2: {:?}", output);
    
    Ok(())
}