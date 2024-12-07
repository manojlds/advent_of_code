use std::io;
use std::fs;
use std::collections::{HashSet, HashMap};
use either::{Either, Left, Right};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn step(&self, dir: &Direction) -> Self {
        let (dx, dy) = dir.to_offset();
        Coord::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, Clone)]
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

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub fn read_input() -> io::Result<(Vec<Vec<char>>, Coord, Direction)> {
    let input = fs::read_to_string("inputs/day6/input.txt")?;
    let mut input_vec = Vec::new();
    let mut start: Coord = Coord::new(0, 0);
    let direction = Direction::North;
    
    for (i,line) in input.lines().enumerate() {
        let mut line_vec = Vec::new();
        for (j,char) in line.chars().enumerate() {
            line_vec.push(char);
            if char == '^' {
                start = Coord::new(i as i32, j as i32);
            }
        }
        input_vec.push(line_vec);
    }

    return Ok((input_vec, start, direction));
}

pub fn traverse(grid: &Vec<Vec<char>>, mut coord: Coord, mut dir: Direction) -> Either<HashSet<Coord>, bool> {
    let mut visited = HashMap::new();
    
    *visited.entry(coord).or_insert(0) += 1;

    loop {
        let new_coord = coord.step(&dir);
        if new_coord.x < 0 || new_coord.x >= grid.len() as i32 || new_coord.y < 0 || new_coord.y >= grid.len() as i32 {
            break;
        }
        if grid[new_coord.x as usize][new_coord.y as usize] == '#' {
            dir = dir.turn_right()
        } else {
            *visited.entry(new_coord).or_insert(0) += 1;
            if let Some(&value) = visited.get(&coord) {
                if value > 10 {
                    return Right(true);
                }
            }
            coord = new_coord;
        }
    }

    return Left(visited.keys().cloned().collect());
}