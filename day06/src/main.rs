use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
        
    println!("Answer #1L {}", first_loop(input));
     
}

type Bank = u32;
type Memory = Vec<u32>;
type State = Memory;
type Path = HashSet<Memory>;

fn first_loop(input: &str) -> u32 {
    let mut memory: Memory = input
        .split_whitespace()
        .map(|x| x.parse::<Bank>().unwrap())
        .collect();
    let mut visited = Path::new();
    let count = 0;

    while let true = visited.insert(memory.clone()) {
        unimplemented!();
    }

    count
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_loop(){
        let input = include_str!("test_input.txt");
        
        assert_eq!(first_loop(input), 5);
    }
}