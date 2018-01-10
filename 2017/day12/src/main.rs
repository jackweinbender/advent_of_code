#![feature(vec_remove_item)]

use std::collections::HashMap;
use std::collections::HashSet;

type NodeID = &'static str;
type Graph = HashMap<NodeID, &'static str>;
type Group = HashSet<NodeID>;

fn main() {
    let input = include_str!("input.txt");
    let graph = parse_graph(input);

    println!("Answer #1: {}", get_group_by_root("0", &graph).len());
    println!("Answer #2: {}", get_all_groups(&graph).len());
}

fn get_group_by_root(root_id: NodeID, graph: &Graph) -> Group {

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
            if visited.contains(candidate) {
                // do nothing
            } else {
                to_visit.push(candidate);
            }
        }
    }
    visited
}

fn get_all_groups(graph: &Graph) -> Vec<Group> {
    let mut hash_keys: Vec<&str> = graph.keys().map(|x| *x).collect();
    let mut groups: Vec<Group> = vec![];

    // Take the next Node
    while let Some(next) = hash_keys.pop() {
        // Get all the nodes in the group
        let group = get_group_by_root(next, graph);

        // Remove all nodes in the current group from the node list
        for node in &group {
            hash_keys.remove_item(&node);
        }
        groups.push(group);
    }
    groups
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
        assert_eq!(6, get_group_by_root("0", &graph).len());
    }
    #[test]
    fn test_get_all_groups() {
        let input = include_str!("test_input.txt");
        let graph = parse_graph(input);
        assert_eq!(2, get_all_groups(&graph).len());
    }
}
