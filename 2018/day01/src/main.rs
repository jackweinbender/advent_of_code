use std::collections::HashSet;
type Frequency = i32;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", sum_calibrate(input)); // 425
    println!("Answer #2: {}", cycle_calibrate(input)); // 57538
}

fn parse_input(input: &str) -> Vec<Frequency> {
    input
        .split_whitespace()
        .filter_map(|l| l.parse::<Frequency>().ok())
        .collect()
}

fn sum_calibrate(input: &str) -> Frequency {
    let freqs = parse_input(input);
    freqs.iter().sum::<i32>()
}

fn cycle_calibrate(input: &str) -> Frequency {
    let freqs = parse_input(input);
    let mut freq = 0;

    let mut visited = HashSet::new();
    visited.insert(0);

    let mut it = freqs.iter().cycle();

    while let Some(n) = it.next() {
        freq = freq + n;
        if !visited.insert(freq) {
            break;
        }
    }
    return freq;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_calibrate() {
        assert_eq!(sum_calibrate("+1 +1 +1"), 3);
        assert_eq!(sum_calibrate("+1 +1 -2"), 0);
        assert_eq!(sum_calibrate("-1 -2 -3"), -6);
    }
    #[test]
    fn test_cycle_calibrate() {
        assert_eq!(cycle_calibrate("+1 -1"), 0);
        assert_eq!(cycle_calibrate("+3 +3 +4 -2 -4"), 10);
        assert_eq!(cycle_calibrate("-6 +3 +8 +5 -6"), 5);
        assert_eq!(cycle_calibrate("+7 +7 -2 -7 -4"), 14);
    }
}
