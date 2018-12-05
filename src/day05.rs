const INPUT: &str = include_str!("../input/day05.txt");

fn main() {
    let input = INPUT.trim();
    println!("Part 1: {}", part1(input).len());
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: &str) -> String {
    let mut stack = Vec::new();
    for c in input.chars() {
        if stack.is_empty() {
            stack.push(c);
        } else {
            if let Some(v) = stack.pop() {
                if !can_react(v, c) {
                    stack.push(v);
                    stack.push(c);
                }
            }
        }
    }

    stack.iter().collect()
}

fn can_react(c1: char, c2: char) -> bool {
    let diff: i32 = (c1 as i32) - (c2 as i32); // since ascii
    diff.abs() == 32
}

fn part2(input: &str) -> Option<usize> {
    "abcd"
        .chars()
        .map(|c| {
            let reacted = input
                .chars()
                .filter(|x| *x != c && *x != c.to_ascii_uppercase())
                .collect::<String>();
            part1(&reacted).len()
        }).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_can_react() {
        assert_eq!(can_react('a', 'A'), true);
        assert_eq!(can_react('A', 'a'), true);
        assert_eq!(can_react('a', 'b'), false);
    }

    #[test]
    fn day05_part1() {
        assert_eq!(part1("aA"), "");
        assert_eq!(part1("abBA"), "");
        assert_eq!(part1("abAB"), "abAB");
        assert_eq!(part1("aabAAB"), "aabAAB");
        assert_eq!(part1("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn day05_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), Some(4));
    }
}
