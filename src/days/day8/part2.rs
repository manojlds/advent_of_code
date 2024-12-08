use std::collections::HashSet;

use super::utils::{find_anti_nodes, read_input, Coord};

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (map, bounds) = read_input().expect("Failed to read input");

    let mut anti_nodes: HashSet<Coord> = HashSet::new();

    for antennas in map.values() {
        for (i, coord1) in antennas.iter().enumerate() {
            for coord2 in antennas.iter().skip(i + 1) {
                let nodes = find_anti_nodes(coord1, coord2, bounds, true);
                anti_nodes.extend(nodes);
            }
        }
    }

    println!("Day 8 Part 2: {}", anti_nodes.len());

    Ok(())
}