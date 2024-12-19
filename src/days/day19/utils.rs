use std::io;
use std::fs;
use std::collections::{HashMap, HashSet};

pub fn can_form_pattern(pattern: &str, available_towels: &HashSet<String>, cache: &mut HashMap<String, bool>) -> bool {
    
    if cache.contains_key(pattern) {
        return cache[pattern];
    }
    
    if pattern.is_empty() {
        return true;
    }

    for i in 1..=pattern.len() {
        let prefix = &pattern[0..i];
        if available_towels.contains(prefix) {
            let suffix = &pattern[i..];
            if can_form_pattern(suffix, available_towels, cache) {
                cache.insert(pattern.to_string(), true);
                return true;
            }
        }
    }
    cache.insert(pattern.to_string(), false);
    false
}


pub fn check_patterns(available_towels: HashSet<String>, patterns: Vec<String>) -> usize {
    let mut count = 0;
    for pattern in patterns {
        if can_form_pattern(&pattern, &available_towels, &mut HashMap::new()) {
            count += 1;
        }
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