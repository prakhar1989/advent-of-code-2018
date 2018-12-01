use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day01.txt");
    let total = part1(input);
    println!("Total: {}", total);
    println!("already seen: {}", part2(input));

}

fn to_nums(input: &str) -> Vec<i32> {
    let xs: Vec<&str>  = input.split("\n")
        .map(|s: &str| s.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let mut entries = vec![];
    for x in &xs {
        let multiplier = if x.starts_with("-") { -1 } else { 1 };
        let rest = &x[1..];
        entries.push(multiplier * rest.parse::<i32>().unwrap());
    }

    entries
}

fn part1(input: &str) -> i32 {
    to_nums(&input).iter().sum()
}

fn part2(input: &str) -> i32 {
    let entries = to_nums(input);
    let mut last_seen = HashSet::new();
    let mut total = 0;

    loop {
        for x in &entries {
            total += x;
            if last_seen.contains(&total) {
                return total;
            } else {
                last_seen.insert(total);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01() {
        assert_eq!(part1("-4\n+5\n+10"), 11);
    }

    #[test]
    fn day01_part_two() {
        assert_eq!(part2("-6\n,+3\n,+8\n,+5\n,-6"), 5);
        assert_eq!(part2("+3\n,+3\n,+4\n,-2\n,-4"), 10);
    }
}
