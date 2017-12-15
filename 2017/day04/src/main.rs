fn main() {
    let input = include_str!("input.txt");

    let simple_valid_pwds: Vec<&str> = input.lines().filter(|x| is_valid_simple(x)).collect();
    println!("Answer #1: {}", simple_valid_pwds.len());

    let twofa_pwds: Vec<&str> = input.lines().filter(|x| is_valid_twofa(x)).collect();
    println!("Answer #2: {}", twofa_pwds.len());
}

fn is_valid_simple(phrase: &str) -> bool {
    let mut has_dupes: bool = false;
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort();

    let mut sorted = words.into_iter().peekable();

    while let Some(w) = sorted.next() {
        match sorted.peek() {
            Some(x) if &w == x => {
                has_dupes = true;
                break;
            }
            _ => {
                continue;
            }
        }
    }
    !has_dupes
}
fn is_valid_twofa(phrase: &str) -> bool {
    let mut has_dupes: bool = false;
    let mut words: Vec<Vec<char>> = phrase
        .split_whitespace()
        .map(|x| x.chars())
        .map(|x| x.collect::<Vec<char>>())
        .map(|mut x| {
            x.sort();
            x
        })
        .collect();
    words.sort();

    let mut sorted = words.into_iter().peekable();

    while let Some(w) = sorted.next() {
        match sorted.peek() {
            Some(x) if x.len() != w.len() => {
                continue;
            }
            Some(x) => {
                if *x == w {
                    has_dupes = true;
                    break;
                }
                continue;
            }
            _ => {
                break;
            }
        }
    }
    !has_dupes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid_simple() {
        assert_eq!(true, is_valid_simple("aa bb cc dd ee"));
        assert_eq!(false, is_valid_simple("aa bb cc dd aa"));
        assert_eq!(true, is_valid_simple("aa bb cc dd aaa"));
    }
    #[test]
    fn test_is_valid_twofa() {
        assert_eq!(true, is_valid_twofa("abcde fghij"));
        assert_eq!(false, is_valid_twofa("abcde xyz ecdab"));
        assert_eq!(true, is_valid_twofa("a ab abc abd abf abj"));
        assert_eq!(true, is_valid_twofa("iiii oiii ooii oooi oooo"));
        assert_eq!(false, is_valid_twofa("oiii ioii iioi iiio"));
    }
}
