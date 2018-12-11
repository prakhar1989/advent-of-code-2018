use regex::Regex;
#[macro_use]
extern crate lazy_static;

mod matrix;

const INPUT: &str = include_str!("../input/day10.txt");

fn main() {
    let points: Vec<Point> = INPUT.trim().lines().map(Point::from_str).collect();

    let mut t = 0;
    let mut area = area_of_grid(&points);

    loop {
        t += 1;

        if t % 100 == 0 {
            println!("Iteration count: {}", t);
        }

        let new_points = tick_points(&points, t);
        let new_area = area_of_grid(&new_points);
        if new_area < area {
            // points are coming closer, continue
            area = new_area
        } else {
            // already converged, but moved one extra step
            // so decrement t
            t -= 1;
            println!("{}\n", draw_grid(&tick_points(&points, t)));
            println!("Time it took: {}", t);
            break;
        }
    }
}

fn tick_points(points: &Vec<Point>, time: i32) -> Vec<Point> {
    points
        .iter()
        .map(|point| Point {
            position: Vector(
                point.position.0 + point.velocity.0 * time,
                point.position.1 + point.velocity.1 * time,
            ),
            velocity: point.velocity,
        })
        .collect()
}

fn area_of_grid(points: &Vec<Point>) -> i64 {
    let (Vector(xmin, xmax), Vector(ymin, ymax)) = grid_size(&points);

    (ymax - ymin) as i64 * (xmax - xmin) as i64
}

fn draw_grid(points: &Vec<Point>) -> String {
    let (Vector(xmin, xmax), Vector(ymin, ymax)) = grid_size(&points);

    let mut grid = matrix::Matrix::new((ymax - ymin + 1) as usize, (xmax - xmin + 1) as usize, '.');

    for point in points {
        let pos = &point.position;
        grid[((pos.1 - ymin) as usize, (pos.0 - xmin) as usize)] = '#';
    }

    let result = grid
        .rows()
        .map(|row| {
            let r = row.iter().map(|c| c.to_string()).collect::<String>();
            format!("{}\n", r)
        })
        .collect::<String>();

    result.trim().to_string()
}

fn grid_size(points: &Vec<Point>) -> (Vector, Vector) {
    let mut xs: Vec<_> = points.iter().map(|p| p.position.0).collect();
    let mut ys: Vec<_> = points.iter().map(|p| p.position.1).collect();

    xs.sort();
    ys.sort();

    (
        Vector(*xs.first().unwrap(), *xs.last().unwrap()),
        Vector(*ys.first().unwrap(), *ys.last().unwrap()),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector(i32, i32);

#[derive(Debug)]
struct Point {
    position: Vector,
    velocity: Vector,
}

impl Point {
    fn from_str(s: &str) -> Point {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"position=<\s*(-?\d+),\s+(-?\d+)>\s+velocity=<\s*(-?\d+),\s+(-?\d+)>")
                    .unwrap();
        }
        let captures = RE.captures(s).unwrap();

        let position = Vector(
            captures[1].parse::<i32>().unwrap(),
            captures[2].parse::<i32>().unwrap(),
        );

        let velocity = Vector(
            captures[3].parse::<i32>().unwrap(),
            captures[4].parse::<i32>().unwrap(),
        );

        Point { position, velocity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_parse_point() {
        let point = Point::from_str("position=< 20416, -10005> velocity=<-2,  1>");

        assert_eq!(point.position, Vector(20416, -10005));
        assert_eq!(point.velocity, Vector(-2, 1));
    }

    #[test]
    fn day10_test_tick() {
        let point = Point::from_str("position=< 20, -10> velocity=<-2,  1>");

        assert_eq!(point.tick().position, Vector(18, -9));
    }

    #[test]
    fn day10_test_grid_size() {
        let points = get_test_data();

        let (Vector(xmin, xmax), Vector(ymin, ymax)) = grid_size(&points);

        assert_eq!(xmin, -6);
        assert_eq!(xmax, 15);
        assert_eq!(ymin, -4);
        assert_eq!(ymax, 11);
    }

    fn get_test_data() -> Vec<Point> {
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        input.lines().map(Point::from_str).collect()
    }
}
