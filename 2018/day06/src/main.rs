use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let hot_spots = parse_input(input);

    println!("Answer #1: {}", answer_1(hot_spots.clone()));
    println!("Answer #2: {}", answer_2(&hot_spots, 10000));
}
fn answer_1(mut hot_spots: Vec<Point>) -> i32 {
    let mut map: HashMap<Point, Vec<Point>> = HashMap::new();
    let maxes = hot_spots.clone();
    let some_x_max = maxes.iter().max_by(|&x, &y| {
        let &Point(a, _) = x;
        let &Point(b, _) = y;
        a.cmp(&b)
    });
    let some_y_max = maxes.iter().max_by(|&x, &y| {
        let &Point(_, a) = x;
        let &Point(_, b) = y;
        a.cmp(&b)
    });

    let x_max = some_x_max.unwrap().0;
    let y_max = some_y_max.unwrap().1;

    for x in 0..x_max {
        for y in 0..y_max {
            let point = Point(x, y);

            hot_spots.sort_by(|x, y| {
                let x_dist = x.distance(&point);
                let y_dist = y.distance(&point);

                x_dist.cmp(&y_dist)
            });

            let min_dist = hot_spots[0].distance(&point);
            let mut nearest = hot_spots
                .iter()
                .take_while(|x| x.distance(&point) == min_dist)
                .collect::<Vec<&Point>>();
            if nearest.len() == 1 {
                let mut hotspot = map.entry(nearest[0].clone()).or_insert(vec![]);
                hotspot.push(point);
            }
        }
    }

    let max = map
        .into_iter()
        .filter(|(k, _v)| k.bounded_by(&hot_spots.clone()))
        .max_by(|x, y| {
            let x_len = x.1.len();
            let y_len = y.1.len();
            x_len.cmp(&y_len)
        });
    max.unwrap().1.len() as i32
}

fn answer_2(hotspots: &Vec<Point>, radius: i32) -> i32 {
    let mut safezone_size = 0;

    for x in 0..radius {
        for y in 0..radius {
            let p = Point(x, y);
            if let Some(_) = hotspots
                .iter()
                .try_fold(0, |acc, next| match acc + p.distance(next) {
                    new if new >= radius => None,
                    new => Some(new),
                })
            {
                safezone_size += 1;
            }
        }
    }

    safezone_size
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|l| l.parse::<Point>().ok())
        .collect()
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        let x_offset = self.0 - other.0;
        let y_offset = self.1 - other.1;

        x_offset.abs() + y_offset.abs()
    }
    fn bounded_by(&self, points: &Vec<Point>) -> bool {
        let north = points.iter().any(|p| {
            if p == self {
                return false;
            }
            let run = (self.1 - p.1).abs();
            let rise = self.0 - p.0;
            rise >= run
        });
        let south = points.iter().any(|p| {
            if p == self {
                return false;
            }
            let run = (self.1 - p.1).abs();
            let rise = p.0 - self.0;
            rise >= run
        });
        let east = points.iter().any(|p| {
            if p == self {
                return false;
            }
            let run = (self.0 - p.0).abs();
            let rise = p.1 - self.1;
            rise >= run
        });
        let west = points.iter().any(|p| {
            if p == self {
                return false;
            }
            let run = (self.0 - p.0).abs();
            let rise = self.1 - p.1;
            rise >= run
        });

        north && south && east && west
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x_fromstr = coords[0].trim().parse::<i32>().unwrap();
        let y_fromstr = coords[1].trim().parse::<i32>().unwrap();

        let point = Point(x_fromstr, y_fromstr);

        Ok(point)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer_1() {
        let input = include_str!("test_input.txt");
        let points = parse_input(input);

        assert_eq!(answer_1(points), 17);
    }
    #[test]
    fn test_answer_2() {
        let input = include_str!("test_input.txt");
        let points = parse_input(input);

        assert_eq!(answer_2(&points, 32), 16);
    }

}
