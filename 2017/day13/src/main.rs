use std::string::ParseError;
use std::str::FromStr;

type Severity = usize;
type Depth = usize;
type Range = usize;
type Time = usize;

fn main() {
    let input = include_str!("input.txt");

    let trip_severity: usize = trip_severity(input).iter().sum();
    println!("Answer #1: {:?}", trip_severity);
}

fn trip_severity(input: &str) -> Vec<Severity> {
    input
        .lines()
        .map(|x| x.parse::<Scanner>().unwrap())
        .map(|x| x.caught_at(x.0))
        .collect()
}

struct Scanner(Depth, Range);
impl Scanner {
    fn caught_at(&self, time: Time) -> Severity {
        let full_range = (2 * self.1) - 2;
        match time % full_range {
            0 => self.0 * self.1,
            _ => 0,
        }
    }
}

impl FromStr for Scanner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanner: Vec<&str> = s.split(": ").collect();
        Ok(Scanner(
            usize::from_str_radix(scanner[0], 10).unwrap(),
            usize::from_str_radix(scanner[1], 10).unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_severity() {
        let input = include_str!("test_input.txt");

        let trip_severity: usize = trip_severity(input).iter().sum();
        assert_eq!(24, trip_severity);
    }
}
