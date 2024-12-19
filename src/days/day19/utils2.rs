use std::io;
use std::fs;
use std::collections::{HashMap, HashSet};

pub fn count_patterns(pattern: &str, available_towels: &HashSet<String>, cache: &mut HashMap<String, usize>) -> usize {
    
    if cache.contains_key(pattern) {
        return cache[pattern];
    }
    
    if pattern.is_empty() {
        return 1;
    }

    let mut ways = 0;

    for i in 1..=pattern.len() {
        let prefix = &pattern[0..i];
        if available_towels.contains(prefix) {
            let suffix = &pattern[i..];
            ways += count_patterns(suffix, available_towels, cache);
        }
    }
    cache.insert(pattern.to_string(), ways);
    ways
}


pub fn check_patterns(available_towels: HashSet<String>, patterns: Vec<String>) -> usize {
    let mut count = 0;
    for pattern in patterns {
        let ways = count_patterns(&pattern, &available_towels, &mut HashMap::new());
        count += ways;

    }
    
    count
}


pub fn read_input() -> io::Result<(HashSet<String>, Vec<String>)> {
    let input = fs::read_to_string("inputs/day19/input.txt")?;

    let mut sections = input.split("\n\n");

    let available_towels = sections
        .next()
        .unwrap_or_default() 
        .split(", ")
        .map(|s| s.trim().to_string()) 
        .collect();

    let patterns = sections
        .next()
        .unwrap_or_default()
        .lines()
        .map(|line| line.to_string()) 
        .collect();

    Ok((available_towels, patterns))
}