use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", run(input))
}

type Instruction = i32;
type Index = i32;
type InstructionSet = HashMap<Index, Instruction>;


fn run(input: &str) -> i32 {
    let mut instruction_set = process(input);
    let mut index = 0;
    let mut count = 0;
    loop {
        match step(&instruction_set, &index){
            Some(value) => {
                instruction_set.insert(index, value + 1);
                index += value;
                count += 1;
            }
            None => { break; }
        }
    }
    count
}

fn step(instr: &InstructionSet, index: &Index) -> Option<Instruction> {
    // println!("{:?}", index);
    if instr.contains_key(&index) {
        let v = *instr.get(index).unwrap();
        Some(v)
    } else {
        None
    }
}
fn process(input: &str) -> InstructionSet {
    let mut instruction_set = InstructionSet::new();
    let lines = (0..).zip(input.lines());

    for (idx, val) in lines {
        let v = Instruction::from_str_radix(val, 10)
            .expect("Error processing file");
        let k = idx;
        
        instruction_set.insert(k, v);
    }
    instruction_set
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count_steps() {
        let input = include_str!("test_input.txt");
        assert_eq!(run(input), 5);
    }
}

// Answer #1: 358309 <- WAY TOO LONG TO GET THIS