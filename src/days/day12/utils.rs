use std::i32;
use std::io;
use std::fs;
use std::collections::HashSet;
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
    let mut boundary_edges = 0;

    while let Some(coord) = stack.pop() {
        if visited.contains(&coord) {
            continue;
        }
        visited.insert(coord);

        for direction in Direction::iter() {
            let next_coord = coord.next_coord(&direction);

            if !next_coord.is_within_bounds(grid) || grid[next_coord.x as usize][next_coord.y as usize] != *cell {
                boundary_edges += 1;
            } else if !visited.contains(&next_coord) {
                stack.push(next_coord);
            }
        }
    }

    (visited.len() as i32, boundary_edges, visited)
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
