use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", run(input))
}

type Instruction = i32;
type Index = i32;
type Next = (Index, InstructionSet);
type InstructionSet = HashMap<Index, Instruction>;


fn run(input: &str) -> i32 {
    let init = process(input);
    count_steps(init)
}
fn process(input: &str) -> Next {
    let mut instruction_set = InstructionSet::new();
    let lines = (0..).zip(input.lines());

    for (idx, val) in lines {
        let v = Instruction::from_str_radix(val, 10)
            .expect("Error processing file");
        let k = idx;
        
        instruction_set.insert(k, v);
    }
    (0, instruction_set)
}
fn count_steps(next: Next) -> i32 {
    let mut count = 0;
    let mut n = next;
    
    loop {
        match step(n) {
            Some(nx) => { 
                n = nx;
                count += 1;
            }
            _ => { break; }
        }
    }

    count
}

fn step(next: Next) -> Option<Next> {
    println!("{:?}", next.0);
    match next {
        (idx, instr) => {
            if instr.contains_key(&idx) {
                let v = instr.get(&idx).unwrap();
                let new_idx = idx + v;
                let mut new_instr = instr.clone();
                
                new_instr.insert(idx, v + 1);
                Some((new_idx, new_instr))
            } else {
                None
            }
        }
        _ => { None }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count_steps() {
        let input = include_str!("test_input.txt");
        let init = process(input);

        assert_eq!(count_steps(init), 5);

    }
}

// Answer #1: 358309 <- WAY TOO LONG TO GET THIS