#![feature(slice_rotate)]

use std::str::FromStr;
use std::string::ParseError;

fn main() {
    let input = "uugsqrei";

    println!("Answer #1: {}", input.parse::<DiskDefragmenter>().unwrap().sum_used());
}

type Knot = Vec<Increment>;
type Length = usize;
type Increment = usize;
type Skip = usize;
type Index = usize;
type DenseHash = String;
type Binary = String;

struct DiskDefragmenter (Vec<KnotHash>);

impl DiskDefragmenter {
    fn new(input: &str, size: usize) -> DiskDefragmenter {
        let mut hashes = vec![];
        for i in 0..size {
            let row_input = format!("{}-{}", input, i);
            hashes.push( row_input.parse::<KnotHash>().unwrap() )
        }
        DiskDefragmenter(hashes)
    }
    fn sum_used(&self) -> usize {
        self.0.iter().map(|x| x.sum_bits() ).sum()
    }
}

impl FromStr for DiskDefragmenter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DiskDefragmenter::new(s, 128))
    }
}

#[derive(Debug)]
struct KnotHash(DenseHash);

impl KnotHash {
    fn new(input: &str, knot_length: usize) -> KnotHash {
        let mut knot: Knot = (0..knot_length).collect();
        let lengths = lengths_ascii(input);
        tie_knot(&mut knot, lengths, 64);

        KnotHash(dense_hash(&knot))
    }
    fn sum_bits(&self) -> usize {
        let bits = self.to_bits();
        bits.split("")
            .map(|x| x.parse::<usize>())
            .filter_map(Result::ok)
            .sum()
    }
    fn to_bits(&self) -> Binary {
        hash_to_bin(self.0.clone())
    }
    fn to_str(&self) -> DenseHash {
        self.0.clone()
    }
}

impl FromStr for KnotHash {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(KnotHash::new(s, 256))
    }
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

fn lengths_ascii(input: &str) -> Vec<Length> {
    let mut ascii_codepoints: Vec<usize> = input.chars().map(|x| x as usize).collect();
    let mut append_codepoints = vec![17, 31, 73, 47, 23];

    ascii_codepoints.append(&mut append_codepoints);

    ascii_codepoints
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

fn hex_to_bin(ch: String) -> Binary {
    let hex_int = usize::from_str_radix(ch.as_str(), 16);
    format!("{:04b}", hex_int.unwrap())
}

fn hash_to_bin(hash: DenseHash) -> Binary {
    let mut chs = hash.chars();
    let mut bin_vec: Vec<String> = vec![];

    while let Some(ch) = chs.next() {
        bin_vec.push( hex_to_bin( char::to_string(&ch) ) )
    }
    
    bin_vec.join("")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dense_hash_empty() {
        assert_eq!(
            String::from("a2582a3a0e66e6e86e3812dcb672a272"),
            "".parse::<KnotHash>().unwrap().to_str()
        );
        assert_eq!(
            String::from("33efeb34ea91902bb2f59c9920caa6cd"),
            "AoC 2017".parse::<KnotHash>().unwrap().to_str()
        );
    }
    #[test]
    fn test_hex_to_bin() {
        assert_eq!(hex_to_bin(String::from("0")), "0000");
        assert_eq!(hex_to_bin(String::from("1")), "0001");
        assert_eq!(hex_to_bin(String::from("e")), "1110");
        assert_eq!(hex_to_bin(String::from("f")), "1111");
    }
    #[test]
    fn test_hash_to_bin() {
        assert_eq!(hash_to_bin(String::from("a0c2017")), "1010000011000010000000010111");
    }
    #[test]
    fn test_sum_disk() {
        let disk = "flqrgnkx".parse::<DiskDefragmenter>().unwrap();
        assert_eq!(8108, disk.sum_used());
    }
}
