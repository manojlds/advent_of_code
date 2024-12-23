use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fmt;
use std::hash::Hash;
use std::io;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::ops::Add;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
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

    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Element {
    Empty,
    Wall,
    Start,
    End
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
    end: Coord,
}

impl Grid {
    pub fn new(input: String) -> Self {
        let mut grid = Vec::new();
        let mut start = Coord { x: 0, y: 0 };
        let mut end = Coord { x: 0, y: 0 };

        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let element: Element = c.into();
                row.push(element);
                if element == Element::Start {
                    start = Coord { x: i as i32, y: j as i32 };
                } else if element == Element::End {
                    end = Coord { x: i as i32, y: j as i32 };
                }
            }
            grid.push(row);
        }

        println!("Start: {}, End: {}", start, end);

        Grid { grid, start, end }
    }

    fn is_within_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0
            && coord.x < self.grid.len() as i32
            && coord.y >= 0
            && coord.y < self.grid[0].len() as i32
    }

    fn element_at(&self, coord: Coord) -> Element {
        self.grid[coord.x as usize][coord.y as usize]
    }

    pub fn race_path(&self) -> (Vec<Coord>, HashMap<Coord, usize>) {
        let mut path = Vec::new();
        let mut distances = HashMap::new();
        let mut current_coord = self.start;
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut distance = 0;
        loop {
            path.push(current_coord);
            visited.insert(current_coord);
            distances.insert(current_coord, distance);
            distance += 1;
            
            if current_coord == self.end {
                break;
            }

            for (neigbour, _) in self.neighbours(current_coord) {
                if !visited.contains(&neigbour) && self.element_at(neigbour) != Element::Wall {
                    current_coord = neigbour;
                    break;
                }
            }
        }
        (path, distances)
    }

    pub fn duration(first: Coord, second: Coord, distances: &HashMap<Coord, usize>) -> usize {
        let from = *distances.get(&first).unwrap();
        let to = *distances.get(&second).unwrap();
        to - from
    }

    pub fn get_distance(&self, first: Coord, second: Coord) -> usize {
        let distance = (first.x - second.x).abs() + (first.y - second.y).abs();
        
        distance as usize
    }

    pub fn neighbours(&self, coord: Coord) -> HashSet<(Coord, Direction)> {
        Direction::iter()
            .map(|d| (coord + d, d))
            .filter(|(c, _)| self.is_within_bounds(*c))
            .collect()
    }

    pub fn find_cheats(&self, min_duration: usize, allowed_distance: usize) -> HashMap<usize, usize> {
        let (path, distances) = self.race_path();
        let mut cheats  = HashMap::new();

        for x in 0..(path.len() - min_duration) {
            for y in (x + min_duration)..path.len() {
                let first = path[x];
                let second = path[y];
                let distance = self.get_distance(first, second);
                if distance <= allowed_distance {
                    let duration = Grid::duration(first, second, &distances);
                    //Current duration minus the new cheat distance gives the duration saved
                    let duration_saved = duration - distance;
                    *cheats.entry(duration_saved).or_default() += 1
                }
            }
        }
        cheats.retain(|k, _| *k >= min_duration);

        cheats
    }
}

pub fn read_input() -> io::Result<Grid> {
    let input = fs::read_to_string("inputs/day20/input.txt")?;

    Ok(Grid::new(input))
}