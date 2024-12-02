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

    list1.sort_unstable();
    list2.sort_unstable();

    let total_difference: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Day 1 Part 1: {}", total_difference);
    Ok(())
}