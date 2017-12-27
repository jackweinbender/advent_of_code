
use std::collections::HashMap;

type PID = &'static str;
type Weight = usize;
type Graph = HashMap<PID, Program>;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", get_root(input));
    println!("Answer #2: {}", get_unbalanced_node_corrected(input));
}

fn get_root(input: &'static str) -> PID {
    let mut program_list: Vec<Program> = input
        .lines()
        .into_iter()
        .map(|x| Program::from_str(x))
        .collect();
    let mut graph = Graph::new();

    for i in &program_list {
        graph.insert(i.id.clone(), i.clone());
    }

    let mut node = program_list.last().unwrap();

    while let Some(n) = node.get_parent(&graph) {
        node = &graph.get(n).clone().unwrap();
    }

    node.id
}

fn get_unbalanced_node_corrected(input: &'static str) -> Weight {
    let mut program_list: Vec<Program> = input
        .lines()
        .into_iter()
        .map(|x| Program::from_str(x))
        .collect();
    let mut graph = Graph::new();

    for i in &program_list {
        graph.insert(i.id.clone(), i.clone());
    }
    
    let mut pid = get_root(input);
    while let Some(n) = next_unbalanced(pid, &graph) {
        println!("{:?}", pid);
        pid = n;
    }
0
}
fn next_unbalanced(pid: PID, graph: &Graph) -> Option<PID> {
    let node = graph.get(pid).unwrap();
    let children = node.get_children(&graph);
    
    let weights: Vec<usize> = children.unwrap().into_iter().map(|x| x.get_weight(&graph) ).collect();

    let num_weights = weights.len().clone();
    let cmp = weights[0].clone();

    if cmp == weights.into_iter().fold(0, |x, y| x + y ) / num_weights {
        return None;
    }

    // Figure out how to return the unbalanced Node here
    None // Remove when you get this
}

#[derive(Debug, PartialEq, Clone)]
struct Program {
    id: PID,
    children: Option<Vec<PID>>,
    weight: usize,
}

impl Program {
    fn get_weight(&self, graph: &Graph) -> Weight {
        let children = self.get_children(graph);

        match children {
            Some(chs) =>  {
                let ch_weight = chs.into_iter().fold(0, |acc, c| acc + c.get_weight(graph));
                return ch_weight + self.weight;
            },
            None => return self.weight,
        }
    }
    fn get_children(&self, graph: &Graph) -> Option<Vec<Program>> {
        match self.children {
            Some(ref ch) => {
                let mut chs = vec![];

                for c in ch {
                    let child = graph.get(c).unwrap();
                    chs.push(child.clone());
                }
                Some(chs)
            }
            None => None,
        }
    }
    fn get_parent(&self, graph: &Graph) -> Option<PID> {
        for (k, v) in graph {
            match v.children {
                Some(ref chs) => {
                    if chs.iter().any(|x| *x == self.id) {
                        return Some(k);
                    }
                }
                _ => continue,
            }
        }
        None
    }
    fn from_str(input: &'static str) -> Program {
        match input.find("->") {
            Some(index) => {
                let head_tail: Vec<&str> = input.split(" -> ").collect();
                let d: Vec<&str> = head_tail[0].split_whitespace().collect();

                Program {
                    id: d[0],
                    weight: d[1]
                        .replace('(', "")
                        .replace(')', "")
                        .parse::<usize>()
                        .unwrap(),
                    children: Some(head_tail[1].trim().split(", ").collect()),
                }

            }
            None => {
                let d: Vec<&'static str> = input.split_whitespace().collect();
                Program {
                    id: d[0],
                    weight: d[1]
                        .replace('(', "")
                        .replace(')', "")
                        .parse::<usize>()
                        .unwrap(),
                    children: None,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prgm_from_str() {

        let input_1 = "fwft (72) -> ktlj, cntj, xhth";
        let pgm_1 = Program {
            id: "fwft",
            weight: 72,
            children: Some(vec!["ktlj", "cntj", "xhth"]),
        };
        assert_eq!(pgm_1, Program::from_str(input_1));

        let input_2 = "qoyq (66)";
        let pgm_2 = Program {
            id: "qoyq",
            weight: 66,
            children: None,
        };
        assert_eq!(pgm_2, Program::from_str(input_2));
    }
    #[test]
    fn test_get_root() {
        let input = include_str!("test_input.txt");
        assert_eq!(get_root(input), "tknk");
    }

    #[test]
    fn test_get_unbalanced_node_corrected() {
        let input = include_str!("test_input.txt");
        assert_eq!(get_unbalanced_node_corrected(input), 60);
    }
}
