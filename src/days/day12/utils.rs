use std::i32;
use std::io;
use std::fs;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Region {
    pub id: char,
    pub area: i32,
    pub perimeter: i32
}

impl Region {
    pub fn cost(&self) -> i32 {
        self.area * self.perimeter
    }
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

    pub fn next_in_region(&self, grid: &Vec<Vec<char>>, current_value: char, visited: &HashSet<Coord>) -> HashMap<Coord, i32> {
        let mut next_coords = HashMap::new();
    
        for direction in Direction::iter() {
            let next_coord = self.next_coord(&direction);
            if next_coord.is_within_bounds(grid)
                && grid[next_coord.x as usize][next_coord.y as usize] == current_value
            {
                let perimeter = if visited.contains(&next_coord) {
                    0
                } else {
                    next_coord.find_perimeter(grid, current_value)
                };
                next_coords.insert(next_coord, perimeter);
            }
        }
    
        next_coords
    }

    pub fn find_perimeter(&self, grid: &Vec<Vec<char>>, current_value: char) -> i32 {
        let mut perimeter = 0;
        for direction in Direction::iter() {
            let next_coord = self.next_coord(&direction);
           
            if !next_coord.is_within_bounds(grid) ||
            grid[next_coord.x as usize][next_coord.y as usize] != current_value  {
                perimeter += 1;
            }
        }
        
        perimeter
        
    }

    pub fn is_within_bounds(&self, grid: &Vec<Vec<char>>) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < grid.len() as i32 && self.y < grid[0].len() as i32
    }
}


pub fn read_input() -> io::Result<Vec<Vec<char>>> {
    let input = fs::read_to_string("inputs/day12/input.txt")?;
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok(grid)
}

pub fn find_region(cell: &char, current_coord: Coord, grid: &Vec<Vec<char>>) -> (i32, i32, HashSet<Coord>) {
    let mut stack = vec![current_coord];
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut perimeter_visited: HashSet<Coord> = HashSet::new();
    let mut current_perimeter = current_coord.find_perimeter(grid, *cell);

    while let Some(coord) = stack.pop() {
        if visited.contains(&coord) {
            continue;
        }
        visited.insert(coord);
        perimeter_visited.insert(coord);
        let next_coords = coord.next_in_region(grid, *cell, &perimeter_visited);
        perimeter_visited.extend(next_coords.keys());
        let next_coords: HashMap<Coord, i32> = next_coords
            .into_iter()
            .filter(|(coord, _)| !visited.contains(coord))
            .collect();
        stack.extend(next_coords.keys());
        
        current_perimeter += next_coords.values().sum::<i32>();

    }
    (visited.len() as i32, current_perimeter, visited)
}

pub fn build_regions(grid: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let current_coord = Coord { x: i as i32, y: j as i32 };
            if visited.contains(&current_coord) {
                continue;
            }
            let (new_area, new_perimeter, new_visited) = find_region(&cell, current_coord, &grid);
            
            regions.push(Region {
                id: cell,
                area: new_area,
                perimeter: new_perimeter,
            });

            visited.extend(new_visited);
        }
    }
    regions
}

pub fn cost(grid: Vec<Vec<char>>) -> i32 {
    let regions = build_regions(grid);

    regions.iter().map(|region| region.cost()).sum::<i32>()

}