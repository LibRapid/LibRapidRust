//! Traits and functions for general purpose, everyday mathematics.
//! Everything you need.

use crate::eval_postfix;

/// Trait for the Digits of a given number.
pub trait NumDigits {
    /// Calculates the cross sum of a number.
    ///
    /// # Returns
    /// A `Self` containing the result.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumDigits;
    /// 
    /// assert_eq!(3u8, 12.cross_sum());
    /// assert_eq!(9u16, 342.cross_sum());
    /// assert_eq!(52u64, 4928947234.cross_sum());
    /// ```
    #[must_use]
    fn cross_sum(&self) -> Self;
    /// Gets the digits as a number with the 1s place at index 0.
    ///
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
/// Trait for mapping numbers to another number range.
pub trait MapToNumRange<T> {
    /// Maps a given number of a range onto another range.
    ///
    /// # Arguments
    /// * `self` - The value which is to be mapped.
    /// * `start1` - The original start value of the number range.
    /// * `end1` - The original end value of the number range.
    /// * `start2` - The new start value of the number range.
    /// * `end2` - The new start value of the number range.
    ///
    /// # Returns
    /// A number containing the new mapped value.
    ///
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::MapToNumRange;
    ///
    /// let result: f32 = 5.0.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    /// assert_eq!(result, 0.5);
    /// ```
    #[must_use]
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T;
}

/// Common powers.
pub trait CommonPowers {
    /// Square a number.
    /// # Returns
    /// The square of the number.
    /// # Caution
    /// This function does not check if overflow occurs.
    /// # Examples
    /// ```
    ///use lib_rapid::math::general::CommonPowers;
    ///let i = 12;
    ///let res = i.square();
    ///
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
    ///use lib_rapid::math::general::CommonPowers;
    ///let i = 12;
    ///let res = i.cube();
    ///
    ///assert_eq!(1728, res);
    /// ```
    #[must_use]
    fn cube(&self) -> Self;
}
/// Trait for incrementing by value. Shorthand syntax for `x += y;`.
pub trait Increment {
    /// Increment a number by one.
    /// # Returns
    /// Nothing.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Increment;
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
    /// use lib_rapid::math::general::Increment;
    /// let mut five: i32 = 5;
    /// five.inc_by(2);
    /// assert_eq!(five, 7);
    /// ```
    fn inc_by(&mut self, n: Self);
}
/// Trait for incrementing by value. Shorthand syntax for `x -= y;`.
pub trait Decrement {
    /// Decrement a number by one.
    /// # Returns
    /// Nothing.
    /// # Warning
    /// Does not check for underflow.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Decrement;
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
    /// use lib_rapid::math::general::Decrement;
    /// let mut five: i32 = 5;
    /// five.dec_by(2);
    /// assert_eq!(five, 3);
    /// ```
    fn dec_by(&mut self, n: Self);
}

impl<T: std::ops::AddAssign + From<u8>> Increment for T {
    fn inc(&mut self) {
        *self += 1u8.into();
    }

    fn inc_by(&mut self, n: Self) {
        *self += n;
    }
}

impl<T: std::ops::SubAssign + From<u8>> Decrement for T {
    fn dec(&mut self) {
        *self -= 1u8.into();
    }

    fn dec_by(&mut self, n: Self) {
        *self -= n;
    }
}

impl NumDigits for u8 {
    fn cross_sum(&self) -> Self {
        self.digits().iter().sum()
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u8{ return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(3);
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n);
        }
        digits
    }
}

impl NumDigits for u16 {
    fn cross_sum(&self) -> Self {
        let mut res = 0;
        for i in self.digits() {
            res.inc_by(i as Self);
        }
        res
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u16 { return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(5);
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n as u8);
        }
        digits
    }
}

impl NumDigits for u32 {
    fn cross_sum(&self) -> Self {
        let mut res = 0;
        for i in self.digits() {
            res.inc_by(i as Self);
        }
        res
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u32 { return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(10);
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n as u8);
        }
        digits
    }
}

impl NumDigits for u64 {
    fn cross_sum(&self) -> Self {
        let mut res = 0;
        for i in self.digits() {
            res.inc_by(i as Self);
        }
        res
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u64 { return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(20);
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n as u8);
        }
        digits
    }
}

impl NumDigits for u128 {
    fn cross_sum(&self) -> Self {
        let mut res = 0;
        for i in self.digits() {
            res.inc_by(i as Self);
        }
        res
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u128 { return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(39);
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n as u8);
        }
        digits
    }
}

impl<T: std::ops::Add<Output = T> + 
        std::ops::Sub<Output = T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Div<Output = T> + 
        Copy> MapToNumRange<T> for T {
            fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T {
                eval_postfix!(start2 end2 start2 - + (*self) start1 - end1 / start1 - *)
            }
        }

impl<T: std::ops::Mul<Output = T> + Copy> CommonPowers for T {
    fn square(&self) -> Self {
        *self * *self
    }

    fn cube(&self) -> Self {
        *self * *self * *self
    }
}