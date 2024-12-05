use std::fs;
use std::io;

fn read_grid() -> io::Result<Vec<Vec<char>>> {
    let input = fs::read_to_string("inputs/day4/input.txt")?;
    let mut grid = Vec::new();
    
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

fn check(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid[i][j] != 'A' {
        return false
    }

    return ((grid[i-1][j-1] == 'M' && grid[i+1][j+1] == 'S') || (grid[i-1][j-1] == 'S' && grid[i+1][j+1] == 'M'))
        && ((grid[i+1][j-1] == 'M' && grid[i-1][j+1] == 'S') || (grid[i+1][j-1] == 'S' && grid[i-1][j+1] == 'M'))

}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    
    let grid = read_grid().expect("Failed to read input");

    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if check(&grid, i, j) {
                count += 1;
            }
            
        }
    }

    println!("Day 4 Part 2: {}", count);

    Ok(())
}