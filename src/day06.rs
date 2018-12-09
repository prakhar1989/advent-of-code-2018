use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input/day06.txt");

fn main() {
    let points = to_points(INPUT.trim());
    println!("Part1: {}", part1(&points));
}

#[derive(Debug, PartialEq)]
struct Point(usize, usize);

impl Point {
    /// parse point from a string
    fn from_str(s: &str) -> Point {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+), (\d+)$").unwrap();
        }

        let captures = RE.captures(s).unwrap();
        Point(captures[1].parse().unwrap(), captures[2].parse().unwrap())
    }

    /// distance from another point
    fn dist(&self, p2: &Point) -> i32 {
        (self.0 as i32 - p2.0 as i32).abs() + (self.1 as i32 - p2.1 as i32).abs()
    }

    /// returns the index of the point that closest to the current point
    fn closest_to(&self, points: &Vec<Point>) -> Option<usize> {
        let distances: Vec<_> = points.iter().map(|p| self.dist(p)).collect();
        let (closest_index, min_dist) = distances
            .iter()
            .enumerate()
            .min_by_key(|(_, d)| *d)
            .unwrap();

        // if a point is closest to more than one point, return None
        if distances.iter().filter(|y| *y == min_dist).count() > 1 {
            None
        } else {
            Some(closest_index)
        }
    }
}

fn grid_size(points: &Vec<Point>) -> (usize, usize, usize, usize) {
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();

    (min_x, max_x, min_y, max_y)
}

fn to_points(s: &str) -> Vec<Point> {
    s.lines().map(Point::from_str).collect()
}

fn part1(points: &Vec<Point>) -> usize {
    let (min_x, max_x, min_y, max_y) = grid_size(points);

    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in min_x..max_x {
        for j in min_y..max_y {
            let point = Point(i, j);
            if let Some(closest_index) = point.closest_to(&points) {
                let entry = map.entry(closest_index).or_insert(0);
                *entry += 1;
            }
        }
    }

    // remove values for boundary points on x axis
    for x in min_x..max_x {
        for y in &[min_y, max_y] {
            if let Some(closest_index) = Point(x, *y).closest_to(&points) {
                map.remove(&closest_index);
            }
        }
    }

    // remove values for boundary points on y axis
    for y in min_y..max_y {
        for x in &[min_x, max_x] {
            if let Some(closest_index) = Point(*x, y).closest_to(&points) {
                map.remove(&closest_index);
            }
        }
    }

    *map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_parse_point() {
        let p = Point::from_str("20, 44");
        assert_eq!(p, Point(20, 44));
    }

    #[test]
    fn day06_grid_size() {
        let points = get_points();
        assert_eq!(grid_size(&points), (1, 8, 1, 9));
    }

    #[test]
    fn day06_dists() {
        let point = Point(6, 7);
        let p2 = Point(20, 3);
        assert_eq!(point.dist(&p2), 18);
    }

    #[test]
    fn day06_closest_to() {
        let points = get_points();
        assert_eq!(Point(7, 7).closest_to(&points), Some(5));
        assert_eq!(Point(0, 0).closest_to(&points), Some(0));
        assert_eq!(Point(0, 8).closest_to(&points), Some(1));
    }

    #[test]
    fn day06_part1() {
        let points = get_points();
        let max_area = part1(&points);
        assert_eq!(max_area, 17);

        let points = to_points(INPUT.trim());
        let max_area = part1(&points);
        assert_eq!(max_area, 3223);
    }

    fn get_points() -> Vec<Point> {
        let test_data = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

        to_points(test_data)
    }
}
