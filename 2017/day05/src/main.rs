fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", increment(input));
    println!("Answer #2: {}", decrement(input));
}

type Instruction = i32;
type Index = i32;
type InstructionSet = Vec<Instruction>;


fn increment(input: &str) -> Index {
    let mut instruction_set = process(input);
    let mut index: Index = 0;
    let mut count = 0;

    while let Some(i) = instruction_set.get_mut(index as usize){
        index += *i;
        *i += 1;
        count += 1;
    }
    count
}
fn decrement(input: &str) -> Index {
    let mut instruction_set = process(input);
    let mut index: Index = 0;
    let mut count = 0;

    while let Some(i) = instruction_set.get_mut(index as usize){
        index += *i;
        match i {
            _ if *i >= 3 => { *i -= 1; }
            _ => { *i += 1; }
        }
        count += 1;
    }
    count
}

fn process(input: &str) -> InstructionSet {
    input.lines()
        .filter_map(|v| v.parse::<Index>().ok())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count_steps_inc() {
        let input = include_str!("test_input.txt");
        assert_eq!(increment(input), 5);
    }
    #[test]
    fn test_count_steps_dec() {
        let input = include_str!("test_input.txt");
        assert_eq!(decrement(input), 10);
    }
}