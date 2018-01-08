
type Score = usize;

fn main() {
    let input = include_str!("input.txt");
}

fn score(input: &str) -> Score {
    unimplemented!();
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