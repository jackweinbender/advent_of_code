use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let reqs = parse_input(input);

    println!("Answer #1: {}", answer_1(reqs.clone()));
    println!("Answer #2: {}", answer_2(reqs, 60, 5));
}

fn parse_input(input: &str) -> Vec<Requirement> {
    input
        .lines()
        .map(|l| {
            let words: Vec<&str> = l.split_whitespace().collect();
            Requirement {
                id: words[7].parse::<char>().unwrap(),
                req: words[1].parse::<char>().unwrap(),
            }
        })
        .collect::<Vec<Requirement>>()
}

fn answer_1(reqs: Vec<Requirement>) -> String {
    let mut tree: HashMap<Step, Dependencies> = HashMap::new();
    let mut candidates: BTreeSet<Step> = BTreeSet::new();
    let mut complete: Dependencies = Dependencies::new();

    let mut order = String::new();

    // Setup the graph
    for r in reqs {
        let step = tree.entry(r.id).or_insert(Dependencies::new());
        step.insert(r.req);

        candidates.insert(r.req);
        candidates.insert(r.id);
    }

    while let Some(current_step) = candidates.clone().iter().find(|x| match tree.get(x) {
        Some(v) => complete.is_superset(v),
        None => true,
    }) {
        complete.insert(*current_step);
        candidates.remove(current_step);
        order.push(*current_step);
    }
    order
}

fn answer_2(reqs: Vec<Requirement>, offset: Second, workers: usize) -> Second {
    // Setup
    let mut graph: HashMap<Step, Dependencies> = HashMap::new();
    let mut candidates: BTreeSet<Step> = BTreeSet::new();
    let mut complete: Dependencies = Dependencies::new();

    let mut scheduler: Vec<Task> = vec![];

    // Setup the graph
    for r in reqs {
        let step = graph.entry(r.id).or_insert(Dependencies::new());
        step.insert(r.req);

        candidates.insert(r.req);
        candidates.insert(r.id);
    }
    let mut duration: Second = 0;

    while duration == 0 || scheduler.len() > 0 {
        duration += 1;
        // Get tasks about to be completed
        let (comp, remainder): (Vec<Task>, Vec<Task>) =
            scheduler.iter().partition(|t| t.duration == 1);
        // Mark completed tasks as such
        for c in comp {
            complete.insert(c.id);
        }
        // Reassign unfinished tasks to scheduler
        scheduler = remainder.iter().filter_map(|t| t.dec()).collect();
        // Check for free threads
        let free_threads = workers - scheduler.len();

        // Break if no free threads
        if free_threads == 0 {
            continue;
        }

        // Check for candidates and take up to the number of free threads
        let candidate_steps = candidates
            .clone()
            .iter()
            .filter(|c| match graph.get(c) {
                Some(v) => complete.is_superset(v),
                None => true,
            })
            .map(|&c| c)
            .take(free_threads)
            .collect::<Vec<Step>>();

        // Break if no candidates
        if candidate_steps.len() == 0 {
            continue;
        }

        for cand in candidate_steps {
            let duration = step_duration(cand, offset);
            // Add candidates to scheduler
            scheduler.push(Task::new(cand, duration));
            // Remove candidates from candidates
            candidates.remove(&cand);
        }
    }
    duration - 1
}

fn step_duration(step: Step, offset: Second) -> Second {
    (step as Second - 64) + offset
}

type Step = char;
type Second = isize;
type Dependencies = HashSet<Step>;

#[derive(Clone, Copy, Debug)]
struct Task {
    id: Step,
    duration: Second,
}

impl Task {
    fn new(s: Step, d: Second) -> Task {
        Task { id: s, duration: d }
    }
    fn dec(&self) -> Option<Task> {
        if self.duration > 0 {
            let t = Task {
                id: self.id,
                duration: self.duration - 1,
            };
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Requirement {
    id: Step,
    req: Step,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let req = "Step B must be finished before step A can begin.";
        let expected = Requirement { id: 'A', req: 'B' };
        let result = parse_input(req);
        assert_eq!(result[0], expected);
    }
    #[test]
    fn test_duration_a() {
        let offset = 0;
        let ch = 'A';

        assert_eq!(step_duration(ch, offset), 1);
    }
    #[test]
    fn test_duration_b() {
        let offset = 60;
        let ch = 'A';

        assert_eq!(step_duration(ch, offset), 61);
    }
    #[test]
    fn test_answer_1() {
        let input = include_str!("test_input.txt");
        let reqs = parse_input(input);

        assert_eq!(answer_1(reqs), "CABDFE");
    }
    #[test]
    fn test_answer_1b() {
        let input = include_str!("input.txt");
        let reqs = parse_input(input);

        assert_eq!(answer_1(reqs), "ADEFKLBVJQWUXCNGORTMYSIHPZ");
    }
    #[test]
    fn test_answer_2() {
        let input = include_str!("test_input.txt");
        let reqs = parse_input(input);

        assert_eq!(answer_2(reqs, 0, 2), 15);
    }
    #[test]
    fn test_answer_2b() {
        let input = include_str!("input.txt");
        let reqs = parse_input(input);

        assert_eq!(answer_2(reqs, 60, 5), 1120);
    }
}
