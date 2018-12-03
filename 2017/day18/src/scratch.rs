
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::VecDeque;

type Index = usize;

fn main() {
    let input = include_str!("input.txt");
    let instructions: Vec<Instr> = parse_input(input);
        
    // let mut part_1 = Registry::new(&instructions);
    // part_1.next_receive();
    
    // println!("Answer #1: {}", part_1.last_played);
    
    // Part 2
    let mut part_2a = Registry::new(&instructions);
    let mut part_2b = Registry::new(&instructions);

    part_2a.map.insert('p', 0);
    part_2b.map.insert('p', 1);

    part_2a.next_receive();

    while !part_2a.is_spent() || !part_2b.is_spent() {
        println!("MAIN");
        part_2a.inbox.append(&mut part_2b.outbox);
        while let Some(_) = part_2b.next() {println!("*A");}

        part_2b.inbox.append(&mut part_2a.outbox);
        while let Some(_) = part_2a.next() { }
    }
    
    println!("Answer #2: {:?}", part_2a.send_count);
}

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
    outbox: VecDeque<isize>,
    inbox: VecDeque<isize>,
}

impl<'a> Registry<'a> {
    fn new(instructions: &'a Vec<Instr>) -> Registry<'a> {
        Registry {
            map: HashMap::new(),
            instr: instructions,
            index: 0,
            send_count: 0,
            last_played: 0,
            outbox: VecDeque::new(),
            inbox: VecDeque::new(),
        }
    }
    fn next(&mut self) -> Option<Index> {
        let mut new_index = self.index as isize + 1;
        // println!("{:?}", self.instr[self.index].clone());
        match self.instr[self.index].clone() {
            Instr::Send(v) => {
                self.send(v);
            }
            Instr::Set(val, to_val) => {
                self.set(val, to_val);
            }
            Instr::Add(val, to_val) => {
                self.add(val, to_val);
            }
            Instr::Multiply(val, by_val) => {
                self.multiply(val, by_val);
            }
            Instr::Mod(val, by_val) => {
                self.modulo(val, by_val);
            }
            Instr::Receive(val) => {
                println!("RECEIEVING");
                match self.inbox.pop_front() {
                    Some(incoming) => {
                        self.receive(val, incoming);
                    }
                    None => new_index = self.index as isize,
                }
            }
            Instr::Jump(gtz, offset) => {
                match self.jump(gtz, offset) {
                    Some(ofs) => new_index += ofs,
                    _ => {}
                }
            }
        }
        match new_index {
            i if i == self.index as isize => None,
            i if i >= self.instr.len() as isize => None,
            i if i < 0 => None,
            i => {self.index = i as usize; Some(i as Index)},
        }
    }

    fn is_spent(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }

    fn send(&mut self, value: Value) {
        match value {
            Value::Register(r) => {
                let f = self.map.entry(r).or_insert(0);
                self.last_played = *f;
            }
            Value::Frequency(f) => {
                self.last_played = f;
            }
        }
        self.outbox.push_back(self.last_played);
        self.send_count += 1;
    }
    fn receive(&mut self, register: Value, incoming: isize) {
        if let Value::Register(r) = register {
            let reg = self.map.entry(r).or_insert(0);
            *reg = incoming;
        }
    }
    fn set(&mut self, val: Value, to_val: Value) {
        let to_value = self.get_value(to_val);

        if let Value::Register(x) = val {
            let set_value = self.map.entry(x).or_insert(0);
            *set_value = to_value;
        }
    }
    fn add(&mut self, val: Value, to_val: Value) {
        let to_value = self.get_value(to_val);

        if let Value::Register(x) = val {
            let set_value = self.map.entry(x).or_insert(0);
            *set_value += to_value;
        }
    }
    fn multiply(&mut self, val: Value, by_val: Value) {
        let by_value = self.get_value(by_val);

        if let Value::Register(x) = val {
            let set_value = self.map.entry(x).or_insert(0);
            *set_value *= by_value;
        }
    }
    fn modulo(&mut self, val: Value, by_val: Value) {
        let by_value = self.get_value(by_val);

        if let Value::Register(x) = val {
            let set_value = self.map.entry(x).or_insert(0);
            *set_value %= by_value;
        }
    }
    fn jump(&mut self, gtz: Value, ofs: Value) -> Option<isize> {
        let gt_zero = self.get_value(gtz);

        match ofs {
            Value::Frequency(i) if gt_zero > 0 => return Some(i),
            _ => return None,
        }
    }
    fn get_value(&mut self, value: Value) -> isize {
        match value {
            Value::Frequency(x) => x,
            Value::Register(x) => *self.map.entry(x).or_insert(0),
        }
    }
    fn next_receive(&mut self) -> () {
        while let Some(i) = self.next() {
            
            match self.instr[i] {
                Instr::Receive(_) => { break; }
                _ => { self.index = i; }
            }
            
            /* Optimisations */
            if self.index == 4 {
                println!("**Opt4-7**");
                self.map.insert('a', 2147483648);
                self.map.insert('i', 0);
                self.index = 7;
            }
            if self.index == 10 {
                println!("**Opt10-20**");
                let mut p = 826;
                let a = *self.map.entry('a').or_insert(0);

                for _ in 0..127 {
                    p *= 8505;
                    p %= a;
                    p *= 129749;
                    p += 12345;
                    p %= a;
                }
                {
                    let b = self.map.entry('b').or_insert(0);
                    *b = p % 10000;
                }

                if let Some(&b) = self.map.get(&'b'){
                    self.send(Value::Frequency(b));
                }
                
                self.map.insert('p', p);
                self.map.insert('i', 0);
                self.index = 20;
            }
        }
    }
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
        let instructions = parse_input(input);
        .lines()
        .map(|x| x.parse::<Instr>().unwrap())
        .collect();
        
        let mut first = Registry::new(&instructions);
        first.next_receive();

        assert_eq!(first.last_played, 7071);
    }
    // #[test]
    // fn test_count_send() {
    // }
}
