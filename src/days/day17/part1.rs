use super::utils::read_input;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (mut computer, program) = read_input().expect("Input not found");

    let output = computer.run(program);

    let output = output.iter()
    .map(|num| num.to_string())
    .collect::<Vec<_>>()
    .join(","); 

    println!("Day 17 Part 1: {:?}", output);
    
    Ok(())
}