use std::collections::HashMap;

type Count = usize;

fn main() {
    let input = include_str!("input.txt");
    let box_ids = parse_input(input);

    println!("Answer #1: {}", BoxID::checksum(&box_ids));
    println!("Answer #2: {}", BoxID::find_box(&box_ids).unwrap());
}

fn parse_input(input: &'static str) -> Vec<BoxID> {
    input.lines().map(|l| BoxID { id: l.trim() }).collect()
}

struct BoxID {
    id: &'static str,
}
impl BoxID {
    fn find_box(ids: &Vec<BoxID>) -> Option<String> {
        for id in ids {
            for i in ids {
                if let Some(common) = BoxID::diff(id.id, i.id) {
                    return Some(common);
                }
            }
        }
        None
    }
    fn diff(a: &str, b: &str) -> Option<String> {
        let mut diffs = 0;
        let mut common = String::new();
        let mut a_iter = a.chars();
        let mut b_iter = b.chars();
        while let Some(ch_a) = a_iter.next() {
            if let Some(ch_b) = b_iter.next() {
                if diffs > 1 {
                    break;
                }
                if ch_a == ch_b {
                    common.push(ch_a);
                } else {
                    diffs += 1;
                }
            }
        }
        if diffs == 1 {
            Some(common)
        } else {
            None
        }
    }
    fn checksum(ids: &Vec<BoxID>) -> usize {
        let (twos, threes) =
            ids.iter()
                .map(|b| BoxID::check(b))
                .fold((0, 0), |(mut a, mut b), (x, y)| {
                    if x >= 1 {
                        a += 1
                    }
                    if y >= 1 {
                        b += 1
                    }

                    (a, b)
                });
        twos * threes
    }

    fn check(box_id: &BoxID) -> (Count, Count) {
        let mut letter_counter = HashMap::new();
        for ch in box_id.id.chars() {
            let counter = letter_counter.entry(ch).or_insert(0);
            *counter += 1;
        }
        let twos = letter_counter
            .values()
            .filter(|v| **v == 2)
            .collect::<Vec<&usize>>()
            .len();
        let threes = letter_counter
            .values()
            .filter(|v| **v == 3)
            .collect::<Vec<&usize>>()
            .len();

        (twos, threes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check() {
        let id_1 = BoxID { id: "abcdef" };
        let id_2 = BoxID { id: "bababc" };
        let id_3 = BoxID { id: "abbcde" };

        assert_eq!(BoxID::check(&id_1), (0, 0));
        assert_eq!(BoxID::check(&id_2), (1, 1));
        assert_eq!(BoxID::check(&id_3), (1, 0));
    }
    #[test]
    fn test_checksum() {
        let input = include_str!("test_input.txt");
        let box_ids = parse_input(input);

        assert_eq!(BoxID::checksum(&box_ids), 12);
    }
    #[test]
    fn test_find_box() {
        let input = include_str!("test_input_2.txt");
        let box_ids = parse_input(input);

        assert_eq!(BoxID::find_box(&box_ids).unwrap(), String::from("fgij"));
    }
}
