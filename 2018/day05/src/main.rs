use std::collections::VecDeque;

fn main() {
    
}

fn compress(input: &str) -> String {
    let mut s: VecDeque<char> = VecDeque::new();
    let mut chars = input.chars();

    let while Some(ch) = chars.next() {
        match s.last() {
            Some(c) => { for n in combine(c, ch) { s.push_back(n); }},
            None => s.push_back(ch)
        }
    }
    String.from_iter(s)
}

fn combine(a: char, b: char) -> Option<Vec<char>> {
    if a.to_lowercase == b.to_lowercase {
        if a.is_lowercase() && b.is_uppercase() { None }
        if b.is_lowercase() && a.is_uppercase() { None }
    }
    Some(vec![a, b])
}