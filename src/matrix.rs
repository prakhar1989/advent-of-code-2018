// A 2D matrix implementation that's implemented with a 1D vector

use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    buf: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    /// Creates a new matrix with an initial value of height and width
    pub fn new(height: usize, width: usize, initial: T) -> Matrix<T> {
        Matrix {
            height,
            width,
            buf: vec![initial; height * width],
        }
    }
}

impl<T> Matrix<T> {
    fn index_of(&self, s: (usize, usize)) -> usize {
        assert!(s.0 < self.height);
        assert!(s.1 < self.width);

        s.0 * self.width + s.1
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> {
        let width = self.width;

        (0..self.height).map(move |y| (0..width).map(|x| &self[(y, x)]).collect())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, s: (usize, usize)) -> &T {
        let i = self.index_of(s);
        &self.buf[i]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, s: (usize, usize)) -> &mut T {
        let i = self.index_of(s);
        &mut self.buf[i]
    }
}

impl<T> Debug for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        for y in 0..self.height {
            write!(f, "[")?;
            for x in 0..self.width {
                write!(f, "{:?}", self[(y, x)])?;
                if x < self.width - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if y < self.height - 1 {
                write!(f, "\n ")?;
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut m = Matrix::new(3, 4, 4.0);
        m[(0, 0)] = 33.0;
        m[(1, 0)] = 22.0;
        m[(1, 2)] = 11.0;

        assert_eq!(m[(1, 1)], 4.0);
        assert_eq!(m[(0, 0)], 33.0);
        assert_eq!(m[(1, 0)], 22.0);
        assert_eq!(m[(1, 2)], 11.0);

        assert_eq!(
            format!("{:?}", m),
            "[[33.0, 4.0, 4.0, 4.0]
 [22.0, 4.0, 11.0, 4.0]
 [4.0, 4.0, 4.0, 4.0]]"
        );
    }

    #[test]
    fn test_matrix_iter() {
        let m = Matrix::new(3, 3, 5);
        let total = m
            .rows()
            .flat_map(|r| r.into_iter())
            .fold(0, |acc, val| acc + val);

        assert_eq!(total, 3 * 3 * 5);
    }
}
