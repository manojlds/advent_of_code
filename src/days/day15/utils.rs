use std::io;
use std::fs;
use strum_macros::EnumIter;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
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

impl Robot {
    
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
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
                grid[i].push(c);
                if c == '@' {
                    robot.coord = Coord { x: i as i32, y: j as i32 };
                }
            }
        }
        Grid {
            grid,
            robot
        }
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }

        println!()
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

    pub fn move_robot(&mut self, direction: Direction) {
        let offset = direction.to_offset();
        let (new_x, new_y) = (self.robot.coord.x as i32 + offset.0, self.robot.coord.y as i32 + offset.1);

        if self.grid[new_x as usize][new_y as usize] == '#' {
            return;
        }
        if self.grid[new_x as usize][new_y as usize] == '.' {
            self.grid[new_x as usize][new_y as usize] = '@';
            self.grid[self.robot.coord.x as usize][self.robot.coord.y as usize] = '.';
            self.robot.coord = Coord { x: new_x, y: new_y };
            return;
        }
        if self.grid[new_x as usize][new_y as usize] == 'O' {
            let mut obstacle_x = new_x;
            let mut obstacle_y = new_y;

            loop {
                obstacle_x += offset.0;
                obstacle_y += offset.1;

                if self.grid[obstacle_x as usize][obstacle_y as usize] != 'O' {
                    break;
                }
            }

            if self.grid[obstacle_x as usize][obstacle_y as usize] == '#' {
                return;
            }

            self.grid[obstacle_x as usize][obstacle_y as usize] = 'O';
            self.grid[new_x as usize][new_y as usize] = '@';
            self.grid[self.robot.coord.x as usize][self.robot.coord.y as usize] = '.';
            self.robot.coord = Coord { x: new_x, y: new_y };
            
        }
    }

    pub fn process_commands(&mut self, commands: Vec<char>) {
        for c in commands {
            println!("Command: {}", c);
            self.move_robot_char(c);
            self.print_grid();
        }
    }

    pub fn gps(&self) -> i32 {
        let mut sum: i32 = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == 'O' {
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
