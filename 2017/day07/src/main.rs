use std::collections::HashMap;

type Graph = HashMap<&'static str, Program<'static>>;

fn main() {
    let input = include_str!("input.txt");

    println!("Answer #1: {}", get_root(input));
}

fn get_root(input: &'static str) -> &str {
    let mut program_list: Vec<Program> = input.lines().into_iter().map(|x| Program::from_str(x)).collect();
    let mut graph = Graph::new();
    
    for i in &program_list {
        graph.insert(i.id.clone(), i.clone());
    }

    let mut node = program_list.last().unwrap();

    while let Some(n) = get_parent(node.id, &graph) {
        node = &graph.get(n).clone().unwrap();
    }

    node.id
}

fn get_parent(id: &str, graph: &Graph) -> Option<&'static str> {
    for (k,v) in graph {
        match v.children {
            Some(ref chs) => { 
                if chs.iter().any(|x| *x == id ) {
                    return Some(k);
                }
            }
            _ => { continue }
        }
    }
    None
}

#[derive(Debug,PartialEq, Clone)]
struct Program<'a> {
    id: &'a str,
    children: Option<Vec<&'a str>>,
    weight: usize
}

impl<'a> Program<'a> {
    fn from_str(input: &str) -> Program {
        match input.find("->") {
            Some(index) => {
                let head_tail: Vec<&str> = input.split(" -> ").collect();
                let d: Vec<&str> = head_tail[0].split_whitespace().collect();

                Program {
                    id: d[0],
                    weight: d[1].replace('(', "").replace(')',"").parse::<usize>().unwrap(),
                    children: Some(head_tail[1].trim().split(", ").collect())
                }

            }
            None => {
                let d: Vec<&str> = input.split_whitespace().collect();
                Program {
                    id: d[0],
                    weight: d[1].replace('(', "").replace(')',"").parse::<usize>().unwrap(),
                    children: None
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prgm_from_str(){

        let input_1 = "fwft (72) -> ktlj, cntj, xhth";
        let pgm_1 = Program {
            id: "fwft",
            weight: 72,
            children: Some(vec!["ktlj", "cntj", "xhth"])
        };
        assert_eq!(pgm_1, Program::from_str(input_1));
        
        let input_2 = "qoyq (66)";
        let pgm_2 = Program {
            id: "qoyq",
            weight: 66,
            children: None
        };
        assert_eq!(pgm_2, Program::from_str(input_2));
    }
    #[test]
    fn test_get_root(){
        let input = include_str!("test_input.txt");
        assert_eq!(get_root(input), "tknk");
    }
}