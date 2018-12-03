use std::str::FromStr;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::thread;
use std::time::Duration;

type Index = usize;


fn main() {

    // let input = include_str!("input.txt");
    // let instructions: Vec<Instr> = parse_input(input);

    // println!("Answer #1: {}", find_first_freq(&instructions));

    // let (send_to_a, recv_a): (Sender<Registry>, Receiver<Registry>) = mpsc::channel();
    // let (send_to_b, recv_b): (Sender<Registry>, Receiver<Registry>) = mpsc::channel();

    // let mut reg_a = Registry::new(&instructions, send_to_b, recv_a);
    // let mut reg_b = Registry::new(&instructions, send_to_a, recv_b);

    // reg_a.map.insert('p', 0);
    // reg_b.map.insert('p', 1);

}

fn find_first_freq(input: &Vec<Instr>) -> isize { unimplemented!() }

fn parse_input(input: &str) -> Vec<Instr> {   
    input.lines().map(|x| x.parse::<Instr>().unwrap()).collect()
}

#[derive(Debug)]
struct Registry<'a> {
    map: HashMap<char, isize>,
    index: usize,
    last_played: isize,
    send_count: isize,
    instr: &'a Vec<Instr>,
    send: mpsc::Sender<Registry<'a>>,
    recv: mpsc::Receiver<Registry<'a>>,
}

impl<'a> Registry<'a> {
    fn new(instructions: &'a Vec<Instr>, send: Sender<Registry<'a>>, recv: Receiver<Registry<'a>>) -> Registry<'a> {
        Registry {
            map: HashMap::new(),
            instr: instructions,
            index: 0,
            send_count: 0,
            last_played: 0,
            send: send,
            recv: recv,
        }
    }
    fn next(&mut self) -> Option<Index> {
        match self.instr[self.index] {
            Instr::Send(v) => {
                self.send(v);
                self.shift(1);
            }
            Instr::Set(val, to_val) => {
                self.set(val, to_val);
                self.shift(1);
            }
            Instr::Add(val, to_val) => {
                self.add(val, to_val);
                self.shift(1);
            }
            Instr::Multiply(val, by_val) => {
                self.multiply(val, by_val);
                self.shift(1);
            }
            Instr::Mod(val, by_val) => {
                self.modulo(val, by_val);
                self.shift(1);
            }
            Instr::Receive(val) => {
                self.receive(val);
                self.shift(1);
            }
            Instr::Jump(gtz, offset) => {
                match self.jump(gtz, offset) {
                    Some(ofs)   => { self.shift(ofs); },
                    None        => { self.shift(1); }
                }
            }
        }


        match self.index {
            index if index >= self.instr.len() => None,
            index if index < 0 => None,
            index => Some(index),
        }
    }
    fn get_value(&mut self, value: Value) -> isize {
        match value {
            Value::Frequency(x) => x,
            Value::Register(x) => *self.map.entry(x).or_insert(0),
        }
    }
    fn send() -> () {}
    // snd X plays a sound with a frequency equal to the value of X.
    fn set(&mut self, val: Value, to_val: Value) {
        // set X Y sets register X to the value of Y.
        let to_value = self.get_value(to_val);

        if let Value::Register(x) = val {
            let set_value = self.map.entry(x).or_insert(0);
            *set_value = to_value;
        }
    }
    
    fn add(&mut self, val: Value, to_val: Value) {
        // add X Y increases register X by the value of Y.
        let to_value = self.get_value(to_val);

        if let Value::Register(x) = val {
            let set_value = self.map.entry(x).or_insert(0);
            *set_value += to_value;
        }
    }
    
    fn multiply() -> () {}
    // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    fn modulo() -> () {}
    // mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
    fn receive() -> () {}
    // rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
    fn jump() -> () {}
    // jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)

}

#[derive(Debug, Clone)]
enum Instr {
    Send(Value),
    Set(Value, Value),
    Add(Value, Value),
    Multiply(Value, Value),
    Mod(Value, Value),
    Receive(Value),
    Jump(Value, Value),
}

impl FromStr for Instr {
    type Err = InstrParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split_whitespace().collect();
        let mut i = split.into_iter();

        match i.next() {
            Some("snd") => Ok(Instr::Send(i.next().unwrap().parse::<Value>().unwrap())),
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
            Some("rcv") => Ok(Instr::Receive(i.next().unwrap().parse::<Value>().unwrap())),
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

#[derive(Debug, Clone)]
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
        let input = include_str!("test_input.txt");
        let instructions: Vec<Instr> = parse_input(input);

        assert_eq!(find_first_freq(&instructions), 4);
    }
}