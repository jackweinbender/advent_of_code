#![feature(slice_rotate)]

fn main() {
    let input = include_str!("input.txt");

    println!("{}", sum(input))
}

fn sum(input: &str) -> u32 {
    let total: u32 = 0;

    

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        // 1122 produces a sum of 3 (1 + 2) 
        // because the first digit 
        // (1) matches the second digit and the third digit (2) matches the fourth digit.
        assert_eq!(sum("1122"), 3);
        // 1111 produces 4 
        // because each digit (all 1) matches the next.
        assert_eq!(sum("1111"), 4);
        // 1234 produces 0 
        // because no digit matches the next.
        assert_eq!(sum("1234"), 0);
        // 91212129 produces 9 
        // because the only digit that matches the next one is the last digit, 9.
        assert_eq!(sum("91212129"), 9);
    }
}