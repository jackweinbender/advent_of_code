fn main() {
    let input = 361527;
    println!("Answer #1: {}", get_distance_from_num(input));
}

fn get_distance_from_num(num: i32) -> i32 {
    unimplemented!();
}

fn get_linear_pos(num: i32) -> (i32, i32) {
    let mut buffer = num;
    let mut layer = 0;
    let mut x_pos = 0;

    for i in 0.. {
        let offset = (8*i);
        if buffer == 1 {break;}
        if buffer - offset <= 1 {  
            layer = i;
            x_pos = buffer - 2;
            break;
        }
        buffer -= offset;
    }
    (layer, x_pos)
}

fn get_cartesian_pos() -> (i32, i32) {

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_linear_pos() {
        assert_eq!((0, 0), get_linear_pos(1));
        assert_eq!((1, 0), get_linear_pos(2));
        assert_eq!((1, 6), get_linear_pos(8));
        assert_eq!((1, 7), get_linear_pos(9));
        assert_eq!((2, 0), get_linear_pos(10));
        assert_eq!((2, 15), get_linear_pos(25));
        assert_eq!((3, 0), get_linear_pos(26));
    }
    fn test_get_cartesian_pos() {
        assert_eq!((0, 0), get_cartesian_pos(0, 0));
        assert_eq!((1, 0), get_cartesian_pos(1, 0));
        assert_eq!((0, 1), get_cartesian_pos(1, 6));
        assert_eq!((1, -1), get_cartesian_pos(1, 7));
        assert_eq!((2, -1), get_cartesian_pos(2, 0));
        assert_eq!((2, -2), get_cartesian_pos(2, 15));
        assert_eq!((3, -2), get_cartesian_pos(3, 0));
    }
}
