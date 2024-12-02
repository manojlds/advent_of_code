use std::collections::HashMap;
use std::fs;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day1/input.txt")?;

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    let mut occurrences = HashMap::new();
    for &num in &list2 {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = list1
        .iter()
        .map(|&num| num * occurrences.get(&num).unwrap_or(&0))
        .sum();

    println!("Day 1 Part 2: {}", similarity_score);

    Ok(())
}
