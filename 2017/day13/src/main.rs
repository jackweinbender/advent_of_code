use std::string::ParseError;
use std::str::FromStr;

type Severity = usize;
type Depth = usize;
type Range = usize;
type Time = usize;
type Trip = Vec<Scanner>;

fn main() {
    let input = include_str!("input.txt");

    let trip = get_trip(input);
    println!("Answer #1: {:?}", trip_severity(&trip, 0));
    println!("Answer #2: {:?}", shortest_delay(&trip));
}

fn shortest_delay(trip: &Trip) -> Time {
    let mut offset = 0;
    loop {
        if is_clean_trip(trip, offset) {
            return offset;
        } else {
            offset += 1
        } // Do nothing
    }
}

fn trip_severity(trip: &Trip, offset: Time) -> Severity {
    trip.iter().filter_map(|x| x.caught_at(x.0 + offset)).sum()
}

fn is_clean_trip(trip: &Trip, offset: Time) -> bool {
    trip.iter().map(|x| x.caught_at(x.0 + offset)).all(
        |x| x == None,
    )
}

fn get_trip(input: &str) -> Trip {
    input
        .lines()
        .map(|x| x.parse::<Scanner>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Scanner(Depth, Range);
impl Scanner {
    fn caught_at(&self, time: Time) -> Option<Severity> {
        let full_range = (2 * self.1) - 2;
        match time % full_range {
            0 => Some(self.0 * self.1),
            _ => None,
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

        let trip = get_trip(input);
        assert_eq!(24, trip_severity(&trip, 0));
        assert_eq!(10, shortest_delay(&trip));
    }
}
