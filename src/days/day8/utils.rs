use std::fs;
use std::io;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub fn read_input() -> io::Result<(HashMap<char, Vec<Coord>>, (i32, i32))> {
    let input = fs::read_to_string("inputs/day8/input.txt")?;
    let lines: Vec<&str> = input.lines().collect();

    let num_chars_first_line = lines.get(0).map_or(0, |line| line.chars().count());

    let mut map = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = Coord { x: i as i32, y: j as i32};
            map.entry(c).or_insert_with(Vec::new).push(coord);
        }
    }
    return Ok((map, (lines.len() as i32, num_chars_first_line as i32)));
}



pub fn find_anti_nodes(coord1: &Coord, coord2: &Coord, bounds: (i32, i32)) -> Vec<Coord> {
    let mut antinodes = Vec::new();

    let x_offset = (coord1.x - coord2.x);
    let y_offset = (coord1.y - coord2.y);

    let x1 = coord1.x + x_offset;
    let y1 = coord1.y + y_offset;
    
    let x2 = coord2.x - x_offset;
    let y2 = coord2.y - y_offset;

    if x1 >= 0 && x1 < bounds.0 && y1 >= 0 && y1 < bounds.1 {
        antinodes.push(Coord{x: x1, y: y1});
    }
    
    if x2 >=0 && x2 < bounds.0 && y2 >= 0 && y2 < bounds.1 {
        antinodes.push(Coord{x: x2, y: y2});
    }

    return antinodes; 
}