use super::utils::{read_input, Grid};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = read_input().expect("Failed to read input");

    let mut grid = Grid::new(101, 103, inputs);

    for _ in 0..100{
        grid.tick(false);
    }

    println!("Day 14 Part 1: {:?}", grid.safety_factor());
    
    Ok(())
}