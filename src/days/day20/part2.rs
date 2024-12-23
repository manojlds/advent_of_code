
use super::utils::read_input;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_input().expect("Input not found");

    let cheats = grid.find_cheats(100, 20);

    let num_min_duration = cheats.values().sum::<usize>();
    
    println!("Day 20 Part 2: {:?}", num_min_duration);
    
    Ok(())
}