use std::io::{self, Write};
use std::fs;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::ops::Add;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
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
    pub fn turn_cost(from: Direction, to: Direction) -> usize {
        let from_index = from as usize;
        let to_index = to as usize;
        let diff = (4 + to_index - from_index) % 4;
        1000 * diff.min(4 - diff)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Robot {
    coord: Coord,
    direction: Direction,
    cost: usize,
    path: Vec<Coord>
}

impl Robot {
    pub fn new(coord: Coord, direction: Direction) -> Self {
        Robot { coord, direction, cost: 0, path: vec![coord]}
    }

    pub fn move_in_direction(&self) -> Self {
        let mut path = self.path.clone();
        path.push(self.coord);
        Robot { coord: self.coord + self.direction, direction: self.direction, cost: self.cost + 1, path: path}
    }

    pub fn turn(&self, direction: Direction) -> Self {
        let cost = self.cost + Direction::turn_cost(self.direction, direction);
        Robot { coord: self.coord, direction, cost, path: self.path.clone() }
    }
}

impl Add<usize> for Robot {
    type Output = Robot;

    fn add(self, rhs: usize) -> Self::Output {
        Robot { coord: self.coord, direction: self.direction, cost: self.cost + rhs, path: self.path }
    }
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Element {
    Empty,
    Wall,
    Start,
    End
}

impl Element {
    fn to_colored_char(self) -> String {
        match self {
            Element::Empty => format!("\x1B[90m{}\x1B[0m", '.'),
            Element::Wall => format!("\x1B[37;1m{}\x1B[0m", '#'),
            Element::End => format!("\x1B[33m{}\x1B[0m", 'E'),
            Element::Start => format!("\x1B[32;1m{}\x1B[0m", 'S'),
        }
    }
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => Self::Empty,
        }
    }
}

impl From<Element> for char {
    fn from(val: Element) -> Self {
        match val {
            Element::Empty => '.',
            Element::Wall => '#',
            Element::Start => 'S',
            Element::End => 'E',
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Element>>,
    start: Coord,
    end: Coord
}

impl Grid {
    pub fn new(input: String) -> Grid {
        let mut grid = Vec::new();
        let mut start = Coord {x: 0, y: 0};
        let mut end = Coord {x: 0, y: 0};
        
        for (i, line) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (j,c) in line.chars().enumerate() {
                let element = Element::from(c);
                grid[i].push(element);
                if element == Element::Start {
                    start = Coord { x: i as i32, y: j as i32 };
                } else if element == Element::End {
                    end = Coord { x: i as i32, y: j as i32 };
                }
            }
        }
        Grid {
            grid,
            start,
            end   
        }
    }

    pub fn print_grid(&self) {
        print!("\x1B[2J\x1B[H");
        
        for row in &self.grid {
            println!("{}", row.iter().map(|&e| e.to_colored_char()).collect::<String>());
        }
        io::stdout().flush().unwrap();

        println!()
    }
    
    pub fn print_grid_best_points(&self, best_points: &HashSet<Coord>) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                print!("{}", if best_points.contains(&Coord { x: i as i32, y: j as i32 }) {
                    String::from("O")
                } else {
                    format!("{}", char::from(*element))
                });
            }
            println!()
        }
        println!()
    }

    pub fn navigate(&self) -> (usize, HashSet<Coord>) {
        let mut heap = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut best_cost = usize::MAX;
        let mut best_points: HashSet<Coord> = HashSet::new();
    
        let start_robot = Robot::new(self.start, Direction::East);

        heap.push(start_robot);

        while let Some(current) = heap.pop() {
            if current.coord == self.end {
                if current.cost < best_cost {
                    best_cost = current.cost;
                }
                if current.cost == best_cost {

                   best_points.extend(current.path.clone());
                   best_points.insert(current.coord);
                }
            }

            if let Some(&visited_cost) = visited.get(&(current.coord, current.direction)) {
                if visited_cost < current.cost {
                    continue;
                }
            }

            visited.insert((current.coord, current.direction), current.cost);

            let new_robot = current.move_in_direction();
            
            
            if self.grid[new_robot.coord.x as usize][new_robot.coord.y as usize] != Element::Wall {
                heap.push(new_robot);
                
            }

            for new_direction in Direction::iter() {
                if new_direction == current.direction {
                    continue;
                }
                let new_robot = current.turn(new_direction);
                heap.push(new_robot);
                
            }            
        }

        (best_cost, best_points)
    }
}

pub fn read_input() -> io::Result<Grid> {
    let input = fs::read_to_string("inputs/day16/input.txt")?;

    let grid = Grid::new(input);

    Ok(grid)
}
