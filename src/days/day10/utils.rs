use std::io;
use std::fs;
use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, EnumIter)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

impl Coord {
    pub fn next_coord(&self, direction: &Direction) -> Coord {
        let (dx, dy) = direction.to_offset();
        Coord {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn next_in_trail(&self,grid: &Vec<Vec<u8>>, current_value: u8) -> Vec<Coord> {
        let mut next_coords = Vec::new();

        for direction in Direction::iter() {
            let next_coord = self.next_coord(&direction);
            if next_coord.is_within_bounds(grid) &&
            grid[next_coord.x as usize][next_coord.y as usize] == current_value + 1  {
                next_coords.push(next_coord);
            }
        }

        next_coords
    }

    pub fn is_within_bounds(&self, grid: &Vec<Vec<u8>>) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < grid.len() as i32 && self.y < grid[0].len() as i32
    }
}


pub fn read_input() -> io::Result<Vec<Vec<u8>>> {
    let input = fs::read_to_string("inputs/day10/input.txt")?;
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }

    Ok(grid)
}

fn bfs_score(grid: &Vec<Vec<u8>>, start: Coord) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = 0;

    queue.push_back(start);
    visited.insert(start);

    while let Some(coord) = queue.pop_front() {
        let current_value = grid[coord.x as usize][coord.y as usize];

        for next in coord.next_in_trail(grid, current_value) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
                if grid[next.x as usize][next.y as usize] == 9 {
                    reachable_nines += 1;
                }
            }
        }
    }

    reachable_nines
}

pub fn find_trail_heads(grid: &Vec<Vec<u8>>) -> Vec<Coord> {
    let mut trail_heads = Vec::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, &val) in row.iter().enumerate() {
            if val == 0 {
                trail_heads.push(Coord {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    return trail_heads
}


pub fn find_trail_scores(grid: &Vec<Vec<u8>>) -> usize {
    
    let trail_heads = find_trail_heads(grid);
    let mut results = Vec::new();
    let mut score_sums: usize = 0;
    for head in &trail_heads {
        let score = bfs_score(&grid, *head);
        results.push((head, score));
        score_sums += score;
    }

    return score_sums;
}

fn count_unique_paths(grid: &Vec<Vec<u8>>, start: Coord) -> usize {
    fn dfs(
        grid: &Vec<Vec<u8>>, 
        coord: Coord, 
        visited: &mut HashSet<Coord>, 
        current_value: u8,
    ) -> usize {
        if grid[coord.x as usize][coord.y as usize] == 9 {
            return 1;
        }

        visited.insert(coord);
        let mut path_count = 0;

        for next in coord.next_in_trail(grid, current_value) {
            if !visited.contains(&next) {
                path_count += dfs(grid, next, visited, current_value + 1);
            }
        }

        visited.remove(&coord);
        path_count
    }

    let mut visited = HashSet::new();
    dfs(grid, start, &mut visited, 0)
}

pub fn find_trail_ratings(grid: &Vec<Vec<u8>>) -> usize {
    
    let trail_heads = find_trail_heads(grid);
    let mut results = Vec::new();
    let mut rating_sums: usize = 0;
    for head in &trail_heads {
        let rating = count_unique_paths(&grid, *head);
        results.push((head, rating));
        rating_sums += rating;
    }

    return rating_sums;
}