use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = 361527;
    println!("Answer #1: {}", distance(input));
    println!("Answer #2: {}", first_val_over(input));
}

#[derive(PartialEq, Eq, Debug)]
struct Node(i32, i32, Compass);
impl Node {
    fn turn(&self) -> Node {
        match self {
            &Node(_, _, Compass::South) => {self.east()}
            &Node(_, _, Compass::East) => {self.north()}
            &Node(_, _, Compass::North) => {self.west()}
            &Node(_, _, Compass::West) => {self.south()}
        }
    }
    fn straight(&self) -> Node {
        match self {
            &Node(_, _, Compass::South) => {self.south()}
            &Node(_, _, Compass::East) => {self.east()}
            &Node(_, _, Compass::North) => {self.north()}
            &Node(_, _, Compass::West) => {self.west()}
        }
    }
    fn north(&self) -> Node {
        Node(self.0, self.1 + 1, Compass::North)
    }
    fn south(&self) -> Node {
        Node(self.0, self.1 - 1, Compass::South)
    }
    fn east(&self) -> Node {
        Node(self.0 + 1, self.1, Compass::East)
    }
    fn west(&self) -> Node {
        Node(self.0 - 1, self.1, Compass::West)
    }
    fn neighbors(&self) -> Vec<String> {
        vec![
            self.north().to_str(),
            self.south().to_str(),
            self.east().to_str(),
            self.west().to_str(),
            self.north().west().to_str(),
            self.south().west().to_str(),
            self.north().east().to_str(),
            self.south().east().to_str(),
        ]
    }
    fn from_center(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
    fn to_str(&self) -> String {
        format!("x{}y{}", self.0, self.1)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Compass {
    North,
    South,
    East,
    West
}
fn distance(val: i32) -> i32 {
    let d = walk(val);
    d.from_center()
}

fn walk(distance: i32) -> Node {
    
    let mut visited = HashSet::new();
    let mut current = Node(0, 0, Compass::South);
    
    for _ in 1..distance {

        visited.insert(current.to_str());
        let turn = current.turn().to_str();

        match visited.contains(&turn) {
            true => { current = current.straight(); }
            _ => { current = current.turn(); }
        }
    }
    current
}

fn first_val_over(distance: i32) -> i32 {
    let mut visited = HashMap::new();
    let mut current = Node(0, 0, Compass::South);
    let mut val = 1;
    visited.insert(current.to_str(), val);
    current = current.turn();

    while val <= distance {

        let ns = current.neighbors();
        let sum: i32 = ns.iter()
            .filter_map(|x| visited.get(x))
            .sum();

        visited.insert(current.to_str(), sum);

        match visited.contains_key(&current.turn().to_str()) {
            true => { current = current.straight(); }
            _ => { current = current.turn(); }
        }

        val = sum;
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance() {
        // Data from square 1 is carried 0 steps, since it's at the access port.
        assert_eq!(distance(1), 0);
        assert_eq!(distance(2), 1);
        assert_eq!(distance(3), 2);
        // Data from square 12 is carried 3 steps, such as: down, left, left.
        assert_eq!(distance(12), 3);
        // Data from square 23 is carried only 2 steps: up twice.
        assert_eq!(distance(23), 2);
        // Data from square 1024 must be carried 31 steps.
        assert_eq!(distance(1024), 31);
    }
    #[test]
    fn test_get_value() {
        assert_eq!(first_val_over(1), 2);
        assert_eq!(first_val_over(4), 5);
        assert_eq!(first_val_over(23), 25);
    }
}
