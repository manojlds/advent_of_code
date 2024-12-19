use std::fs;
use std::fmt;
use std::hash::Hash;
use std::io;
use std::collections::HashMap;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::{BinaryHeap};
use std::ops::Add;
use std::cmp::Ordering;


#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.y, self.x)
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Coord;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        let offset = rhs.to_offset();
        self + offset 
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
enum Element {
    Empty,
    Obstacle,
}

impl From<Element> for char {
    fn from(val: Element) -> Self {
        match val {
            Element::Empty => '.',
            Element::Obstacle => '#',
        }
    }
}


#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Robot {
    coord: Coord,
    steps: usize
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Robot {
    pub fn new(coord: Coord) -> Self {
        Robot { coord, steps: 0,}
    }

    pub fn move_in_direction(&self, direction: Direction) -> Self {
        let new_coord = self.coord + direction;

        Robot { coord: new_coord, steps: self.steps + 1 }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Element>>,
    start: Coord,
    end: Coord
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let grid = vec![vec![Element::Empty; width]; height];
        let start =  Coord { x: 0, y: 0 };
        let end =  Coord { x: (height - 1) as i32, y: (width - 1) as i32 };
        Grid { grid, start, end }
    }

    fn is_within_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0
            && coord.x < self.grid.len() as i32
            && coord.y >= 0
            && coord.y < self.grid[0].len() as i32
    }

    pub fn apply_obstacles(&mut self, obstacles: &Vec<Coord>, ticks: usize) {
        for obstacle in obstacles.iter().take(ticks) {
            self.grid[obstacle.x as usize][obstacle.y as usize] = Element::Obstacle;
        }
    }

    pub fn navigate(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut visited = HashMap::new();

        let start_robot = Robot::new(self.start);

        heap.push(start_robot);

        while let Some(current) = heap.pop() {
            if current.coord == self.end {
                return Some(current.steps);
            }

            if let Some(&steps) = visited.get(&current.coord) {
                if current.steps >= steps {
                    continue;
                }
            }

            visited.insert(current.coord, current.steps);

            for direction in Direction::iter() {
                let new_robot = current.move_in_direction(direction);
                
                if self.is_within_bounds(new_robot.coord) {
                    if self.grid[new_robot.coord.x as usize][new_robot.coord.y as usize] != Element::Obstacle {
                        heap.push(new_robot);
                    }
                }
            } 
        }
        None
    }
}

pub fn read_input() -> io::Result<Vec<Coord>> {
    let input = fs::read_to_string("inputs/day18/input.txt")?;

    let coords = input.lines().map(|line| {
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        //easier to have it row and column based
        Coord { x: y as i32, y: x as i32 }
    }).collect();

    Ok(coords)
}