use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input/day07.txt");
fn main() {
    let steps: Vec<Dep> = INPUT.trim().lines().map(Dep::from_str).collect();
}

type Step = char;
type Graph = HashMap<Step, Vec<Step>>;

struct Dep {
    step: Step,
    requirement: Step,
}

impl Dep {
    fn from_str(s: &str) -> Dep {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin").unwrap();
        }

        let captures = RE.captures(s).unwrap();
        Dep {
            step: captures[1].as_bytes()[0] as Step,
            requirement: captures[2].as_bytes()[0] as Step,
        }
    }
}

fn build_graph(deps: &Vec<Dep>) -> Graph {
    let mut graph = HashMap::new();

    for dep in deps {
        let edges = graph.entry(dep.step).or_insert(Vec::new());
        edges.push(dep.requirement);

        graph.entry(dep.requirement).or_insert(Vec::new());
    }

    graph
}

/// returns the step with the lowest indegree. In case of a clash,
/// orders by alphabetical ordering.
fn lowest_indegree(graph: &Graph) -> Step {
    //let n = graph.values().flat_map(|node| node.iter()).collect();

    'A'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_some_test() {
        let dep = Dep::from_str("Step C must be finished before step A can begin.");
        assert_eq!(dep.step, 'C');
        assert_eq!(dep.requirement, 'A');
    }

    #[test]
    fn day07_build_graph() {
        let graph = build_graph(&to_deps());

        assert_eq!(graph[&'A'], vec!['B', 'D']);
        assert_eq!(graph[&'B'], vec!['E']);
        assert_eq!(graph[&'C'], vec!['A', 'F']);
        assert_eq!(graph[&'D'], vec!['E']);
        assert_eq!(graph[&'E'].is_empty(), true);
        assert_eq!(graph[&'F'], vec!['E']);
    }

    fn to_deps() -> Vec<Dep> {
        let s = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        s.lines().map(Dep::from_str).collect()
    }
}
