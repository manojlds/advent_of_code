use super::utils::read_input;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_input().expect("Failed to read input");
    let (_, best_points) = grid.navigate();

    grid.print_grid_best_points(&best_points);

    println!("Day 16 Part 2: {:?}", best_points.len());
    
    Ok(())
}