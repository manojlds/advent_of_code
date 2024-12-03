use std::fs;

#[derive(PartialEq)]
enum ReportState {
    Unknown,
    Descending,
    Ascending,
}

fn is_safe(report: Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut violations = 0;
    let mut trend = ReportState::Unknown;

    for pair in report.windows(2) {
        let diff = pair[1] - pair[0];
        let current_trend = if diff > 0 { ReportState::Ascending } else { ReportState::Descending };

        if trend == ReportState::Unknown {
            trend = current_trend;
        } else if trend != current_trend {
                violations += 1;
                if violations > 1 {
                    return false;
                }
                continue;
        }

        if diff.abs() > 3 || diff == 0 {
            violations += 1;
            if violations > 1 {
                return false;
            }
        }
        
    }
    true
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day2/input.txt")?;

    let mut num_safe_reports = 0;

    for line in input.lines() {
        let report = line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
        
        if is_safe(report) {
            num_safe_reports += 1;
        }
    }

    println!("Day 2 Part 2: {}", num_safe_reports);

    Ok(())
}