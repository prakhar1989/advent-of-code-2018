extern crate regex;
use regex::Regex;
use std::error::Error;
use std::result;

#[macro_use]
extern crate lazy_static;

mod matrix;

const INPUT: &str = include_str!("../input/day03.txt");

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn from(s: &str) -> Result<Claim> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        let captures = match RE.captures(s) {
            Some(captures) => captures,
            None => return Err(Box::<Error>::from(format!("invalid format for claim"))),
        };

        Ok(Claim {
            id: captures[1].parse()?,
            x: captures[2].parse()?,
            y: captures[3].parse()?,
            width: captures[4].parse()?,
            height: captures[5].parse()?,
        })
    }
}

fn main() {
    let claims: Vec<Claim> = INPUT.lines()
        .map(Claim::from)
        .filter_map(Result::ok).collect();

    println!("Part 1: {}", part1(&claims));
    println!("Part 2: {:?}", part2(&claims));
}

fn build_grid(claims: &Vec<Claim>) -> matrix::Matrix<u32> {
    let mut grid = matrix::Matrix::new(1000, 1000, 0);

    for claim in claims {
        for i in claim.x..(claim.x + claim.width) {
            for j in claim.y..(claim.y + claim.height) {
                grid[(i as usize, j as usize)] += 1;
            }
        }
    }

    grid
}

fn part1(claims: &Vec<Claim>) -> u32 {
    let grid = build_grid(claims);

    grid.rows()
        .map(|row| row.into_iter().filter(|v| **v > 1).count())
        .fold(0, |acc, val| acc + (val as u32))
}

fn part2(claims: &Vec<Claim>) -> Option<u32> {
    let grid = build_grid(claims);

    for claim in claims {
        let mut nonoverlapping = true;
        for i in claim.x..(claim.x + claim.width) {
            for j in claim.y..(claim.y + claim.height) {
                if grid[(i as usize,j as usize)] > 1 {
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
            Claim::from("#27 @ 355,118: 12x15").unwrap(),
            Claim {
                id: 27,
                x: 355,
                y: 118,
                width: 12,
                height: 15,
            }
        );
        assert_eq!(
            Claim::from("#774 @ 814,799: 17x17").unwrap(),
            Claim {
                id: 774,
                x: 814,
                y: 799,
                width: 17,
                height: 17,
            }
        );

        assert_eq!(Claim::from("some random string").is_err(), true);
    }

    #[test]
    fn day03_part1() {
        let claims = vec![
            Claim::from("#1 @ 1,3: 4x4").unwrap(),
            Claim::from("#2 @ 3,1: 4x4").unwrap(),
            Claim::from("#3 @ 5,5: 2x2").unwrap(),
        ];

        assert_eq!(part1(&claims), 4);
    }

    #[test]
    fn day03_part2() {
        let claims = vec![
            Claim::from("#1 @ 1,3: 4x4").unwrap(),
            Claim::from("#2 @ 3,1: 4x4").unwrap(),
            Claim::from("#3 @ 5,5: 2x2").unwrap(),
        ];

        assert_eq!(part2(&claims), Some(3));
    }
}
