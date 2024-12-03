use std::fs;

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report
        .windows(2)
        .all(|pair| pair[1] > pair[0] && (pair[1] - pair[0]) <= 3);
    let decreasing = report
        .windows(2)
        .all(|pair| pair[0] > pair[1] && (pair[0] - pair[1]) <= 3);
    
    increasing || decreasing
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day2/input.txt")?;

    let mut num_safe_reports = 0;

    for line in input.lines() {
        let report = line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
        
        if is_safe(&report) {
            num_safe_reports += 1;
        }
    }

    println!("Day 2 Part 1: {}", num_safe_reports);

    Ok(())
}