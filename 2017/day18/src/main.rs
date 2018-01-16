
use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;


type Index = usize;

fn main() {
    let instructions: Vec<Instr> = include_str!("input.txt")
        .lines()
        .map(|x| x.parse::<Instr>().unwrap())
        .collect();
    let mut registry = Registry::new();

    let mut index = 0;
    let mut counter = 0;
    println!("Debugger::\n---------------\n");
    while let Some(i) = dispatch(index, &instructions, &mut registry) {
        index = i;

        /* Optimisations */
        if index == 4 {
            let base: isize = 2;
            registry.map.insert('a', 2147483648);
            registry.map.insert('i', 0);
            index = 7;
        }
        if index == 10 {
            let mut p = 826;
            let a = *registry.map.entry('a').or_insert(0);

            for _ in 0..127 {
                p *= 8505;
                p %= a;
                p *= 129749;
                p += 12345;
                p %= a;
            }
            {
                let b = registry.map.entry('b').or_insert(0);
                *b = p % 10000;
                registry.last_played = *b;
            }
            registry.map.insert('p', p);
            registry.map.insert('i', 0);
            index = 20;
        }

        // counter += 1;
        // if counter % (6) == 0 { break; }
    }
}

fn dispatch(index: usize, instructions: &Vec<Instr>, registry: &mut Registry) -> Option<Index> {

    if index >= instructions.len() {
        return None;
    }

    let mut new_index: isize = index as isize;
    println!("------------------------");
    println!("Index: {}:", new_index);

    let instruction = &instructions[index];
    println!("{:?}", registry);
    println!("{:?}", instruction);
    match instruction {
        &Instr::Sound(ref r) => {
            registry.sound(r);
            new_index += 1;
        }
        &Instr::Set(ref set_a, ref set_b) => {
            let b = {
                match *set_b {
                    Value::Register(reg) => *registry.map.entry(reg).or_insert(0),
                    Value::Frequency(f) => f,
                }
            };
            let mut a = {
                match *set_a {
                    Value::Register(x) => registry.map.entry(x).or_insert(0),
                    Value::Frequency(x) => {
                        panic!("Bad add");
                    }
                }
            };
            *a = b;
        }
        &Instr::Add(ref add_a, ref add_b) => {
            let b = {
                match *add_b {
                    Value::Frequency(x) => x,
                    Value::Register(x) => *registry.map.entry(x).or_insert(0),
                }
            };
            let mut a = {
                match *add_a {
                    Value::Register(x) => registry.map.entry(x).or_insert(0),
                    Value::Frequency(x) => {
                        panic!("Bad add");
                    }
                }
            };
            *a += b;
        }
        &Instr::Multiply(ref mult_a, ref mult_b) => {
            let b = {
                match *mult_b {
                    Value::Frequency(x) => x,
                    Value::Register(x) => *registry.map.entry(x).or_insert(0),
                }
            };
            let mut a = {
                match *mult_a {
                    Value::Register(x) => registry.map.entry(x).or_insert(0),
                    Value::Frequency(x) => {
                        panic!("Bad add");
                    }
                }
            };
            *a *= b;
        }
        &Instr::Mod(ref mod_a, ref mod_b) => {
            let b = {
                match *mod_b {
                    Value::Frequency(x) => x,
                    Value::Register(x) => *registry.map.entry(x).or_insert(0),
                }
            };
            let mut a = {
                match *mod_a {
                    Value::Register(x) => registry.map.entry(x).or_insert(0),
                    Value::Frequency(x) => {
                        panic!("Bad add");
                    }
                }
            };
            println!("Mod {:?} of {} = {}", b, a, *a % b);
            *a %= b;
        }
        &Instr::Recover(ref value) => {
            match *value {
                Value::Register(x) => {
                    let v = registry.map.entry(x).or_insert(0);
                    if *v != 0 {
                        println!("Answer #1: {:?}", registry.last_played);
                        return None;
                    }
                }
                Value::Frequency(x) => {
                    if x != 0 {
                        println!("Answer #1: {:?}", registry.last_played);
                        return None;
                    }
                }
            }
        }
        &Instr::Jump(ref jump_a, ref jump_b) => {
            let a = {
                match *jump_a {
                    Value::Register(r) => *registry.map.entry(r).or_insert(0),
                    Value::Frequency(f) => f,
                }
            };
            if a > 0 {
                match *jump_b {
                    Value::Register(r) => {
                        let i = new_index + *registry.map.entry(r).or_insert(0);
                        println!("Jumping to index {:?}", i as usize);
                        println!("{:?}", registry);
                        return Some(i as usize);
                    }
                    Value::Frequency(f) => {
                        let i = new_index + f;
                        println!("{:?}", registry);
                        println!("Jumping to index {:?}", i as usize);
                        return Some(i as usize);
                    }
                }
            }
        }
        err => panic!("\nPanic from Enum: {:?}", err),
    }

    if new_index < 0 {
        return None;
    }
    new_index += 1;
    println!("{:?}", registry);
    println!("Next: {:?}", new_index);
    Some(new_index as usize)
}

