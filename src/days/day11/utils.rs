use std::i32;
use std::io;
use std::fs;
use std::collections::HashMap;
use std::time::Instant;

pub fn read_input() -> io::Result<HashMap<u64, u64>> {
    let input = fs::read_to_string("inputs/day11/input.txt")?;
    let mut stones: HashMap<u64, u64> = HashMap::new();

    input.split_whitespace().for_each(|s| *stones.entry(s.parse().unwrap()).or_default() += 1);    

    Ok(stones)
}

fn split_number(num: u64) -> Option<(u64, u64)> {
    let len = num.ilog10() + 1;

    if len % 2 != 0 {
        return None;
    }

    let magnitude = 10u64.pow(len / 2);

    Some((num % magnitude, num / magnitude))
}

pub fn blink(input: HashMap<u64, u64>, times: i32) -> u64 {
    let mut current = input;
    for k in 1..=times {
        let start = Instant::now();
        let mut updated = HashMap::new();
        
        for (stone, count) in current {
            if stone == 0 {
                *updated.entry(1).or_default() += count;
            } else if let Some((first, second)) = split_number(stone) {
                *updated.entry(first).or_default() += count;
                *updated.entry(second).or_default() += count;
            } else {
                *updated.entry(stone * 2024).or_default() += count;
            }
        }
        current = updated;
        let duration = start.elapsed();
        println!("Blink: {}. Stones: {}. Time taken: {:.2?}", k, current.values().sum::<u64>(), duration);
    }

    current.values().sum::<u64>()
}

