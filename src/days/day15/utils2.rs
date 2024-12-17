use core::panic;
use std::io::{self, Write};
use std::fs;
use strum_macros::EnumIter;
use std::ops::Add;

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
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Robot {
    coord: Coord,
}

impl Add<Direction> for Robot {
    type Output = Robot;

    fn add(self, rhs: Direction) -> Self::Output {
        Robot { coord: self.coord + rhs }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Element {
    Empty,
    Wall,
    Obstacle,
    Robot,
    ObstacleLeft,
    ObstacleRight
}

impl Element {
    fn expand(self) -> Vec<Self> {
        match self {
            Element::Empty => vec![Element::Empty, Element::Empty],
            Element::Wall => vec![Element::Wall, Element::Wall],
            Element::Obstacle => vec![Element::ObstacleLeft, Element::ObstacleRight],
            Element::Robot => vec![Element::Robot, Element::Empty],
            Element::ObstacleLeft | Element::ObstacleRight => panic!("Invalid input"),
        }
    }
}

impl Element {
    fn to_colored_char(self) -> String {
        match self {
            Element::Empty => format!("\x1B[90m{}\x1B[0m", '.'),
            Element::Wall => format!("\x1B[37;1m{}\x1B[0m", '#'),
            Element::Obstacle => format!("\x1B[33m{}\x1B[0m", 'O'),
            Element::Robot => format!("\x1B[32;1m{}\x1B[0m", '@'),
            Element::ObstacleLeft => format!("\x1B[31m{}\x1B[0m", '['),
            Element::ObstacleRight => format!("\x1B[31m{}\x1B[0m", ']'),
        }
    }
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Obstacle,
            '@' => Self::Robot,
            '[' => Self::ObstacleLeft,
            ']' => Self::ObstacleRight,
            _ => Self::Empty,
        }
    }
}

impl From<Element> for char {
    fn from(val: Element) -> Self {
        match val {
            Element::Empty => '.',
            Element::Wall => '#',
            Element::Obstacle => 'O',
            Element::Robot => '@',
            Element::ObstacleLeft => '[',
            Element::ObstacleRight => ']'
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Element>>,
    robot: Robot
}


impl Grid {
    pub fn new(input: String) -> Grid {
        let mut grid = Vec::new();
        let mut robot = Robot {
            coord: Coord { x: 0, y: 0 }
        };
        for (i, line) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (j,c) in line.chars().enumerate() {
                let element = Element::from(c);
                grid[i].extend(element.expand());
                if element == Element::Robot {
                    robot.coord = Coord { x: i as i32, y: (j * 2) as i32 };
                }
            }
        }
        Grid {
            grid,
            robot
        }
    }

    pub fn print_grid(&self) {
        print!("\x1B[2J\x1B[H");
        println!("Robot is at: {:?}", self.robot.coord);
        for row in &self.grid {
            println!("{}", row.iter().map(|&e| e.to_colored_char()).collect::<String>());
        }
        io::stdout().flush().unwrap();

        // println!()
    }

    pub fn move_robot_char(&mut self, command: char) {
        match command {
            '^' => self.move_robot(Direction::North),
            '>' => self.move_robot(Direction::East),
            'v' => self.move_robot(Direction::South),
            '<' => self.move_robot(Direction::West),
            _ => panic!("Invalid command"),
        }
    }

    pub fn move_element(&mut self, current_coord: Coord, direction: Direction) {
        let new_coord = current_coord + direction;
        let next_element = self.grid[new_coord.x as usize][new_coord.y as usize];

        match next_element  {
            Element::Empty => {
                self.grid[new_coord.x as usize][new_coord.y as usize] = self.grid[current_coord.x as usize][current_coord.y as usize];
                self.grid[current_coord.x as usize][current_coord.y as usize] = Element::Empty;
            },
            Element::ObstacleLeft => {
                self.move_element(new_coord + (0, 1), direction);
                self.move_element(new_coord, direction);
                self.grid[new_coord.x as usize][new_coord.y as usize] = self.grid[current_coord.x as usize][current_coord.y as usize];
                self.grid[current_coord.x as usize][current_coord.y as usize] = Element::Empty;

            },
            Element::ObstacleRight => {
                self.move_element(new_coord + (0, -1), direction);
                self.move_element(new_coord, direction);
                self.grid[new_coord.x as usize][new_coord.y as usize] = self.grid[current_coord.x as usize][current_coord.y as usize];
                self.grid[current_coord.x as usize][current_coord.y as usize] = Element::Empty;

            }
            _ => panic!("Invalid state")
        }


    }

    pub fn can_move(&self,  current_coord: Coord, direction: Direction) -> bool {
        let new_coord = current_coord + direction;
        let next_element = self.grid[new_coord.x as usize][new_coord.y as usize];

        match next_element {
            Element::Empty => true,
            Element::Wall => false,
            Element::ObstacleLeft => {
                if direction == Direction::West {
                    self.can_move(new_coord, direction)
                } else if direction == Direction::East {
                    self.can_move(new_coord + (0,1), direction)
                } else {
                    self.can_move(new_coord, direction) && self.can_move(new_coord + (0, 1), direction)
                }
            },
            Element::ObstacleRight => {
                
                if direction == Direction::East {
                    self.can_move(new_coord, direction)
                } else if direction == Direction::West {
                    self.can_move(new_coord + (0, -1), direction)
                } else {
                    self.can_move(new_coord, direction) && self.can_move(new_coord + (0, -1), direction)
                }
            }
            _ => panic!("Invalid state")
        }
    }

    pub fn move_robot(&mut self, direction: Direction) {

        if self.can_move(self.robot.coord, direction) {
            self.move_element(self.robot.coord, direction);
            self.robot = self.robot + direction;

        }
    }

    pub fn process_commands(&mut self, commands: Vec<char>, visualize: bool) {
        for c in commands {
            self.move_robot_char(c);
            
            if visualize {
                self.print_grid();
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    }

    pub fn gps(&self) -> i32 {
        let mut sum: i32 = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == Element::ObstacleLeft {
                    sum += 100 * (i as i32) + (j as i32);        
                }
            }
        }

        sum
    }
}

pub fn read_input() -> io::Result<(Grid, Vec<char>)> {
    let input = fs::read_to_string("inputs/day15/input.txt")?;

    let mut sections = input.split("\n\n");

    let grid = Grid::new(sections.next().unwrap().to_string());

    let commands = sections.next().unwrap().chars()
        .filter(|c| *c != '\n')
        .collect();

    Ok((grid, commands))
}
