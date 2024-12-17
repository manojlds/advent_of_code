use std::fs;
use std::hash::Hash;
use std::io;
use std::collections::HashMap;
use regex::Regex;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use once_cell::sync::Lazy;

pub struct Computer {
    pub registers: HashMap<char, usize>
}

pub trait Instruction {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize>;

    fn get_operand(&self, operand: usize, registers: &HashMap<char, usize>) -> usize {
        match operand {
            0..=3 => operand,
            4 => *registers.get(&'A').unwrap(),
            5 => *registers.get(&'B').unwrap(),
            6 => *registers.get(&'C').unwrap(),
            _ => panic!("Invalid operand: {}", operand),
        }
    }
}

pub struct Adv;

impl Instruction for Adv {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        let operand = self.get_operand(operand, &registers);
        let denominator = 2usize.pow(operand as u32);
        registers.entry('A').and_modify(|e| *e /= denominator);
        *pointer += 2;
        None
    }
}

pub struct Bxl;

impl Instruction for Bxl {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize>{
        registers.entry('B').and_modify(|e| *e ^= operand);
        *pointer += 2;
        None
    }
}

pub struct Bst;

impl Instruction for Bst {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        let operand = self.get_operand(operand, &registers);
        let value = operand % 8;
        registers.insert('B', value);
        *pointer += 2;
        None
    }
}

pub struct Jnz;

impl Instruction for Jnz {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        if *registers.get(&'A').unwrap() != 0 {
            *pointer = operand;
        } else {
            *pointer += 2;
        }
        None
    }
}

pub struct Bxc;

impl Instruction for Bxc {
    fn execute(&self, _operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        let b_value = *registers.get(&'B').unwrap();
        let c_value = *registers.get(&'C').unwrap();
        registers.insert('B', b_value ^ c_value);
        *pointer += 2;
        None
    }
}

pub struct Out;

impl Instruction for Out {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        let operand = self.get_operand(operand, &registers);
        let value = operand % 8;
        *pointer += 2;

        Some(value)
    }
}

pub struct Bdv;

impl Instruction for Bdv {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        let operand = self.get_operand(operand, &registers);
        let denominator = 2usize.pow(operand as u32);
        let result = *registers.get(&'A').unwrap() / denominator;
        registers.insert('B', result);
        *pointer += 2;
        None
    }
}

pub struct Cdv;

impl Instruction for Cdv {
    fn execute(&self, operand: usize, registers: &mut HashMap<char, usize>, pointer: &mut usize) -> Option<usize> {
        let operand = self.get_operand(operand, &registers);
        let denominator = 2usize.pow(operand as u32);
        let result = *registers.get(&'A').unwrap() / denominator;
        registers.insert('C', result);
        *pointer += 2;
        None
    }
}

static INSTRUCTIONS: Lazy<HashMap<usize, Box<dyn Instruction  + Send + Sync>>> = Lazy::new(|| {
    let mut map: HashMap<usize, Box<dyn Instruction + Send + Sync>> = HashMap::new();
    map.insert(0, Box::new(Adv));
    map.insert(1, Box::new(Bxl));
    map.insert(2, Box::new(Bst));
    map.insert(3, Box::new(Jnz));
    map.insert(4, Box::new(Bxc));
    map.insert(5, Box::new(Out));
    map.insert(6, Box::new(Bdv));
    map.insert(7, Box::new(Cdv));
    map
});


impl Computer {
    pub fn new(input: String) -> Computer {
        let register_regex = Regex::new(r"Register ([A-Z]): (\d+)").unwrap();
        let mut registers = HashMap::new();

        for line in input.lines() {
            if let Some(captures) = register_regex.captures(line) {
                let register_name = &captures[1].chars().next().unwrap();
                let value: usize = captures[2].parse().unwrap();
    
                registers.insert(*register_name, value);
            }
        }

        Computer {
            registers
        }
    }

    pub fn set_register(&mut self, register: char, value: usize) {
        self.registers.insert(register, value);
    }

    pub fn print_state(&self) {
        println!("Registers: {:?}", self.registers)
    }   

    pub fn run(&mut self, program: Vec<usize>) -> Vec<usize> {
        let mut pointer: usize = 0;
        let mut outputs: Vec<usize> = Vec::new();
    
        while pointer < program.len() {
            let opcode = program[pointer];
    
            if let Some(instruction) = INSTRUCTIONS.get(&opcode) {
                if let Some(output) = instruction.execute(program[pointer + 1], &mut self.registers, &mut pointer) {
                    outputs.push(output);
                }
            } else {
                panic!("Invalid opcode: {} at pointer {}", opcode, pointer);
            }
        }
    
        outputs
    }
}

pub fn read_input() -> io::Result<(Computer, Vec<usize>)> {
    let input = fs::read_to_string("inputs/day17/input.txt")?;

    let mut sections = input.split("\n\n");

    let computer = Computer::new(sections.next().unwrap().to_string());

    let next_section = sections.next().unwrap().to_string();
    let program_part = next_section.strip_prefix("Program: ").unwrap();

    let program: Vec<usize> = program_part
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    Ok((computer, program))
}