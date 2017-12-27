
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
    let program_list: Vec<Program> = input
        .lines()
        .into_iter()
        .map(|x| Program::from_str(x))
        .collect();
    let mut graph = Graph::new();

    for i in &program_list {
        graph.insert(i.pid.clone(), i.clone());
    }

    let mut node = program_list.last().unwrap();

    while let Some(n) = node.get_parent(&graph) {
        node = &graph.get(n).clone().unwrap();
    }

    node.pid
}

fn get_unbalanced_node_corrected(input: &'static str) -> Weight {
    let program_list: Vec<Program> = input
        .lines()
        .into_iter()
        .map(|x| Program::from_str(x))
        .collect();
    let mut graph = Graph::new();

    for i in &program_list {
        graph.insert(i.pid.clone(), i.clone());
    }
    
    let mut pid = get_root(input);
    let mut nd = ("",0,0);
    while let Some(n) = next_unbalanced(pid, &graph) {
        nd = n.clone();
        pid = n.0;
    }
    let change: i32 = (nd.2 as i32 - nd.1 as i32) + graph.get(nd.0).unwrap().weight as i32;
    return change as Weight
}
fn next_unbalanced(pid: PID, graph: &Graph) -> Option<(PID, Weight, Weight)> {
    let node = graph.get(pid).unwrap();
    let children = node.get_children(&graph);
    
    let weights: Vec<(PID, Weight)> = children.unwrap().into_iter().map(|x| (x.pid, x.get_weight(&graph)) ).collect();

    let mut acc = HashMap::new();

    for next in weights.clone() { 
        if acc.clone().contains_key(&next.1) {
            let mut x = acc.get_mut(&next.1).unwrap();
            *x += 1; 
        } else {
            acc.insert(next.1, 1);
        }
    }
    if acc.len() == 1 { return None; }
    let next_weight  = acc.iter().clone().min_by(|x, y| x.1.cmp(y.1) ).unwrap();
    let normal_weight = *acc.iter().clone().max_by(|x, y| x.1.cmp(y.1) ).unwrap().0;
    let next_node: Vec<(&str, usize)> = weights.iter().filter(|x| x.1 == *next_weight.0).map(|x| *x ).collect();
    Some((next_node[0].0, next_node[0].1, normal_weight ))
}

#[derive(Debug, PartialEq, Clone)]
struct Program {
    pid: PID,
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
                    if chs.iter().any(|x| *x == self.pid) {
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
            Some(_) => {
                let head_tail: Vec<&str> = input.split(" -> ").collect();
                let d: Vec<&str> = head_tail[0].split_whitespace().collect();

                Program {
                    pid: d[0],
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
                    pid: d[0],
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
            pid: "fwft",
            weight: 72,
            children: Some(vec!["ktlj", "cntj", "xhth"]),
        };
        assert_eq!(pgm_1, Program::from_str(input_1));

        let input_2 = "qoyq (66)";
        let pgm_2 = Program {
            pid: "qoyq",
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
