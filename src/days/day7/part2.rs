use super::utils::{read_input, evaluate};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = read_input().expect("Failed to read input");

    let result = evaluate(inputs, true);

    println!("Day 7 Part 2: {}", result);

    Ok(())
}