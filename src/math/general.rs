//! Traits and functions for general purpose, everyday mathematics.
//! Everything you need.

use std::{convert::{TryInto, TryFrom}, ops::*, cmp::*};

use crate::eval_postfix;

use super::constants::{SQRT5, GOLDENRATIO};
pub mod avg_impl;
pub mod avg_macros;
/// Trait for several kinds of averages.
pub trait Averages<T> {
    type Output;
    /// Calculate the arithmetic mean.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(3.125, v.arithmetic_mean());
    /// ```
    #[must_use]
    fn arithmetic_mean(&self) -> Self::Output;
    /// Calculate the harmonic mean.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(2.318840579710145, v.harmonic_mean());
    /// ```
    #[must_use]
    fn harmonic_mean(&self) -> Self::Output;
    /// Calculate the median.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(2.5, v.median());
    /// ```
    #[must_use]
    fn median(&self) -> Self::Output;
    /// Calculate the mode.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1, 2, 2, 2, 3, 4, 5, 6];
    /// 
    /// assert_eq!(2, v.mode());
    /// ```
    #[must_use]
    fn mode(&self) -> T;
    /// Calculate the mid range.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(3.5, v.mid_range());
    /// ```
    #[must_use]
    fn mid_range(&self) -> Self::Output;
}
/// Trait for the Digits of a given number.
pub trait NumDigits {
    /// Calculates the cross sum of a number.
    /// # Returns
    /// A `Self` containing the result.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumDigits;
    /// 
    /// assert_eq!(3u8, 12.cross_sum());
    /// assert_eq!(9u16, 342.cross_sum());
    /// assert_eq!(52u64, 4928947234u64.cross_sum());
    /// ```
    #[must_use]
    fn cross_sum(&self) -> Self;
    /// Gets the digits as a number with the 1s place at index 0.
    /// # Returns
    /// A `Vec<u8>`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumDigits;
    /// 
    /// assert_eq!(vec![4,3,2,1], 1234u16.digits());
    /// assert_eq!(vec![0], 0u8.digits());
    /// ```
    #[must_use]
    fn digits(&self) -> Vec<u8>;
}
/// Trait for several useful generic functions.
pub trait NumTools<T> {
    /// Determine whether `self` is in the interval `[start; end]`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(true, 5.is_in_range(0, 10));
    /// assert_eq!(true, 5.is_in_range(0, 5));
    /// assert_eq!(false, 3.14.is_in_range(5.0, 10.0));
    /// ```
    #[must_use]
    fn is_in_range(&self, start: Self, end: Self) -> bool;
    /// Determine whether `self` is in the interval `(start; end)`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(true, 5.is_in_range_exclusive(0, 10));
    /// assert_eq!(false, 5.is_in_range_exclusive(0, 5));
    /// ```
    #[must_use]
    fn is_in_range_exclusive(&self, start: Self, end: Self) -> bool;
    /// Maps a given number of a range onto another range.
    /// # Arguments
    /// * `self` - The value which is to be mapped.
    /// * `start1` - The original start value of the number range.
    /// * `end1` - The original end value of the number range.
    /// * `start2` - The new start value of the number range.
    /// * `end2` - The new start value of the number range.
    /// # Returns
    /// A number containing the new mapped value.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let result: f32 = 5.0.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    /// assert_eq!(result, 0.5);
    /// ```
    #[must_use]
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T;
    /// Increment a number by one.
    /// # Returns
    /// Nothing.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.inc();
    /// assert_eq!(five, 6);
    /// ```
    fn inc(&mut self);
    /// Increment a number by a specified value.
    /// # Arguments
    /// * `n` - The value to be incremented by.
    /// # Returns
    /// Nothing.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.inc_by(2);
    /// assert_eq!(five, 7);
    /// ```
    fn inc_by(&mut self, n: Self);
    /// Decrement a number by one.
    /// # Returns
    /// Nothing.
    /// # Warning
    /// Does not check for underflow.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.dec();
    /// assert_eq!(five, 4);
    /// ```
    fn dec(&mut self);
    /// Decrement a number by a specified value.
    /// # Arguments
    /// * `n` - The value to be decremented by.
    /// # Returns
    /// Nothing.
    /// # Warning
    /// Does not check for underflow.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.dec_by(2);
    /// assert_eq!(five, 3);
    /// ```
    fn dec_by(&mut self, n: Self);
    /// Square a number.
    /// # Returns
    /// The square of the number.
    /// # Caution
    /// This function does not check if overflow occurs.
    /// # Examples
    /// ```
    ///use lib_rapid::math::general::NumTools;
    ///let i = 12;
    ///let res = i.square();
    ///assert_eq!(144, res);
    /// ```
    #[must_use]
    fn square(&self) -> Self;

    /// Cube a number.
    /// # Returns
    /// The cube of the number.
    /// # Caution
    /// This function does not check if overflow occurs.
    /// # Examples
    /// ```
    ///use lib_rapid::math::general::NumTools;
    ///let i = 12;
    ///let res = i.cube();
    ///assert_eq!(1728, res);
    /// ```
    #[must_use]
    fn cube(&self) -> Self;
}

