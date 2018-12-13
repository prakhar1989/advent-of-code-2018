use regex::Regex;
use std::collections::HashSet;
#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input/day07.txt");
fn main() {
    let steps: Vec<Dep> = INPUT.trim().lines().map(Dep::from_str).collect();
    println!("Part 1: {}", part1(&steps));
}

type Step = char;

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
            step: captures[2].as_bytes()[0] as Step,
            requirement: captures[1].as_bytes()[0] as Step,
        }
    }
}

fn part1(deps: &Vec<Dep>) -> String {
    let steps = get_sorted_steps(&deps);
    let mut order = vec![];
    let mut seen = HashSet::new();

    loop {
        match unvisited_steps(&steps, &deps, &seen).first() {
            None => break,
            Some(next_step) => {
                order.push(*next_step);
                seen.insert(*next_step);
            },
        }
    };

    order.into_iter().collect()
}

fn get_sorted_steps(deps: &Vec<Dep>) -> Vec<Step> {
    let mut steps: Vec<_> = deps
        .iter()
        .flat_map(|dep| vec![dep.step, dep.requirement].into_iter())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    steps.sort();

    steps
}

fn unvisited_steps(steps: &Vec<Step>, deps: &Vec<Dep>, visited: &HashSet<Step>) -> Vec<Step> {
    steps
        .into_iter()
        .filter(|id| !visited.contains(id)) // not yet visited
        .filter(|id| {
            // all deps have been visited
            deps.iter()
                .filter(|d| d.step == **id)
                .all(|d| visited.contains(&d.requirement))
        })
        .map(|c| *c)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_to_dep() {
        let dep = Dep::from_str("Step C must be finished before step A can begin.");
        assert_eq!(dep.step, 'A');
        assert_eq!(dep.requirement, 'C');
    }

    #[test]
    fn day07_sorted_ids() {
        let deps = to_deps();
        assert_eq!(get_sorted_steps(&deps), vec!['A', 'B', 'C', 'D', 'E', 'F']);
    }

    #[test]
    fn day07_unvisited_steps() {
        let deps = to_deps();
        let steps = vec!['A', 'B', 'C', 'D', 'E', 'F'];
        let mut seen: HashSet<Step> = HashSet::new();
        assert_eq!(unvisited_steps(&steps, &deps, &seen), vec!['C']);

        seen.insert('C');
        assert_eq!( unvisited_steps(&steps, &deps, &seen), vec!['A', 'F']);
    }

    #[test]
    fn day07_part1() {
        let deps = to_deps();
        assert_eq!(part1(&deps), "CABDFE".to_string());
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
