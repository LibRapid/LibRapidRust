use num_traits::NumOps;

use crate::arithmetic;

pub fn pow(base: f64, exponent: i32) -> f64 {
    let mut result: f64;
    let is_greater_zero: bool = exponent > 0;
    let base_is_two: bool = base == 2f64;

    match base_is_two {
        true => return f64::from(arithmetic::mult_pow2(1, exponent)),
        false => {}
    }

    match is_greater_zero {
        true => {
            result = base.clone();
            for _i in 1..exponent {
                result = result*base;
            }
        }
        false => {
            let new_exponent = exponent *-1;
            result = 1f64 / base;
            for _i in 1..new_exponent {
                result = result / base;
            }
        }
    }
    result
}

pub fn map<T: NumOps + Copy>(value: T, start1: T, end1: T, start2: T, end2: T) -> T {

    (start2 + (end2 - start2)).mul((value - start1) / end1.sub(start1))
}