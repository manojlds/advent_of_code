use std::io;
use std::fs;

pub fn read_input() -> io::Result<Vec<String>> {
    let input = fs::read_to_string("inputs/day9/input.txt")?;

    let mut is_file = true;
    let mut id: u32 = 0;
    let mut result = Vec::new();
    for c in input.chars() {
        let c_int = c.to_digit(10).unwrap();

        if is_file {
            for _ in 0..c_int {
                result.push(id.to_string());
            }
            is_file = false;
            id += 1;
        } else {
            for _ in 0..c_int {
                result.push(String::from("."));
            }
            is_file = true;
        }
    }

    return Ok(result);
}

pub fn pack(input: &mut Vec<String>) {
    let mut left = 0;
    let mut right = input.len() - 1;

    while left < right {
        while left < right && input[left] != "." {
            left += 1;
        }

        while left < right && right > 0 && input[right - 1] == "." {
            right -= 1;
        }

        if left >= right {
            break;
        }

        input.swap(left, right - 1);
        left += 1;
        right -= 1;
    }
}

pub fn pack2(input: &mut Vec<String>) {
    let len = input.len();
    let mut left = 0;
    let mut right = len - 1;
    let mut run = 0;

    while right > left {
        while left < len && input[left] != "." {
            left += 1;
        }
    
        if left == len {
            break;
        }

        while right > left && input[right] == "." {
            right -= 1;
        }
        if right <= left {
            break;
        }

        let group_value = &input[right];
        let mut group_start = right;
        while group_start > left && input[group_start - 1] == *group_value {
            group_start -= 1;
        }
        let group_length = right - group_start + 1;

        let mut dots_left = left;
        while dots_left < len {
            while dots_left < len && input[dots_left] != "." {
                dots_left += 1;
            }
            if dots_left >= right {
                break;
            }

            let dots_start = dots_left;
            let mut dots_length = 0;
            while dots_left < len && input[dots_left] == "." {
                dots_length += 1;
                dots_left += 1;
            }

            if dots_length >= group_length {
                for k in 0..group_length {
                    input[dots_start + k] = input[group_start + k].clone();
                    input[group_start + k] = ".".to_string();
                }
                left = 0;
                break;
            } else {
                continue;
            }
        }

        if group_start == 0 {
            break;
        }
        right = group_start - 1;
    }
}


pub fn checksum(input: Vec<String>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if s == "." {
                return None;
            }
            let value = s.parse::<i64>().unwrap();
            return Some(i as i64 * value);
    }).sum()
}
