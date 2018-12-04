
extern crate regex;
use regex::Regex;

type ID = usize;

fn main() {
    let input = include_str!("input.txt");
    let claims = parse_input(input);

    let fabric = Fabric::new(claims);

    println!("Answer #1: {}", fabric.overlapping);
}

fn parse_input(input: &str) -> Vec<Claim> {

    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    input.lines().map(|l| {
        let caps = re.captures(l).unwrap();

        let id = caps.get(1).map_or(0, |m| m.as_str().parse::<ID>().ok().unwrap() );

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

struct Claim {
    id: ID,
    offset: Offset,
    size: Rect
}

struct Fabric {
    claims: Vec<Claim>
}

impl Fabric {
    fn new(claims: Vec<Claim>) -> Fabric {
        Fabric{ claims: claims }
    }
    fn overlapping(&self) -> usize {


        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ovewrlapping() {
        // #1 @ 1,3: 4x4
        // #2 @ 3,1: 4x4
        // #3 @ 5,5: 2x2
        let c1 = Claim { id: 1, offset: Offset{ x:1 , y:3 }, size: Rect{ x:4 , y:4 } };
        let c2 = Claim { id: 2, offset: Offset{ x:3 , y:1 }, size: Rect{ x:4 , y:4 } };
        let c3 = Claim { id: 3, offset: Offset{ x:5 , y:5 }, size: Rect{ x:2 , y:2 } };

        let claims = vec![c1, c2, c3];

        let fabric = Fabric::new(claims);

        assert_eq!(fabric.overlapping, 4);
    }
}
