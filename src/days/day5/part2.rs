use std::collections::HashMap;
use std::collections::HashSet;
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

pub fn find_middle(update: Vec<i32>, before_orderings: &HashMap<i32, Vec<i32>>, after_orderings: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    for i in 0..update.len() {
        let mut others = HashSet::new();
        for j in 0..update.len() {
            if i != j {
                others.insert(update[j]);
            }
        }
        let before_order: HashSet<_> = before_orderings.get(&update[i]).unwrap_or(&Vec::new()).iter().cloned().collect();
        let after_order: HashSet<_> = after_orderings.get(&update[i]).unwrap_or(&Vec::new()).iter().cloned().collect();

        let befores: HashSet<_> = before_order.intersection(&others).collect();
        let afters: HashSet<_> = after_order.intersection(&others).collect();

        if befores.len() == afters.len() && befores.len() == update.len() / 2 {
            return Some(update[i]);
        }
    }

    return None;
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    
    let (before_orderings, updates, after_orderings) = read_input().expect("Failed to read input");
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
        if !is_valid {
            
            find_middle(update, &before_orderings, &after_orderings).map(|x| sum += x);
            
        }
    }
    
    println!("Day 5 Part 2: {}", sum);

    Ok(())
}