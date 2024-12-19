
use super::utils::{read_input, Grid};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let coords = read_input().expect("Input not found");

    let mut grid = Grid::new(71,71);
    grid.apply_obstacles(&coords, 1024);

    let steps =  grid.navigate();

    println!("Day 18 Part 1: {:?}", steps.unwrap());
    
    Ok(())
}