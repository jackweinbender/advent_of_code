#![feature(slice_rotate)]

type Program = char;
type Index = usize;

fn main() -> () {
    let instructions: Vec<Instruction> = include_str!("input.txt")
        .split(",")
        .filter_map(|x| parse_instruction(x))
        .collect();

    let mut input = vec![
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
    ];

    let loopy = 1000000000 % 60;

    for i in 0..loopy {
        dance(&mut input, &instructions);
        if i == 0 {
            println!(
                "Answer #1: {:?}",
                input.clone().into_iter().collect::<String>()
            );
        }

        if input ==
            vec![
                'a',
                'b',
                'c',
                'd',
                'e',
                'f',
                'g',
                'h',
                'i',
                'j',
                'k',
                'l',
                'm',
                'n',
                'o',
                'p',
            ]
        {
            println!("Loop at: {:?}", i);
        }
    }
        println!("Answer #2: {:?}", input.clone().into_iter().collect::<String>());
}
fn dance(mut input: &mut Vec<Program>, instructions: &Vec<Instruction>) -> () {
    for instr in instructions {
        dispatch_instruction(&mut input, instr);
    }
}
fn dispatch_instruction(programs: &mut Vec<Program>, instruction: &Instruction) -> () {
    match instruction {
        &Instruction::Spin(index) => {
            spin(programs, index);
        }
        &Instruction::Exchange(idx_a, idx_b) => {
            exchange(programs, (idx_a, idx_b));
        }
        &Instruction::Partner(pgm_a, pgm_b) => {
            partner(programs, (pgm_a, pgm_b));
        }
    }
}

fn spin(programs: &mut Vec<Program>, index: Index) -> () {
    let shift = programs.len() - index;
    programs.rotate(shift);
}

fn exchange(programs: &mut Vec<Program>, indexes: (Index, Index)) -> () {
    let (index_a, index_b) = indexes;
    programs.swap(index_a, index_b);
}

fn partner(programs: &mut Vec<Program>, partners: (Program, Program)) -> () {
    let partner_a = programs.iter().position(|x| *x == partners.0);
    let partner_b = programs.iter().position(|x| *x == partners.1);

    programs.swap(partner_a.unwrap(), partner_b.unwrap());
}

enum Instruction {
    Spin(Index),
    Exchange(Index, Index),
    Partner(Program, Program),
}

fn parse_instruction(input: &str) -> Option<Instruction> {
    match input.split_at(1) {
        ("s", tail) => {
            let index = tail.parse::<Index>().unwrap();
            Some(Instruction::Spin(index))
        }
        ("x", tail) => {
            let idxs: Vec<&str> = tail.split("/").collect();
            let a = idxs[0].parse::<Index>().unwrap();
            let b = idxs[1].parse::<Index>().unwrap();
            Some(Instruction::Exchange(a, b))
        }
        ("p", tail) => {
            let pgms: Vec<&str> = tail.split("/").collect();
            let a = pgms[0].parse::<Program>().unwrap();
            let b = pgms[1].parse::<Program>().unwrap();
            Some(Instruction::Partner(a, b))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spin() {
        let mut input = vec!['a', 'b', 'c', 'd', 'e'];
        spin(&mut input, 1);
        assert_eq!(input, vec!['e', 'a', 'b', 'c', 'd']);
    }
    #[test]
    fn test_exchange() {
        let mut input = vec!['e', 'a', 'b', 'c', 'd'];
        exchange(&mut input, (3, 4));
        assert_eq!(input, vec!['e', 'a', 'b', 'd', 'c']);
    }
    #[test]
    fn test_partners() {
        let mut input = vec!['e', 'a', 'b', 'd', 'c'];
        partner(&mut input, ('e', 'b'));
        assert_eq!(input, vec!['b', 'a', 'e', 'd', 'c']);
    }
    #[test]
    fn test_dispatch() {
        let mut input = vec!['a', 'b', 'c', 'd', 'e'];
        let instrs = vec![
            Instruction::Spin(1),
            Instruction::Exchange(3, 4),
            Instruction::Partner('e', 'b'),
        ];

        for instr in &instrs {
            dispatch_instruction(&mut input, instr);
        }

        assert_eq!(input, vec!['b', 'a', 'e', 'd', 'c']);
    }
}
