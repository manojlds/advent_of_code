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

fn one_element_dropped(vec: Vec<i32>) -> Vec<Vec<i32>> {
    (0..vec.len())
        .map(|i| {
            let mut new_vec = vec.clone();
            new_vec.remove(i);
            new_vec
        })
        .collect()
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day2/input.txt")?;

    let mut num_safe_reports = 0;

    for line in input.lines() {
        let report = line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

        let one_element_dropped_reports = one_element_dropped(report);

        let any_safe = one_element_dropped_reports
            .iter()
            .any(|report| {
                is_safe(&report)
            });
        
        if any_safe {
            num_safe_reports += 1;
        }
    }

    println!("Day 2 Part 2: {}", num_safe_reports);

    Ok(())
}