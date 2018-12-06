extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

type ID = usize;

fn main() {
    let input = include_str!("input.txt");
    let claims = parse_input(input);

    let fabric = Fabric::new(claims);

    println!("Answer #1: {}", fabric.overlapping());
    println!("Answer #2: {}", fabric.independent().unwrap().id);
}

fn parse_input(input: &str) -> Vec<Claim> {

    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    input.lines().map(|l| {
        let caps = re.captures(l).unwrap();

        Claim {
            id: caps.get(1).map_or(0, |m| m.as_str().parse::<ID>().ok().unwrap() ),
            offset: Offset {
                x: caps.get(2).map_or(0, |m| m.as_str().parse::<ID>().ok().unwrap() ),
                y: caps.get(3).map_or(0, |m| m.as_str().parse::<ID>().ok().unwrap() )
            },
            size: Rect {
                x: caps.get(4).map_or(0, |m| m.as_str().parse::<ID>().ok().unwrap() ),
                y: caps.get(5).map_or(0, |m| m.as_str().parse::<ID>().ok().unwrap() )
            } 
        }

    }).collect()
}

struct Offset { x: usize, y: usize }

struct Rect { x: usize, y: usize }

type Point = ( usize, usize );

struct Claim {
    id: ID,
    offset: Offset,
    size: Rect
}

impl Claim {
    fn cells_from_claim(&self) -> HashSet<Point> {
        let mut cells = HashSet::new();
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                cells.insert( (self.offset.x + x, self.offset.y + y) );
            }
        }
        cells
    }
}

struct Fabric {
    claims: Vec<Claim>
}

impl Fabric {
    fn new(claims: Vec<Claim>) -> Fabric {
        Fabric{ claims: claims }
    }

    fn overlapping(&self) -> usize {
        let mut once = HashSet::new();
        let mut twice = HashSet::new();

        for claim in &self.claims {
            let cells = claim.cells_from_claim();

            for cell in cells {
                if !once.insert(cell){
                    twice.insert(cell);
                }
            }
        }
        twice.len()
    }

    fn independent(&self) -> Option<&Claim> {
        let mut map: HashMap<ID, HashSet<Point>> = HashMap::new();
        for claim in &self.claims {
            let cells = claim.cells_from_claim();
            map.insert(claim.id, cells);
        }

        for claim in &self.claims {
            let claim_cells = claim.cells_from_claim();
            if self.claims.iter().all(|c| {
                if claim.id == c.id { return true; }
                
                if let Some(cells) = map.get(&c.id) {
                    let diff = cells.difference(&claim_cells).collect::<Vec<&Point>>();
                    if diff.len() == cells.len() { return true; }
                }
                false
            }) { return Some(claim); }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ovewrlapping() {
        let input = include_str!("test_input_1.txt");
        let claims = parse_input(input);

        let fabric = Fabric::new(claims);

        assert_eq!(fabric.overlapping(), 4);
    }
    #[test]
    fn test_independent() {
        let input = include_str!("test_input_1.txt");
        let claims = parse_input(input);

        let fabric = Fabric::new(claims);

        assert_eq!(fabric.independent().unwrap().id, 3);
    }
}
