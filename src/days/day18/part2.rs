
use super::utils::{read_input, Grid};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let coords = read_input().expect("Input not found");

    let mut first_blocking = 0;
    for i in 0..coords.len() {
        let mut grid = Grid::new(71,71);
        grid.apply_obstacles(&coords, coords.len() - i);
        if let Some(_) = grid.navigate() {
            first_blocking = coords.len() - i;
            break;
        }
    }

    println!("Day 18 Part 1: {}", coords[first_blocking]);
    
    Ok(())
}