mod matrix;

type Point = (usize, usize);

fn main() {
    let serial_number = 6392;
    let m = build_grid(serial_number);
    println!("Part 1: {:?}", find_fuel_cell(&m, 3).0);
    println!("Part 2: {:?}", largest_cell(&m));
}

fn largest_cell(m: &matrix::Matrix<i32>) -> (Point, usize) {
    let mut best_size = 1;
    let mut best_point = (0, 0);
    let mut max_power = 0;

    for s in 1..300 {
        let (point, power) = find_fuel_cell(m, s);
        if power > max_power {
            best_size = s;
            best_point = point;
            max_power = power;
        }
    }

    (best_point, best_size)
}

fn find_fuel_cell(m: &matrix::Matrix<i32>, size: usize) -> (Point, i32) {
    let mut max_power = 0;
    let mut max_cell = (0, 0);

    for i in 0..(300-size) {
        for j in 0..(300-size) {
            let power = total_power((i, j), &m, size);
            if power > max_power {
                max_cell = (i, j);
                max_power = power;
            }
        }
    }

    (max_cell, max_power)
}

fn build_grid(serial_number: i32) -> matrix::Matrix<i32> {
    let mut m = matrix::Matrix::new(300, 300, 0);

    for i in 0..300 {
        for j in 0..300 {
            m[(i, j)] = power_level((i, j), serial_number);
        }
    }

    m
}

fn total_power(point: Point, grid: &matrix::Matrix<i32>, size: usize) -> i32 {
    let mut area = 0;

    for i in 0..size {
        for j in 0..size {
            area += grid[(point.0 + i, point.1 + j)];
        }
    }

    area
}

fn power_level(point: Point, serial_number: i32) -> i32 {
    let (x, y) = point;
    let rack_id = (x as i32) + 10;
    let j = ((rack_id * y as i32) + serial_number) * rack_id;

    hundered_digit(j) - 5
}

fn hundered_digit(n: i32) -> i32 {
    ((n - (n % 100)) / 100) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_power_level() {
        assert_eq!(power_level((3, 5), 8), 4);
        assert_eq!(power_level((122, 79), 57), -5);
        assert_eq!(power_level((217, 196), 39), 0);
        assert_eq!(power_level((101, 153), 71), 4);
    }

    #[test]
    fn day11_build_grid() {
        let m = build_grid(18);
        assert_eq!(m[(33, 45)], 4);
        assert_eq!(m[(34, 45)], 4);
        assert_eq!(m[(35, 45)], 4);
        assert_eq!(m[(35, 46)], 4);
        assert_eq!(m[(35, 47)], 4);
        assert_eq!(m[(33, 46)], 3);
        assert_eq!(m[(33, 47)], 1);
    }

    #[test]
    fn day11_total_power() {
        let m = build_grid(18);
        assert_eq!(total_power((33, 45), &m, 3), 29);
    }

    #[test]
    fn day11_find_fuel_cell() {
        let m = build_grid(6392);
        assert_eq!(find_fuel_cell(&m, 3).0, (20, 58));
    }
}
