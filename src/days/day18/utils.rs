use std::fs;
use std::hash::Hash;
use std::io;
use std::collections::HashMap;
use regex::Regex;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;


#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}



pub fn read_input() -> io::Result<Vec<Coord>> {
    let input = fs::read_to_string("inputs/day18/input.txt")?;

    let coords = input.lines().map(|line| {
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();
        //easier to have it row and column based
        Coord { x: y, y: x }
    }).collect();

    Ok(coords)
}