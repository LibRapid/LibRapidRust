#![allow(unused_macros)]

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn get_dimensions(self: Matrix<T>) -> [usize; 2] {
        [self.data.len(), self.data[0].len()]
    }
}

macro_rules! matrix {
    ( $ ( $x:expr ),* ) => {
        let mut temp_mat = Matrix::new();
        $(
            temp_mat.data.push($x);
        )*
        temp_mat
    };
}