use std::collections::HashMap;

fn main() {
    // initialize new register
    let cpu = CPU::new();
    // initialize array of instructions
    let instruction_set: InstructionSet = include_str!("input.txt").lines().collect();
    // execute instructions against register struct
    
    // Print highest value of any register
    println!("Answer #1: {}", cpu.iter().map(|(k,v)| v ).max().unwrap());
}

type RegisterKey = &'static str;
type RegisterValue = isize;
type CPU = HashMap<RegisterKey, RegisterValue>;

type Instruction = &'static str;
type InstructionSet = Vec<Instruction>;