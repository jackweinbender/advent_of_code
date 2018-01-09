#![feature(slice_rotate)]

type Knot = Vec<Increment>;
type Length = usize;
type Increment = usize;
type Skip = usize;
type Index = usize;

fn main() {
    let lengths: Vec<Length> = "46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204"
        .split(',')
        .map(|x| usize::from_str_radix(x, 10).unwrap())
        .collect();
    let mut knot: Knot = (0..256).collect();

    tie_knot(&mut knot, lengths);

    println!("Answer #1: {:?}", knot_product(knot));

}

fn tie_knot(knot: &mut Knot, lengths: Vec<Length>) -> () {
    let mut skip: Skip = 0;
    let mut index: Index = 0;
    let knot_len = knot.len();

    for length in lengths {
        if length > knot_len {
            continue;
        }

        reverse_slice(index, length, knot);

        index += length;
        index += skip;
        index %= knot_len;

        skip += 1;
    }
}

fn knot_product(knot: Knot) -> isize {
    knot.clone().into_iter().take(2).fold(1,|acc, x| acc * x as isize)
}

fn reverse_slice(index: Index, length: Increment, knot: &mut Knot) -> () {
    let knot_length = knot.len();

    knot.rotate(index);

    {
        let z = &mut knot[0..length];
        z.reverse();
    }

    knot.rotate(knot_length - index);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_hash_knot() {
        let lengths = "3,4,1,5"
            .split(',')
            .map(|x| usize::from_str_radix(x, 10).unwrap())
            .collect();
        let mut knot: Knot = (0..5).collect();

        tie_knot(&mut knot, lengths);

        assert_eq!(vec![3, 4, 2, 1, 0], knot);
        assert_eq!(12, knot_product(knot));
    }
}
