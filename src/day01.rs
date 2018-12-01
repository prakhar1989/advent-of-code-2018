use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day01.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum()
}

fn part2(input: &str) -> i32 {
    let entries: Vec<_> = input.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut last_seen = HashSet::new();
    let mut total = 0;

    for x in entries.iter().cycle() {
        total += x;
        if last_seen.contains(&total) {
            return total;
        } else {
            last_seen.insert(total);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01() {
        assert_eq!(part1("-4\n+5\n+10"), 11);
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
    }
}
