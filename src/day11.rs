mod matrix;

type Point = (usize, usize);
type Grid = matrix::Matrix<i32>;

fn main() {
    let serial_number = 6392;
    let area_table = summed_area_table(&build_grid(serial_number));
    println!("Part 1: {:?}", find_fuel_cell(&area_table, 3).0);
    println!("Part 2: {:?}", largest_cell(&area_table));
}

fn largest_cell(area_table: &Grid) -> (Point, usize) {
    let mut best_size = 1;
    let mut best_point = (0, 0);
    let mut max_power = 0;

    for s in 1..300 {
        let (point, power) = find_fuel_cell(area_table, s);
        if power > max_power {
            best_size = s;
            best_point = point;
            max_power = power;
        }
    }

    (best_point, best_size)
}

/// converts a table to partial sum as per the algo described here:
/// https://en.wikipedia.org/wiki/Summed-area_table
fn summed_area_table(m: &Grid) -> Grid {
    let mut table = matrix::Matrix::new(m.height, m.width, 0);

    table[(0, 0)] = m[(0, 0)];

    for i in 1..table.width {
        table[(i, 0)] = m[(i, 0)] + table[(i - 1, 0)];
    }

    for j in 1..table.height {
        table[(0, j)] = m[(0, j)] + table[(0, j - 1)];
    }

    for i in 1..table.width {
        for j in 1..table.height {
            table[(i, j)] =
                m[(i, j)] + table[(i - 1, j)] + table[(i, j - 1)] - table[(i - 1, j - 1)];
        }
    }

    table
}

fn find_fuel_cell(area_table: &Grid, size: usize) -> (Point, i32) {
    let mut max_power = 0;
    let mut max_cell = (0, 0);

    for i in 0..(300 - size) {
        for j in 0..(300 - size) {
            let power = total_power((i, j), &area_table, size);
            if power > max_power {
                max_cell = (i, j);
                max_power = power;
            }
        }
    }

    (max_cell, max_power)
}

fn build_grid(serial_number: i32) -> Grid {
    let mut m = matrix::Matrix::new(300, 300, 0);

    for i in 0..300 {
        for j in 0..300 {
            m[(i, j)] = power_level((i, j), serial_number);
        }
    }

    m
}

/// Calculates the total area of a summed area table as per formula in
/// https://en.wikipedia.org/wiki/Summed-area_table
/// The variable names A, B, C, D correspond to the algorithm shown above
fn total_power(point: Point, summed_table: &Grid, size: usize) -> i32 {
    let (i, j) = point;
    let d = summed_table[(i + size - 1, j + size - 1)];
    let a = if (i as isize) - 1 >= 0 && (j as isize) - 1 >= 0 {
        summed_table[(i - 1, j - 1)]
    } else {
        0
    };

    let b = if (j as isize) - 1 >= 0 {
        summed_table[(i + size - 1, j - 1)]
    } else {
        0
    };

    let c = if (i as isize) - 1 >= 0 {
        summed_table[(i - 1, j + size - 1)]
    } else {
        0
    };

    d + a - b - c
}

fn power_level(point: Point, serial_number: i32) -> i32 {
    let (x, y) = point;
    let rack_id = (x as i32) + 10;
    let j = ((rack_id * y as i32) + serial_number) * rack_id;

    hundredth_digit(j) - 5
}

fn hundredth_digit(n: i32) -> i32 {
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
        let area_table = summed_area_table(&build_grid(18));
        assert_eq!(total_power((33, 45), &area_table, 3), 29);
    }

    #[test]
    fn day11_find_fuel_cell() {
        let area_table = summed_area_table(&build_grid(6392));
        assert_eq!(find_fuel_cell(&area_table, 3).0, (20, 58));
    }

    #[test]
    fn day11_summed_area_table() {
        let mut m = matrix::Matrix::new(3, 3, 0);
        let v = vec![5, 2, 3, 1, 5, 4, 2, 2, 1];

        for i in 0..3 {
            for j in 0..3 {
                m[(i, j)] = v[i * 3 + j];
            }
        }

        let summed_table = summed_area_table(&m);
        assert_eq!(
            format!("{:?}", summed_table),
            "[[5, 7, 10]
 [6, 13, 20]
 [8, 17, 25]]"
        );
    }
}
