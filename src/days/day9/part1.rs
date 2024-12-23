use crate::days::day9::utils::checksum;

use super::utils::{read_input, pack};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = read_input().expect("Failed to read input");

    pack(&mut input);

    let sum = checksum(input);

    println!("Day 9 Part 1: {:?}", sum);

    Ok(())
}