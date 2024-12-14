use std::io::{self, Write};
use std::fs;
use regex::Regex;
use strum_macros::EnumIter;
use std::collections::{HashMap, HashSet};


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
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Robot {
    coord: Coord,
    velocity: Coord
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub robots: Vec<Robot>,
}

impl Grid {
    pub fn new(width: i32, height: i32, robots: Vec<Robot>) -> Grid {
        Grid { width, height, robots }
    }

    fn wrap(c: i32, max: i32) -> i32 {
        if c < 0 {
            max + c
        } else if c >= max {
            c - max
        } else {
            c
        }
    }

    fn quadrant(c: i32, max: i32) -> i32 {
        if c < max / 2 {
            0
        } else {
            1
        }
    }

    pub fn print(&self) {
        let mut grid: Vec<Vec<String>> = vec![vec![String::from("."); self.width as usize]; self.height as usize];

        for robot in self.robots.iter() {
            if grid[robot.coord.y as usize][robot.coord.x as usize] ==String::from(".") {
                grid[robot.coord.y as usize][robot.coord.x as usize] = String::from("1");
            } else {
                grid[robot.coord.y as usize][robot.coord.x as usize] = 
                    (grid[robot.coord.y as usize][robot.coord.x as usize].parse::<i32>().unwrap() + 1).to_string();
            }
        }

        for row in grid.iter() {
            println!("{}", row.join(""));
        }

        println!()
    }

    pub fn tick(&mut self, print: bool) {
        for robot in self.robots.iter_mut() {
            robot.coord.x += robot.velocity.x;
            robot.coord.x = Grid::wrap(robot.coord.x, self.width);

            robot.coord.y += robot.velocity.y;
            robot.coord.y = Grid::wrap(robot.coord.y, self.height);
        }

        if print {
            print!("\x1B[2J\x1B[1;1H");
            self.print();
            io::stdout().flush().unwrap();
        }
    }

    pub fn is_unique_locations(&mut self) -> bool {
        let robot_coords: HashSet<_> = self.robots.iter().map(|robot| robot.coord).collect();

        robot_coords.len() == self.robots.len()
    }

    pub fn safety_factor(&self) -> i32 {
        let mut quardrants = HashMap::new();

        self.robots.iter().for_each(|robot| {
            if robot.coord.x == (self.width - 1) / 2 || robot.coord.y == (self.height - 1) / 2 {
                return;
            }
            let quadrant = (Grid::quadrant(robot.coord.x, self.width), Grid::quadrant(robot.coord.y, self.height));
            quardrants.entry(quadrant).and_modify(|count| *count += 1).or_insert(1);
        });

        quardrants.values().product()
    }
}


pub fn read_input() -> io::Result<Vec<Robot>> {
    let input = fs::read_to_string("inputs/day14/input.txt")?;

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots: Vec<Robot> = input.lines()
        .filter_map(|line| re.captures(line))
        .map(|caps| {
            let coord = Coord {
                x: caps[1].parse::<i32>().unwrap(),
                y: caps[2].parse::<i32>().unwrap(),
            };
            let velocity = Coord {
                x: caps[3].parse::<i32>().unwrap(),
                y: caps[4].parse::<i32>().unwrap(),
            };
            Robot { coord, velocity }
        })
        .collect();

    Ok(robots)
}
