use std::collections::HashMap;
use std::str::FromStr;

type CPU = HashMap<RegisterKey, RegisterValue>;
type RegisterKey = String;
type RegisterValue = isize;
type InstructionSet = Vec<Instruction>;

fn main() {
    let instruction_set: InstructionSet = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    println!("Answer #1: {}", max_final_register(instruction_set));
}

fn max_final_register(instructions: InstructionSet) -> isize {
    let mut computer = Computer::new();
    for instruction in instructions {
        computer.apply_instruction(instruction);
    }
    computer.cpu.into_iter().map(|(_,v)| v ).max().unwrap()
}

#[derive(Debug)]
struct Computer {
    cpu: HashMap<RegisterKey, RegisterValue>,
}

impl Computer {
    fn new() -> Computer {
        Computer { cpu: CPU::new() }
    }
    fn apply_instruction(&mut self, mut instruction: Instruction) -> () {
        if instruction.evaluate_condition(self) {
            let offset = instruction.offset();
            *self.cpu.entry(instruction.register).or_insert(0) += offset;
        }
    }
}

#[derive(Debug)]
struct Instruction {
    register: RegisterKey,
    delta: RegisterValue,
    operation: Operation,
    guard: ConditionStatment,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let line: Vec<&str> = s.split_whitespace().collect();
        let instruction = Instruction {
            register: String::from(line[0]),
            operation: line[1].parse().unwrap(),
            delta: isize::from_str_radix(line[2], 10).unwrap(),
            guard: ConditionStatment {
                register: String::from(line[4]),
                condition: line[5].parse().unwrap(),
                value: isize::from_str_radix(line[6], 10).unwrap(),
            }
        };
        Ok(instruction)
    }
}

impl Instruction {
    fn offset(&self) -> RegisterValue {
        match self.operation {
            Operation::Increment => self.delta,
            Operation::Decrement => self.delta * -1,
        }
    }
    fn evaluate_condition(&mut self, computer: &mut Computer) -> bool {
        let register_value = computer.cpu.entry(self.guard.register.clone()).or_insert(0);
        match self.guard.condition {
            Condition::GreaterThan => *register_value > self.guard.value,
            Condition::LessThan => *register_value < self.guard.value,
            Condition::GreaterThanEq => *register_value >= self.guard.value,
            Condition::LessThanEq => *register_value <= self.guard.value,
            Condition::Equal => *register_value == self.guard.value,
            Condition::NotEqual => *register_value != self.guard.value,
        }
    }
}

#[derive(Debug)]
struct ConditionStatment {
    condition: Condition,
    register: RegisterKey,
    value: RegisterValue,
}

#[derive(Debug)]
enum Operation {
    Increment,
    Decrement,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, ()> {
        match s {
            "inc" => Ok(Operation::Increment),
            "dec" => Ok(Operation::Decrement),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Condition {
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    Equal,
    NotEqual,
}
impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Condition, ()> {
        match s {
            ">" => Ok(Condition::GreaterThan),
            "<" => Ok(Condition::LessThan),
            ">=" => Ok(Condition::GreaterThanEq),
            "<=" => Ok(Condition::LessThanEq),
            "==" => Ok(Condition::Equal),
            "!=" => Ok(Condition::NotEqual),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_final_register() {
        let instructions: InstructionSet = include_str!("test_input.txt")
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

        assert_eq!(1, max_final_register(instructions));
    }
}