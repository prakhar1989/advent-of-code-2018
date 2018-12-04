extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input/day03.txt");

type Grid = [[u32; 1000]; 1000];

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn from(s: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        let captures = RE.captures(s).unwrap();

        Claim {
            id: captures[1].parse().expect("no id"),
            x: captures[2].parse().expect("no x"),
            y: captures[3].parse().expect("no y"),
            width: captures[4].parse().expect("no width"),
            height: captures[5].parse().expect("no height"),
        }
    }
}

fn main() {
    let claims: Vec<Claim> = INPUT.lines().map(Claim::from).collect();

    println!("Part 1: {}", part1(&claims));
    println!("Part 2: {:?}", part2(&claims));
}

fn build_grid(claims: &Vec<Claim>) -> Grid {
    let mut grid: Grid = [[0; 1000]; 1000];

    for claim in claims {
        for i in claim.x..(claim.x + claim.width) {
            for j in claim.y..(claim.y + claim.height) {
                grid[i as usize][j as usize] += 1;
            }
        }
    }

    grid
}

fn part1(claims: &Vec<Claim>) -> u32 {
    let grid = build_grid(claims);

    grid.iter()
        .map(|row| row.iter().filter(|v| **v > 1).count())
        .fold(0, |acc, val| acc + (val as u32))
}

fn part2(claims: &Vec<Claim>) -> Option<u32> {
    let grid = build_grid(claims);

    for claim in claims {
        let mut nonoverlapping = true;
        for i in claim.x..(claim.x + claim.width) {
            for j in claim.y..(claim.y + claim.height) {
                if grid[i as usize][j as usize] > 1 {
                    nonoverlapping = false;
                    continue;
                }
            }
        }
        if nonoverlapping {
            return Some(claim.id);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_claim_parsing() {
        assert_eq!(
            Claim::from("#27 @ 355,118: 12x15"),
            Claim {
                id: 27,
                x: 355,
                y: 118,
                width: 12,
                height: 15,
            }
        );
        assert_eq!(
            Claim::from("#774 @ 814,799: 17x17"),
            Claim {
                id: 774,
                x: 814,
                y: 799,
                width: 17,
                height: 17,
            }
        );
    }

    #[test]
    fn day03_part1() {
        let claims = vec![
            Claim::from("#100 @ 1,3: 4x4"),
            Claim::from("#2200 @ 3,1: 4x4"),
            Claim::from("#3300 @ 5,5: 2x2"),
        ];

        //assert_eq!(part2(&claims), Some(3));
    }
}
