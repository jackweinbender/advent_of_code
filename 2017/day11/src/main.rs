use std::ops::Add;
use std::str::FromStr;

type Distance = usize;
type Point = isize;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {:?}", get_distance(input));
    println!("Answer #2: {:?}", get_max_distance(input));
}

fn walk_path(input: &str, start_tile: Tile) -> Tile {
    let path = generate_path(input);
    let mut current_tile = start_tile + Tile(0, 0, 0);

    for dir in path {
        current_tile = next_tile(current_tile, dir);
    }

    current_tile
}

fn generate_path(input: &str) -> Vec<Compass> {
    input
        .split(",")
        .map(|x| x.parse::<Compass>().unwrap())
        .collect()
}

fn get_distance(input: &str) -> Distance {
    let end_tile = walk_path(input, Tile(0, 0, 0));
    distance(Tile(0, 0, 0), end_tile)
}

fn get_max_distance(input: &str) -> Distance {
    let path = generate_path(input);
    let mut current_tile = Tile(0, 0, 0);
    let mut max_distance: Distance = 0;

    for dir in path {
        current_tile = next_tile(current_tile.clone(), dir);

        let current_distance = distance(Tile(0, 0, 0), current_tile.clone());
        if current_distance > max_distance {
            max_distance = current_distance
        }
    }

    max_distance
}

fn distance(start_tile: Tile, end_tile: Tile) -> Distance {
    let coordinates = vec![
        (start_tile.0 - end_tile.0).abs(),
        (start_tile.1 - end_tile.1).abs(),
        (start_tile.2 - end_tile.2).abs(),
    ];

    *coordinates.iter().max().unwrap() as Distance
}

#[derive(Debug)]
enum Compass {
    North,
    South,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl FromStr for Compass {
    type Err = ();

    fn from_str(s: &str) -> Result<Compass, ()> {
        match s {
            "n" => Ok(Compass::North),
            "s" => Ok(Compass::South),
            "ne" => Ok(Compass::NorthEast),
            "se" => Ok(Compass::SouthEast),
            "nw" => Ok(Compass::NorthWest),
            "sw" => Ok(Compass::SouthWest),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile(Point, Point, Point);

impl Add for Tile {
    type Output = Tile;

    fn add(self, other: Tile) -> Tile {
        Tile(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn next_tile(tile: Tile, compass: Compass) -> Tile {
    match compass {
        Compass::North => tile + Tile(0, 1, -1), // x-axis
        Compass::South => tile + Tile(0, -1, 1),
        Compass::SouthWest => tile + Tile(-1, 0, 1), // y-axis
        Compass::NorthEast => tile + Tile(1, 0, -1), 
        Compass::NorthWest => tile + Tile(-1, 1, 0),// z-axis
        Compass::SouthEast => tile + Tile(1, -1, 0),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_walk_distance_1() {
        let input = "ne,ne,ne";
        let end_tile = walk_path(input, Tile(0, 0, 0));
        assert_eq!(3, distance(Tile(0, 0, 0), end_tile));
    }
    #[test]
    fn test_walk_distance_2() {
        let input = "ne,ne,sw,sw";
        let end_tile = walk_path(input, Tile(0, 0, 0));
        assert_eq!(0, distance(Tile(0, 0, 0), end_tile));
    }
    #[test]
    fn test_walk_distance_3() {
        let input = "ne,ne,s,s";
        let end_tile = walk_path(input, Tile(0, 0, 0));
        assert_eq!(2, distance(Tile(0, 0, 0), end_tile));
    }
    #[test]
    fn test_walk_distance_4() {
        let input = "se,sw,se,sw,sw";
        let end_tile = walk_path(input, Tile(0, 0, 0));
        assert_eq!(3, distance(Tile(0, 0, 0), end_tile));
    }
}
