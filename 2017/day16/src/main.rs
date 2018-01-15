#![feature(slice_rotate)]

use std::str::FromStr;
use std::string::ParseError;

fn main() {
    let mut programs: Programs = "abcdefghijklmnop".split("").filter(|&x| x != "").map(|x| String::from(x)).collect();
    let instructions = include_str!("input.txt").split(",").map(|x| x.parse::<Instruction>()).filter_map(Result::ok);

    for instruction in instructions {
        execute_instruction(&mut programs, instruction);
    }

    println!("{:?}", programs.join(""));


}

type Program = String;
type Index = usize;
type Programs = Vec<Program>;


enum Instruction {
    Spin(Index),
    Exchange(Index, Index),
    Partner(Program, Program),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = String::from(s);
        match &string.split_at(1) {
            &("s", index) => { Ok( Instruction::Spin( index.parse::<usize>().unwrap() ) ) },
            &("x", indexes) => { 
                let is: Vec<&str> = indexes.split("/").collect();
                Ok( Instruction::Exchange(is[0].parse::<usize>().unwrap(), is[1].parse::<usize>().unwrap()) ) },
            &("p", programs) => {
                let ps: Vec<&str> = programs.split("/").collect();
                Ok( Instruction::Partner(String::from(ps[0]), String::from(ps[1]) ) )},
            _ => { Ok(Instruction::Spin( 0 )) }
        }
    }
}

fn execute_instruction(programs: &mut Vec<Program>, instruction: Instruction) -> () {
    match instruction {
        Instruction::Spin(index) => spin(programs, index),
        Instruction::Exchange(index_a, index_b) => exchange(programs, (index_a, index_b)),
        Instruction::Partner(prgm_a, prgm_b) => partner(programs, (prgm_a, prgm_b)),
    }
}

fn spin(programs: &mut Vec<Program>, index: Index) -> () {
    let shift = programs.len() - index;
    programs.rotate( shift );
}

fn exchange(programs: &mut Vec<Program>, indexes: (Index, Index)) -> () {
    let (index_a, index_b) = indexes;
    programs.swap(index_a, index_b);
}

fn partner(programs: &mut Vec<Program>, partners: (Program, Program)) -> () {
    let partner_a = programs.iter().position(|x| *x == partners.0 );
    let partner_b = programs.iter().position(|x| *x == partners.1 );

    programs.swap(partner_a.unwrap(), partner_b.unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dance() {
        let mut input = "abcde".split("").filter(|x| *x != "").map(|x| String::from(x)).collect();

        spin(&mut input, 1);
        let expected_1: Vec<Program> = "eabcd".split("").filter(|x| *x != "").map(|x| String::from(x)).collect();
        assert_eq!(input, expected_1);

        exchange(&mut input, (3, 4));
        let expected_2: Vec<Program> = "eabdc".split("").filter(|x| *x != "").map(|x| String::from(x)).collect();
        assert_eq!(input, expected_2);

        partner(&mut input, (String::from("e"), String::from("b")));
        let expected_3: Vec<Program> = "baedc".split("").filter(|x| *x != "").map(|x| String::from(x)).collect();
        assert_eq!(input, expected_3);

    }
}
