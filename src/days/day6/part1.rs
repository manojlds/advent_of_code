use super::utils::{read_input, traverse};


pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (grid, coord, dir) = read_input().expect("Failed to read input");

    let visited = traverse(&grid, coord, dir).left().unwrap();
    println!("Day 6 Part 1: {}", visited.len());

    Ok(())
}