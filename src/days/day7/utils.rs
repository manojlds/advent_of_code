use std::fs;
use std::io;

pub fn read_input() -> io::Result<Vec<(i64, Vec<i64>)>> {
    let input = fs::read_to_string("inputs/day7/input.txt")?;

    let mut input_structured = Vec::new();

    for line in input.lines() {
        let mut split = line.split(": ");
        let result = split.next().unwrap().parse().unwrap();
        let operands = split.next().unwrap()
            .split_whitespace()
            .map(|o| o.parse::<i64>().unwrap())
            .collect();
        input_structured.push((result, operands));
    }

    return Ok(input_structured);
}

fn find_recursive(result: i64, operands: &Vec<i64>, current: i64, index: usize, is_part2: bool) -> bool {
    if index == operands.len() {
        return current == result;
    }

    if find_recursive(result, operands, current + operands[index], index + 1, is_part2) {
        return true;
    }
    
    if find_recursive(result, operands, current * operands[index], index + 1, is_part2) {
        return true;
    }

    if is_part2 {

        let concatenated = format!("{}{}", current, operands[index]);
        let concatenated_int = concatenated.parse::<i64>().unwrap();
        if find_recursive(result, operands, concatenated_int, index + 1, is_part2) {
            return true;
        }
    }

    return false;
}

fn find_expression(result: i64, operands: Vec<i64>, is_part2: bool) -> bool {
    if find_recursive(result, &operands, operands[0], 1, is_part2) {
        return true;
    }

    return false;
}

pub fn evaluate(inputs: Vec<(i64, Vec<i64>)>, is_part2: bool) -> i64{
    let mut sum = 0;
    for input in inputs{
        if find_expression(input.0, input.1, is_part2) {
            sum += input.0;
        }
    }
    return sum;
}
