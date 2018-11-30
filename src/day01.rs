// dummy task: add all numbers in the file
fn main() {
    let input = include_str!("../input/day01.txt");
    let total = parse_and_add(input);
    println!("From day01 {}", total);
}

fn parse_and_add(input: &str) -> u8 {
    let xs: Vec<&str>  = input.split("\n")
        .map(|s: &str| s.trim())
        .collect();

    xs.iter().filter(|x| !x.is_empty())
        .map(|x| x.parse::<u8>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01() {
        assert_eq!(parse_and_add("4\n5"), 9);
        assert_eq!(parse_and_add("4\n         5"), 9);
        assert_eq!(parse_and_add("\n \n 5\n 6\n5"), 16);
    }
}
