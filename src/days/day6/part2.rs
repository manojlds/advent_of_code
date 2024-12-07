use super::utils::{read_input, traverse};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (grid, coord, dir) = read_input().expect("Failed to read input");

    let mut visited = traverse(&grid, coord.clone(), dir.clone()).left().unwrap();
    let mut num_obstacles = 0;

    visited.remove(&coord);

    for node in visited.iter() {
        let mut grid = grid.clone();
        grid[node.x as usize][node.y as usize] = '#';
        let new_visited = traverse(&grid, coord.clone(), dir.clone());
        if new_visited.is_left() {
            continue
        } else {
            num_obstacles += 1;
        }
        
    }
    
    println!("Day 6 Part 2: {}", num_obstacles);

    Ok(())
}