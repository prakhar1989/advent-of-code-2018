const INPUT: &str = include_str!("../input/day05.txt");

fn main() {
    let input = INPUT.trim();
    println!("Part 1: {}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut stack = Vec::new();
    for c in input.chars() {
        if stack.is_empty() {
            stack.push(c);
        } else {
            if let Some(v) = stack.pop() {
                if !(v != c && c.to_ascii_uppercase() == v.to_ascii_uppercase()) {
                    stack.push(v);
                    stack.push(c);
                }
            }
        }
    }

    stack.iter().collect::<String>().len()
}

fn part2(input: &str) -> Option<usize> {
    "abcd"
        .chars()
        .map(|c| {
            let reacted = input
                .chars()
                .filter(|x| *x != c && *x != c.to_ascii_uppercase())
                .collect::<String>();
            part1(&reacted)
        }).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1() {
        assert_eq!(part1("aA"), 0);
        assert_eq!(part1("abBA"), 0);
        assert_eq!(part1("abAB"), 4);
        assert_eq!(part1("aabAAB"), 6);
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn day05_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), Some(4));
    }
}