#[derive(Debug)]
struct Registry {
    map: HashMap<char, isize>,
    last_played: isize,
}

impl Registry {
    fn new() -> Registry {
        Registry {
            map: HashMap::new(),
            last_played: 0,
        }
    }
    fn sound(&mut self, value: &Value) -> () {
        match value {
            &Value::Register(r) => {
                let f = self.map.entry(r).or_insert(0);
                self.last_played = *f;
            }
            &Value::Frequency(f) => {
                self.last_played = f;
            }
        }
    }
}

#[derive(Debug)]
enum Instr {
    Sound(Value),
    Set(Value, Value),
    Add(Value, Value),
    Multiply(Value, Value),
    Mod(Value, Value),
    Recover(Value),
    Jump(Value, Value),
}

impl FromStr for Instr {
    type Err = InstrParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split_whitespace().collect();
        let mut i = split.into_iter();

        match i.next() {
            Some("snd") => Ok(Instr::Sound(i.next().unwrap().parse::<Value>().unwrap())),
            Some("set") => {
                Ok(Instr::Set(
                    i.next().unwrap().parse::<Value>().unwrap(),
                    i.next().unwrap().parse::<Value>().unwrap(),
                ))
            }
            Some("add") => {
                Ok(Instr::Add(
                    i.next().unwrap().parse::<Value>().unwrap(),
                    i.next().unwrap().parse::<Value>().unwrap(),
                ))
            }
            Some("mul") => {
                Ok(Instr::Multiply(
                    i.next().unwrap().parse::<Value>().unwrap(),
                    i.next().unwrap().parse::<Value>().unwrap(),
                ))
            }
            Some("mod") => {
                Ok(Instr::Mod(
                    i.next().unwrap().parse::<Value>().unwrap(),
                    i.next().unwrap().parse::<Value>().unwrap(),
                ))
            }
            Some("rcv") => Ok(Instr::Recover(i.next().unwrap().parse::<Value>().unwrap())),
            Some("jgz") => {
                Ok(Instr::Jump(
                    i.next().unwrap().parse::<Value>().unwrap(),
                    i.next().unwrap().parse::<Value>().unwrap(),
                ))
            }
            _ => Err(InstrParseErr {}),
        }
    }
}

#[derive(Debug)]
enum Value {
    Register(char),
    Frequency(isize),
}
#[derive(Debug)]
struct InstrParseErr();

impl FromStr for Value {
    type Err = InstrParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            match s.parse::<char>() {
                Ok(value) => {
                    if value.is_digit(10) {
                        Ok(Value::Frequency(value.to_digit(10).unwrap() as isize))
                    } else {
                        Ok(Value::Register(value))
                    }
                }
                Err(_) => Err(InstrParseErr {}),
            }
        } else {
            match s.parse::<isize>() {
                Ok(value) => Ok(Value::Frequency(value)),
                Err(_) => Err(InstrParseErr {}),
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_instructions() {
        let instructions: Vec<Instr> = include_str!("test_input.txt")
            .lines()
            .map(|x| x.parse::<Instr>().unwrap())
            .collect();
        let mut registry = Registry::new();

        let mut index = 0;
        while let Some(i) = dispatch(index, &instructions, &mut registry) {
            index = i;
        }

        assert_eq!(registry.last_played, 4);
    }
}
