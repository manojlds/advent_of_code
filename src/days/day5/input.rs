use std::fs;
use std::io;
use std::collections::HashMap;

pub fn read_input() -> io::Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>, HashMap<i32, Vec<i32>>)> {
    let input = fs::read_to_string("inputs/day5/input.txt")?;

    let mut before_orderings = HashMap::new();
    let mut after_orderings = HashMap::new();
    let mut updates = Vec::new();
    let mut is_updates_section = false;

    for line in input.lines() {
        if line == "" {
            is_updates_section = true;
            continue;
        }
        if is_updates_section {
            let pages: Vec<&str> = line.split(",").collect();
            updates.push(pages.iter().map(|page| page.parse().unwrap()).collect());

        } else {
            let parts: Vec<&str> = line.split("|").collect();
            let before = parts[0].parse().unwrap();
            let after = parts[1].parse().unwrap();
            before_orderings.entry(before).or_insert(Vec::new()).push(after);
            after_orderings.entry(after).or_insert(Vec::new()).push(before);
        }
    }

    Ok((before_orderings, updates, after_orderings))
}