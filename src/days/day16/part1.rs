use super::utils::{read_input};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_input().expect("Failed to read input");

    println!("{:?}", grid);

    let (cost, _) = grid.navigate();

    println!("Day 16 Part 1: {:?}", cost);
    
    Ok(())
}