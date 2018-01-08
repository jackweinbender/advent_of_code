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

struct CPU (HashMap<RegisterKey, RegisterValue>);
impl CPU {
    fn apply_instruction(&self, instruction: Instruction) -> () {
        // Apply the instruction to the appropriate registers

        
    }
}

type RegisterKey = &'static str;
type RegisterValue = isize;

type Instruction = &'static str;
type InstructionSet = Vec<Instruction>;

