use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", first_loop(input));
    println!("Answer #2: {}", loop_len(input));

}

type Bank = u32;
type Memory = Vec<u32>;
type State = Memory;
type Path = HashSet<Memory>;
type PathIndexed = HashMap<Memory, u32>;

fn first_loop(input: &str) -> u32 {
    let mut memory: Memory = input
        .split_whitespace()
        .map(|x| x.parse::<Bank>().unwrap())
        .collect();
    let mut visited = Path::new();
    let mut count = 0;
    loop {
        if visited.insert(memory.clone()) == true {
            let &(index, max) = &memory.iter().enumerate().fold(
                (0, 0),
                |(i, v), (index, value)| if *value > v
                {
                    (index, *value)
                } else {
                    (i, v)
                },
            );
            let mut dist = max;
            let len = &memory.len();

            memory[index as usize] = 0;

            let mut iter = memory.clone().into_iter().enumerate().cycle().skip((index + 1) as usize);

            while let Some((i, v)) = iter.next() {
                if dist > 0 {
                    if let Some(&mut value) = memory.get_mut(i) {
                        memory[i] = value + 1;
                        dist -= 1;
                    }
                    
                } else {
                    break;
                }
            }
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn loop_len(input: &str) -> u32 {
    let mut memory: Memory = input
        .split_whitespace()
        .map(|x| x.parse::<Bank>().unwrap())
        .collect();
    let mut visited = PathIndexed::new();
    let mut count = 0;
    let mut loop_len = 0;
    loop {
        match visited.insert(memory.clone(), count) {
            None => {
                let &(index, max) = &memory.iter().enumerate().fold(
                    (0, 0),
                    |(i, v), (index, value)| if *value > v
                    {
                        (index, *value)
                    } else {
                        (i, v)
                    },
                );
                let mut dist = max;
                let len = &memory.len();

                memory[index as usize] = 0;

                let mut iter = memory.clone().into_iter().enumerate().cycle().skip((index + 1) as usize);

                while let Some((i, v)) = iter.next() {
                    if dist > 0 {
                        if let Some(&mut value) = memory.get_mut(i) {
                            memory[i] = value + 1;
                            dist -= 1;
                        }
                        
                    } else {
                        break;
                    }
                }
                count += 1;
            }
            Some(first_match) => {
                loop_len = count - first_match;
                break;
            }
        }
    }
    loop_len
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_loop() {
        let input = include_str!("test_input.txt");

        assert_eq!(first_loop(input), 5);
    }
    #[test]
    fn test_loop_len() {
        let input = include_str!("test_input.txt");

        assert_eq!(loop_len(input), 4);
    }
}
