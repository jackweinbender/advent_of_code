fn main() {
    let input = include_str!("input.txt");
    println!("Answer #1: {}", checksum(input));
}

fn checksum(input: &str) -> i32 {
    let mut checksum = 0;
    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        let mut cells:Vec<&str> = l.split_whitespace().collect();

        let cells:Vec<i32> = cells.into_iter()
            .map(|x| i32::from_str_radix(x, 10).unwrap() ).collect();
        
        let min = cells.iter().cloned().min().unwrap();
        let max = cells.iter().cloned().max().unwrap();

        checksum += max - min;
    }
    checksum
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checksum() {
        let input = include_str!("test_input.txt");

        assert_eq!(checksum(input), 18);
    }
}