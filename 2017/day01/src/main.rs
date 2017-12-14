fn main() {
    let raw = include_str!("input.txt");
    
    println!("Answer #1: {}", sum(raw));
    println!("Answer #2: {}", sum_mid(raw));
}

fn sum(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut s = input.char_indices().peekable();
    while let Some((_, ch)) = s.next() {
        match s.peek() {
            Some(&(_, c)) if ch == c => { total += ch.to_digit(10).unwrap() }
            None => if input.starts_with(ch){ total += ch.to_digit(10).unwrap() }
            _ => {}
        }
    }
    total
}
fn sum_mid(input: &str) -> u32 {
    let mut total: u32 = 0;
    let len = input.len();
    
    let mut a = input.chars();
    let mut b = input.chars().cycle().skip(len / 2);

    while let Some(ch) = a.next() {
        match b.next() {
            Some(c) if ch == c => { total += ch.to_digit(10).unwrap() }
            _ => {}
        }
    }
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
    #[test]
    fn test_sum_mid() {
        // 1212 produces 6:
        // the list contains 4 items,
        // and all four digits match the digit 2 items ahead.
        assert_eq!(sum_mid("1212"), 6);
        // 1221 produces 0, 
        // because every comparison is between a 1 and a 2.
        assert_eq!(sum_mid("1221"), 0);
        // 123425 produces 4, 
        // because both 2s match each other, but no other digit has a match.
        assert_eq!(sum_mid("123425"), 4);
        // 123123 produces 12.
        assert_eq!(sum_mid("123123"), 12);
        // 12131415 produces 4.
        assert_eq!(sum_mid("12131415"), 4);
    }
}
