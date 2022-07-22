//! Polynomials in Rust.

use std::{ops::{Add, Sub, SubAssign, AddAssign}, borrow::BorrowMut};

use crate::math::general::NumTools;

#[derive(Clone, Copy, Debug)]
pub struct Polynomial<const C: usize, T> {
    coefficients: [T; C],
    degree:       usize
}

impl<const C: usize, T: From<u8> +
                        Copy +
                        std::ops::Sub<Output = T> +
                        std::ops::Add<Output = T> +
                        std::ops::Div<Output = T> +
                        std::ops::Mul<Output = T> +
                        std::ops::Div +
                        std::ops::Mul + 
                        std::ops::SubAssign +
                        std::ops::AddAssign +
                        std::ops::MulAssign +
                        std::cmp::PartialOrd +
                        std::ops::Sub +
                        std::ops::Add> Polynomial<C, T> {
    pub fn new() -> Polynomial<C, T> {
        Polynomial {
            coefficients: [1u8.into(); C],
            degree:       C
        }
    }

    pub fn new_from_coefficients(coefficients: [T; C]) -> Polynomial<C, T> {
        Polynomial {
            coefficients,
            degree: coefficients.len()
        }
    }

    pub fn get_coefficients(&self) -> [T; C] {
        self.coefficients
    }

    pub fn get_degree(&self) -> usize {
        self.degree
    }

    pub fn eval(&self, x: T) -> T {
        let mut current_value:    T     = 0u8.into();
        let mut current_exponent: usize = C;
        for i in self.coefficients {
            current_value += i * x.pow(current_exponent);
            if current_exponent == 0
            { break; }
            current_exponent.dec()
        }

        current_value
    }
}

impl<const C: usize, T: std::ops::AddAssign + Copy> Add for Polynomial<C, T> {
    type Output = Polynomial<C, T>;

    fn add(self, rhs: Self) -> Self::Output {

        Polynomial {
            coefficients: {
                let mut res_coeff = self.coefficients;
                for (index, i) in rhs.coefficients.iter().enumerate() {
                    res_coeff[index] += *i;
                }
                res_coeff
            },
            degree:     self.degree
        }
    }
}

impl<const C: usize, T: std::ops::SubAssign +
                        Copy +
                        PartialEq +
                        From<u8>> Sub for Polynomial<C, T> {
    type Output = Polynomial<C, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res_coeff = self.coefficients;
        for (index, i) in rhs.coefficients.iter().enumerate() {
            res_coeff[index] -= *i;
        }
        let mut _deg = C;
        for i in res_coeff {
            if i == 0u8.into() {
                _deg.dec();
                continue;
            }
            break;
        }
        Polynomial {
            coefficients: res_coeff,
            degree:       _deg
        }
    }
}

impl<const C: usize, T: Copy +
                        Clone +
                        PartialEq +
                        Sub<Output = T> +
                        SubAssign +
                        From<u8>> SubAssign for Polynomial<C, T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}


impl<const C: usize, T: Copy +
                        Clone +
                        PartialEq +
                        Add<Output = T> +
                        AddAssign +
                        From<u8>> AddAssign for Polynomial<C, T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}