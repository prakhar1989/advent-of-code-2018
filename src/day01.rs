use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn to_nums(input: &str) -> Vec<i32> {
    let terms: Vec<&str>  = input.split("\n")
        .map(|s: &str| s.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let mut nums = vec![];
    for x in &terms {
        let multiplier = if x.starts_with("-") { -1 } else { 1 };
        match &x[1..].parse::<i32>() {
            Ok(v) => {
                nums.push(multiplier * v);
            }
            Err(_e) => continue
        }
    }

    nums
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
        assert_eq!(part2("-6\n,+3\n,+8\n,+5\n,-6"), 5);
        assert_eq!(part2("+3\n,+3\n,+4\n,-2\n,-4"), 10);
    }
}
