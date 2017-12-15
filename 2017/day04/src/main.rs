fn main() {
    let input = include_str!("input.txt");
    
    let valid_pwds:Vec<&str> = input.lines().filter(|x| is_valid(x)).collect();

    println!("Answer #1: {}", valid_pwds.len());
}

fn is_valid(phrase: &str) -> bool {
    
    let mut has_dupes:bool = false;

    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort();
    let mut sorted = words.into_iter().peekable();

    while let Some(w) = sorted.next() {
        match sorted.peek() {
            Some(x) if &w == x => { has_dupes = true; break; }
            _ => { continue; }
        }
    }
    !has_dupes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid("aa bb cc dd ee"));
        assert_eq!(false, is_valid("aa bb cc dd aa"));
        assert_eq!(true, is_valid("aa bb cc dd aaa"));
    }
}