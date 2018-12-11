use std::collections::LinkedList;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", compress_polymer(input).len());
    println!("Answer #2: {}", get_minimal_compression(input));
}

fn compress_polymer(input: &str) -> String {
    let mut s: LinkedList<char> = LinkedList::new();
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        match s.back() {
            Some(&c) => {
                if let Some(n) = combine(c, ch) {
                    s.push_back(n);
                } else {
                    s.pop_back();
                }
            }
            None => s.push_back(ch),
        }
    }
    s.into_iter().collect::<String>()
}

fn get_candidate_polymers(input: &str) -> Vec<usize> {
    let s = compress_polymer(input);
    let mut candidates = vec![];

    for a in (0..26).map(|x| CharPair((x + 'a' as u8) as char, (x + 'A' as u8) as char)) {
        let mut c = s.clone().replace(a.0, "").replace(a.1, "");
        candidates.push(compress_polymer(&c).len())
    }
    candidates
}

fn get_minimal_compression(input: &str) -> usize {
    let cands = get_candidate_polymers(input);
    *cands.iter().min().unwrap()
}

fn combine(a: char, b: char) -> Option<char> {
    if a.to_lowercase().to_string() == b.to_lowercase().to_string() {
        if a.is_lowercase() && b.is_uppercase() {
            return None;
        }
        if b.is_lowercase() && a.is_uppercase() {
            return None;
        }
    }
    Some(b)
}

struct CharPair(char, char);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress_polymer() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(compress_polymer(input).len(), 10);
    }
    #[test]
    fn test_get_candidate_polymers() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(get_candidate_polymers(input).iter().min().unwrap(), &4);
    }
}
