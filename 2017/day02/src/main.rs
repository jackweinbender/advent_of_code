fn main() {
    let input = include_str!("input.txt");
    println!("Answer #1: {}", sum_checksum(input));
    println!("Answer #2: {}", div_checksum(input));
}

fn sum_checksum(input: &str) -> i32 {
    let mut checksum = 0;
    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        let cells:Vec<i32> = l.split_whitespace()
            .map(|x| i32::from_str_radix(x, 10).unwrap() ).collect();
        
        let min = cells.iter().cloned().min().unwrap();
        let max = cells.iter().cloned().max().unwrap();

        checksum += max - min;
    }
    checksum
}
fn div_checksum(input: &str) -> i32 {
    let mut checksum = 0;
    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        let cells:Vec<i32> = l.split_whitespace()
            .map(|x| i32::from_str_radix(x, 10).unwrap() ).collect();
        
        for (i, cell) in cells.iter().cloned().enumerate() {
            for cmp in cells.iter().skip(i+1){
                if cell % cmp == 0 { 
                    checksum += cell / cmp;
                    break;
                }
                if cmp % cell == 0 { 
                    checksum += cmp / cell;
                    break;
                }
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_checksum() {
        let input = include_str!("test_input_1.txt");

        assert_eq!(sum_checksum(input), 18);
    }
    #[test]
    fn test_div_checksum() {
        let input = include_str!("test_input_2.txt");

        assert_eq!(div_checksum(input), 9);
    }
}