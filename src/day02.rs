use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day02.txt");

fn main() {
    let lines: Vec<_> = INPUT.lines().collect();
    println!("part1: {}", part1(&lines));
    println!("part2: {:?}", part2(&lines));
}

fn part1(strs: &Vec<&str>) -> usize {
    let counters: Vec<HashMap<_, _>> = strs.iter().map(|x| to_counter(x)).collect();
    let twos = counters.iter().filter(|s| has_exactly(s, 2)).count();
    let threes = counters.iter().filter(|s| has_exactly(s, 3)).count();

    twos * threes
}

fn part2(strs: &Vec<&str>) -> Option<String> {
    for (i, s1) in strs.iter().enumerate() {
        for s2 in strs[(i + 1)..].iter() {
            if differing_chars(s1, s2) {
                return Some(matching_chars(s1, s2));
            }
        }
    }
    None
}

fn differing_chars(s1: &str, s2: &str) -> bool {
    s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count() <= 1
}

fn matching_chars(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars()).filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

fn to_counter(s: &str) -> HashMap<String, i32> {
    let mut counts: HashMap<String, i32> = HashMap::new();
    for a in s.chars() {
        let c = counts.entry(a.to_string()).or_insert(0);
        *c += 1;
    }
    counts
}

fn has_exactly(counts: &HashMap<String, i32>, n: i32) -> bool {
    counts.values().any(|c| *c == n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1() {
        assert_eq!(has_exactly(&to_counter("bababc"), 2), true);
        assert_eq!(has_exactly(&to_counter("bababc"), 3), true);
        assert_eq!(has_exactly(&to_counter("bababc"), 4), false);
        assert_eq!(has_exactly(&to_counter("ababab"), 3), true);

        let inputs = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        assert_eq!(part1(&inputs), 12);
    }

    #[test]
    fn day02_part2() {
        let inputs = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
        assert_eq!(differing_chars("abcde", "fghij"), false);
        assert_eq!(differing_chars("fguij", "fghij"), true);
        assert_eq!(differing_chars("axcye", "fghij"), false);
        assert_eq!(matching_chars("fguij", "fghij"), "fgij".to_string());
        assert_eq!(part2(&inputs), Some("fgij".to_string()));
    }
}
