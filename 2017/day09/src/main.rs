
type Score = usize;
type Depth = usize;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", score(input));
}

fn score(input: &str) -> Score {
    let mut input_iter = input.chars();
    
    let mut total = 0 as Score;
    let mut state = Parser::Stream;
    let mut depth = 0 as Depth;

    while let Some(ch) = input_iter.next() {
        match state {
            Parser::Stream => {
                match ch {
                    '{' => {
                        depth += 1;
                        total += depth;
                    }
                    '}' => { depth -= 1; }
                    '<' => { state = Parser::Garbage; }
                    _ => {} // Proceed
                }
            },
            Parser::Garbage => {
                match ch {
                    '>' => { state = Parser::Stream; }
                    '!' => { state = Parser::Skip; }
                    _ => {} // Proceed
                }
            },
            Parser::Skip => { state = Parser::Garbage }
        }
    }
    total
}

#[derive(Debug,PartialEq)]
enum Parser {
    Stream,
    Garbage,
    Skip
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_score(){
        assert_eq!(1, score("{}"));
        assert_eq!(6, score("{{{}}}"));
        assert_eq!(5, score("{{},{}}"));
        assert_eq!(16, score("{{{},{},{{}}}}"));
        assert_eq!(1, score("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}