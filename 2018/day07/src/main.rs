
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input.txt");
    let reqs = parse_input(input);

    println!("Answer #1: {}", answer_1(reqs));
    // println!("Answer #2: {}", answer_2());
}

fn parse_input(input: &str) -> Vec<Requirement> {
    input.lines().map(|l| {
        let words: Vec<&str> = l.split_whitespace().collect();
        Requirement {
            id:  words[7].parse::<char>().unwrap(),
            req: words[1].parse::<char>().unwrap()
        }
    }).collect::<Vec<Requirement>>()
}

fn answer_1(reqs: Vec<Requirement>) -> String {
    let mut tree: HashMap<Step, Dependencies> = HashMap::new();
    let mut queue: BTreeSet<Step> = BTreeSet::new();
    let mut complete: Dependencies = Dependencies::new();
    
    let mut order = String::new();
    
    // Setup the graph
    for r in reqs {
        let step = tree.entry(r.id).or_insert(Dependencies::new());
        step.insert(r.req);
        
        queue.insert(r.req);
        queue.insert(r.id);
    }

    while let Some(current_step) = queue.clone().iter()
        .find(|x| {
            match tree.get(x) {
                Some(v) => complete.is_superset(v),
                None => true
            }
        }) { 
            complete.insert(*current_step);
            queue.remove(current_step);
            order.push(*current_step);
         }
    order
}

type Step = char;
type Dependencies = HashSet<Step>;

#[derive(Debug,PartialEq,Eq)]
struct Requirement {
    id: Step,
    req: Step
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let req = "Step B must be finished before step A can begin.";
        let expected = Requirement {
            id: 'A',
            req: 'B'
        };
        let result = parse_input(req);
        assert_eq!(result[0], expected );
    }
    #[test]
    fn test_answer_1() {
        let input = include_str!("test_input.txt");
        let reqs = parse_input(input);

        assert_eq!(answer_1(reqs), "CABDFE");
    }

}