use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    // println!("Answer #1: {}", );
}

fn get_root(input: &str) -> &str {
    let mut program_list = input.lines().into_iter().map(|x| Program::from_str(x));
    let mut graph = HashMap::new();

    for p in program_list {
        graph.insert(p.id, p);
    }

    for k in graph {
        for c in k.children {
            println!("{}", c);
        }
    }
    "JACK"
}

#[derive(Debug,PartialEq)]
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