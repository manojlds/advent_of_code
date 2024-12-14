use std::io;
use std::fs;
use regex::Regex;

#[derive(Debug)]
pub struct Input {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64)
}

impl Input {
    pub fn with_increased_prize(&self, increment: i64) -> Self {
        Input {
            a: self.a,
            b: self.b,
            prize: (
                (self.prize.0 + increment),
                (self.prize.1 + increment),
            ),
        }
    }
}

fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}
pub fn solve_buttons(input: &Input) -> Option<(i64, i64)> {
    let (a_x, a_y) = input.a;
    let (b_x, b_y) = input.b;
    let (prize_x, prize_y) = input.prize;

    let det_m = determinant(a_x, b_x, a_y, b_y);

    if det_m == 0 {
        return None;
    }

    let det_m_a = determinant(prize_x, b_x, prize_y, b_y);
    let det_m_b = determinant(a_x, prize_x, a_y, prize_y);

    let n_a = det_m_a / det_m;
    let n_b = det_m_b / det_m;

    if det_m_a % det_m != 0 || det_m_b % det_m != 0 {
        return None;
    }

    if n_a >= 0 && n_b >= 0 {
        Some((n_a, n_b))
    } else {
        None
    }
}



pub fn read_input() -> io::Result<Vec<Input>> {
    let input = fs::read_to_string("inputs/day13/input.txt")?;
    let mut inputs: Vec<Input> = Vec::new();
    
    let button_a_re = Regex::new(r"Button A: X([+-]\d+), Y([+-]\d+)").unwrap();
    let button_b_re = Regex::new(r"Button B: X([+-]\d+), Y([+-]\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    for chunk in input.split("\n\n") {
        let mut a = (0, 0);
        let mut b = (0, 0);
        let mut prize = (0, 0);

        for line in chunk.lines() {
            if let Some(caps) = button_a_re.captures(line) {
                a = (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap()
                );
            } else if let Some(caps) = button_b_re.captures(line) {
                b = (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap()
                );
            } else if let Some(caps) = prize_re.captures(line) {
                prize = (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap()
                );
            }
        }

        inputs.push(Input { a, b, prize });
    }

    Ok(inputs)
}