use super::utils::{read_input, solve_buttons};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = read_input().expect("Failed to read input");

    let mut tokens = 0;
    for input in inputs {
        if let Some(solution) = solve_buttons(&input.with_increased_prize(10000000000000)) {
            tokens += solution.0 * 3 + solution.1;
        }
    }

    println!("Day 13 Part 1: {:?}", tokens);
    
    Ok(())
}