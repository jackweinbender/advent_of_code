use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", run(input))
}

type Instruction = i32;
type Index = i32;
type InstructionSet = Vec<Instruction>;


fn run(input: &str) -> i32 {
    let mut instruction_set = process(input);
    let mut index: i32 = 0;
    let mut count = 0;

    while let Some(i) = instruction_set.get_mut(index as usize){
        index += *i;
        *i += 1;
        count += 1;
    }
    count
}

fn process(input: &str) -> InstructionSet {
    input.lines()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect()
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