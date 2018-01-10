use std::collections::HashMap;
use std::collections::HashSet;

type NodeID = &'static str;
type Graph = HashMap<NodeID, &'static str>;

fn main() {
    let input = include_str!("input.txt");

    let graph = parse_graph(input);

    println!("Answer #1: {}", get_group_by_root(input, "0", &graph).len());
}

fn get_group_by_root(input: &str, root_id: NodeID, graph: &Graph) -> HashSet<NodeID> {

    let mut to_visit = vec![root_id];
    let mut visited = HashSet::new();

    // Keep taking candidates from the queue intil empty
    while let Some(next) = to_visit.pop() {
        // Add node to visited
        visited.insert(next);

        // Get associated nodes
        let candidates: Vec<&'static str> = graph.get(next).unwrap().split(", ").collect();

        // Push unvisited candidates to queue
        for candidate in candidates {
            if visited.contains(candidate) { // do nothing
            } else {
                to_visit.push(candidate);
            }
        }
    }

    visited
}

fn parse_graph(input: &'static str) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        let split: Vec<&'static str> = line.split(" <-> ").collect();

        graph.insert(split[0], split[1]);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_group_by_root() {
        let input = include_str!("test_input.txt");
        let graph = parse_graph(input);
        assert_eq!(6, get_group_by_root(input, "0", &graph).len());
    }
}
