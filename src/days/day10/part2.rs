use super::utils::{find_trail_ratings, read_input};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_input().expect("Failed to read input");
    let trail_scores = find_trail_ratings(&grid);

    println!("Day 10 Part 2: {:?}", trail_scores);

    Ok(())
}