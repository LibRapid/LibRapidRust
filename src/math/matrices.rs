//! Mathematical matrices in Rust.
use crate::eval_postfix;

#[derive(Clone)]
pub struct Matrix<T> {
    stride: usize,
    values: Vec<T>,
}

#[allow(dead_code)]

impl<T: Copy> Matrix<T> {
    pub fn new(vals: Vec<T>, row_len: usize) -> Option<Matrix<T>> {
        if vals.len() % row_len != 0
        { return None; }
        Some(Matrix { stride: row_len, values: vals })
    }

    pub fn element_at(&self, row: usize, column: usize) -> Option<&T> {
        if row > self.stride / 10 ||
           column > self.stride ||
           column == 0 ||
           row == 0
        { return None; }

        Some(&self.values[eval_postfix!((self.stride) row 1 - * column + 1 -)])
    }

    pub fn nth_row(&self, row: usize) -> Option<Vec<T>> {
        if row == 0 ||
           row > self.stride / 10
        { return None; }

        let mut res: Vec<T> = Vec::with_capacity(self.stride);

        for i in self.row_beginning(row)..=self.row_ending(row) {
            res.push(self[i]);
        }
        Some(res)
    }

    fn row_beginning(&self, row: usize) -> usize {
        eval_postfix!((self.stride) row 1 - *)
    }

    fn row_ending(&self, row: usize) -> usize {
        eval_postfix!((self.stride) row 1 - * (self.stride) + 1 -)
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.values.len() != rhs.values.len()
        { core::panic!("Error: Lengths of Vectors did not match.") }
        let mut res: Matrix<T> = Matrix::new(
                                   Vec::with_capacity(self.values.len()), 
                                self.stride).unwrap();
        for i in 0..self.values.len() {
            res.values.push(self[i] + rhs[i]);
        }
        res
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.values.len() != rhs.values.len()
        { core::panic!("Error: Lengths of Vectors did not match.") }
        let mut res: Matrix<T> = Matrix::new(
                                   Vec::with_capacity(self.values.len()), 
                                self.stride).unwrap();
        for i in 0..self.values.len() {
            res.values.push(self[i] - rhs[i]);
        }
        res
    }
}