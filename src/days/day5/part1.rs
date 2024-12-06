use std::collections::HashMap;

use crate::days::day5::input::read_input;

pub fn check_valid(page1: i32, page2: i32, orderings: &HashMap<i32, Vec<i32>>) -> bool {
    if orderings.contains_key(&page2) {
        let page_orderings = orderings.get(&page2).unwrap();
        if page_orderings.contains(&page1) {
            return false;
        }
    }
    return true;
    
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    
    let (before_orderings, updates, _) = read_input().expect("Failed to read input");
    let mut sum = 0;

    for update in updates {
        let mut is_valid = true;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !check_valid(update[i], update[j], &before_orderings) {
                    is_valid = false;
                    break 'outer;
                }
            }
        }
        if is_valid {
            sum += update[update.len() / 2]
    
            
        }
    }
    
    
    println!("Day 5 Part 1: {}", sum);

    Ok(())
}