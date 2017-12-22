use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", first_loop(input));

}

type Bank = u32;
type Memory = Vec<u32>;
type State = Memory;
type Path = HashSet<Memory>;

fn first_loop(input: &str) -> u32 {
    // println!("##### DEBUG #####");
    let mut memory: Memory = input
        .split_whitespace()
        .map(|x| x.parse::<Bank>().unwrap())
        .collect();
    let mut visited = Path::new();
    let mut count = 0;
    loop {
        if visited.insert(memory.clone()) == true {
            // println!("VISITED: {:?}", visited);
            let &(index, max) = &memory.iter().enumerate().fold(
                (0, 0),
                |(i, v), (index, value)| if *value > v
                {
                    (index, *value)
                } else {
                    (i, v)
                },
            );
            // println!("DEBUG: max:{:?} at index: {:?} \t {:?}", max, index, memory);
            let mut dist = max;
            let len = &memory.len();

            memory[index as usize] = 0;

            let mut iter = memory.clone().into_iter().enumerate().cycle().skip((index + 1) as usize);

            while let Some((i, v)) = iter.next() {
                if dist > 0 {
                    if let Some(&mut value) = memory.get_mut(i) {
                        memory[i] = value + 1;
                        // println!("--- Adding 1 to {} at index {}\t {:?}", value, i, memory);
                        dist -= 1;
                    }
                    
                } else {
                    break;
                }
            }
            count += 1;
            // panic!();
        } else {
            break;
        }
    }
    count
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_loop() {
        let input = include_str!("test_input.txt");

        assert_eq!(first_loop(input), 5);
    }
}
