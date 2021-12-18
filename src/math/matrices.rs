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
        let mut v: Vec<T> = Vec::with_capacity(self.stride);
        for i in self.stride * (row - 1)..self.stride * (row - 1) + self.stride - 1 {
            v.push(self.values[i]);
        }
        Some(v)
    }
}