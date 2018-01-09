#![feature(slice_rotate)]

type Knot = Vec<Increment>;
type Length = usize;
type Increment = usize;
type Skip = usize;
type Index = usize;
type DenseHash = String;

fn main() {
    let input = "46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204";

    // Answer # 1
    let lengths_numeric = lengths_numeric(input);
    let mut knot: Knot = (0..256).collect();
    tie_knot(&mut knot, lengths_numeric, 1);

    println!("Answer #1: {:?}", knot_product(knot));

    // Answer # 2
    let lengths_ascii = lengths_ascii(input);
    let mut knot: Knot = (0..256).collect();

    tie_knot(&mut knot, lengths_ascii, 64);

    println!("Answer #2: {:?}", dense_hash(&knot));

}

fn tie_knot(knot: &mut Knot, lengths: Vec<Length>, magnitude: usize) -> () {
    let mut skip: Skip = 0;
    let mut index: Index = 0;
    let knot_len = knot.len();
    for _ in 0..magnitude {
        for length in &lengths {
            if *length > knot_len {
                continue;
            }

            reverse_slice(index, *length, knot);

            index += length;
            index += skip;
            index %= knot_len;

            skip += 1;
        }
    }
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

fn lengths_numeric(input: &str) -> Vec<Length> {
    input
        .split(',')
        .map(|x| usize::from_str_radix(x, 10).unwrap())
        .collect()
}

fn lengths_ascii(input: &str) -> Vec<Length> {
    let mut ascii_codepoints: Vec<usize> = input.chars().map(|x| x as usize).collect();
    let mut append_codepoints = vec![17, 31, 73, 47, 23];

    ascii_codepoints.append(&mut append_codepoints);

    ascii_codepoints
}

fn knot_product(knot: Knot) -> isize {
    knot.clone().into_iter().take(2).fold(
        1,
        |acc, x| acc * x as isize,
    )
}

fn dense_hash(knot: &Knot) -> DenseHash {
    let xor_chunks = knot.chunks(16);
    let mut xor: Vec<usize> = vec![];
    for chunk in xor_chunks {
        let x = chunk.iter().fold(0, |acc, x| acc ^ x);
        xor.push(x);
    }

    let dense_hash = xor.iter()
        .map(|x| String::from(format!("{:x}", x)))
        .map(|x| if x.len() == 1 { format!("0{}", x) } else { x })
        .collect::<Vec<String>>()
        .join("");

    dense_hash
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

        tie_knot(&mut knot, lengths, 1);

        assert_eq!(vec![3, 4, 2, 1, 0], knot);
        assert_eq!(12, knot_product(knot));
    }
    #[test]
    fn test_to_ascii_codepoints() {
        let input = "1,2,3";
        assert_eq!(
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23],
            lengths_ascii(input)
        );
    }
    #[test]
    fn test_dense_hash_empty() {
        let mut knot: Knot = (0..256).collect();
        let lengths = lengths_ascii("");

        tie_knot(&mut knot, lengths, 64);

        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", dense_hash(&knot));
    }
    #[test]
    fn test_dense_hash_aoc() {
        let mut knot: Knot = (0..256).collect();
        let lengths = lengths_ascii("AoC 2017");

        tie_knot(&mut knot, lengths, 64);

        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", dense_hash(&knot));
    }
}