impl<T: PartialOrd +
        Sub<Output = T> +
        Add<Output = T> +
        Div<Output = T> +
        Mul<Output = T> +
        From<u8> +
        Copy +
        SubAssign +
        AddAssign> NumTools<T> for T {
    #[inline]
    fn is_in_range(&self, start: Self, end: Self) -> bool {
        self >= &start &&
        self <= &end
    }

    #[inline]
    fn is_in_range_exclusive(&self, start: Self, end: Self) -> bool {
        self > &start &&
        self < &end
    }

    #[inline]
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T {
        eval_postfix!(start2 end2 start2 - + (*self) start1 - end1 / start1 - *)
    }

    #[inline]
    fn dec(&mut self) {
        *self -= 1u8.into();
    }

    #[inline]
    fn dec_by(&mut self, n: Self) {
        *self -= n;
    }

    #[inline]
    fn inc(&mut self) {
        *self += 1u8.into();
    }
    #[inline]
    fn inc_by(&mut self, n: Self) {
        *self += n;
    }
    #[inline]
    fn square(&self) -> Self {
        *self * *self
    }
    #[inline]
    fn cube(&self) -> Self {
        *self * *self * *self
    }
}

impl<'a, T: PartialEq +
        DivAssign + PartialOrd +
        Rem<Output = T> + Copy +
        AddAssign +
        Div<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Mul<Output = T> +
        AddAssign +
        SubAssign>
    NumDigits
    for T
    where
        u8: TryFrom<T>,
        T: From<u8> {
    fn cross_sum(&self) -> Self {
        let     d:   Vec<u8> = self.digits();
        let mut res: Self    = 0.into();
        for i in d {
            res.inc_by(i.into());
        }
        res
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u8.into() { return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(39);
        while v > 0.into() {
            let n: u8 = unsafe { (v % 10.into()).try_into().unwrap_unchecked() };
            if n == 255
            { panic!("Oops! Something went wrong. please contact the developers using the error code 0x00001.")}
            v /= 10.into();
            digits.push(n);
        }
        digits
    }
}
/// Compute the nth-root of a number.
/// # Arguments
/// * `degree` - the `f64` specifying the `n`.
/// * `radicand` - the `f64` of which the root should be computed.
/// # Returns
/// A `f64`.
/// # Examples
/// ```
/// use lib_rapid::math::general::nth_root;
/// use lib_rapid::math::constants;
/// assert_eq!(constants::CUBEROOT3, nth_root(3.0, 3.0));
/// ```
/// ```
/// use lib_rapid::math::general::nth_root;
/// use lib_rapid::math::constants;
/// assert_eq!(constants::SQRT2, nth_root(2.0, 2.0));
/// ```
#[must_use]
#[inline]
pub fn nth_root(degree: f64, radicand: f64) -> f64 {
    radicand.powf(degree.recip())
}
/// Computes th nth fibonacci number (up to 186th) accurately using the fastest available computing method.
/// # Arguments
/// * `n` - the nth-fibonacci number to be computed.
/// # How does it work?
/// This function uses the function `φⁿ / √5` for all numbers `n < 76`, as this is the maximum precision for that formula. For every number above that, it uses the iterative approach.
/// # Examples
/// ```
/// use lib_rapid::math::general::nth_fibonacci;
/// 
/// assert_eq!(1304969544928657, nth_fibonacci(74));
/// assert_eq!(332825110087067562321196029789634457848, nth_fibonacci(186));
/// ```
#[must_use]
pub fn nth_fibonacci(n: u128) -> u128 {
    if n > 186
    { panic!("Error: The 187th Fibonacci number and all above are not allowed, as they would cause a overflow in the u128 type."); }
    if n < 76 {
        return ( GOLDENRATIO.powi(n as i32) / SQRT5 ).round() as u128;
    }

    let mut x: u128;
    let mut y: u128 = 1304969544928657;
    let mut z: u128 = 2111485077978050;
    let mut i: u128 = 75;

    while i < n {
        x = y;
        y = z;
        z = x + y;
        i.inc();
    }

    z
}

/// Compute the absolute difference ("Delta") between two values.
/// # But Rust already has a abs_diff() function!
/// Yes, it does. But it does not have support for floating point numbers.
/// That's why we implemented a generic function.
/// # Arguments
/// - `a: T` - The first value.
/// - `b: T` - The second value.
/// # Returns
/// A `T`.
/// # Examples
/// ```
/// use lib_rapid::math::general::delta;
/// 
/// assert_eq!(4, delta(4, 8));
/// assert_eq!(3.14, delta(6.28, 3.14));
/// assert_eq!(4.905, delta(4.905, 9.81));
/// ```
pub fn delta<T: Sub<Output = T> +
                PartialOrd>(a: T, b: T) -> T {
    match a > b {
        true  => { return a - b; }
        false => { return b - a; }
    }
}