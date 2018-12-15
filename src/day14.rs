fn main() {
    println!("Part 1: {}", part1(290431));
    println!("Part 2: {}", part2(&[2, 9, 0, 4, 3, 1]));
}

fn part1(max_r: usize) -> String {
    let mut recipies = vec![3, 7];
    let mut i = 0; // first elf
    let mut j = 1; // second elf

    while recipies.len() < max_r + 10 {
        recipies.extend(digits(recipies[i] + recipies[j]));
        i = (i + 1 + recipies[i]) % recipies.len();
        j = (j + 1 + recipies[j]) % recipies.len();
    }

    (&recipies[max_r..(max_r + 10)])
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

fn part2(target: &[usize]) -> usize {
    let mut recipies: Vec<usize> = vec![3, 7];
    let mut i = 0; // first elf
    let mut j = 1; // second elf

    loop {
        recipies.extend(digits(recipies[i] + recipies[j]));
        i = (i + 1 + recipies[i]) % recipies.len();
        j = (j + 1 + recipies[j]) % recipies.len();

        for offset in 0..=1 {
            if recipies.len() - offset >= target.len() {
                if &recipies[recipies.len() - target.len() - offset..recipies.len() - offset]
                    == target
                {
                    return recipies.len() - target.len() - offset;
                }
            }
        }
    }
}

fn digits(x: usize) -> Vec<usize> {
    if x == 0 {
        return vec![0];
    }
    let mut number = x;
    let mut digits = vec![];
    while number > 0 {
        digits.push(number % 10);
        number = number / 10;
    }

    digits.reverse();

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1() {
        assert_eq!(part1(9), "5158916779".to_string());
        assert_eq!(part1(5), "0124515891".to_string());
        assert_eq!(part1(18), "9251071085".to_string());
        assert_eq!(part1(2018), "5941429882".to_string());
        assert_eq!(part1(290431), "1776718175".to_string());
    }

    #[test]
    fn day14_part2() {
        assert_eq!(part2(&vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(part2(&vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(part2(&vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(part2(&vec![5, 9, 4, 1, 4]), 2018);
    }

}
