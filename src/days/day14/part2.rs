use super::utils::{read_input, Grid};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = read_input().expect("Failed to read input");
    let mut grid = Grid::new(101, 103, inputs);

    for t in 0..10000{
        grid.tick(false);
        if grid.is_unique_locations() {
            grid.print();
            println!("Day 14 Part 2: {:?}", t + 1);
            break;
        }
    }
    
    Ok(())
}