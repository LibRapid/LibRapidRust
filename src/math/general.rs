//! Traits and functions for general purpose, everyday mathematics.
//! Everything you need.

/// Trait for the cross sum of a given number.
pub trait CrossSum<T> {
    /// Calculates the cross sum of a number.
    ///
    /// # Returns
    /// A `Self` containing the result.
    fn cross_sum(&self) -> Self;
}

/// Trait for left-shifting decimal-numbers.
#[deprecated(note = "This feature is deprecated, as it has not been proven to be faster than multiplying by 10. Use at your own risk.")]
pub trait DecimalLeftShift<T> {
    /**
    Multiplies by 10 (shifts the decimal places to the left by 1) while being more efficient.

    # Returns
    The new shifted number.
    */
    #[deprecated(note = "This feature is deprecated, as it has not been proven to be faster than multiplying by 10. Use at your own risk.")]
    fn dec_lshift(&self) -> T;
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
    /// let result: f32 = 5f32.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    /// assert_eq!(result, 0.5);
    /// ```
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
    fn square(&self) -> Self;

    /// Cube a number.
    /// # Returns
    /// The cube of the number.
    /// # Caution
    /// This function does not check if overflow occurs.
    fn cube(&self) -> Self;
}

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
    /// use lib_rapid::math::general::Increment;
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

impl<T> CrossSum<T> for u8 {
    fn cross_sum(&self) -> Self {
        let mut v: Self = *self;
        let mut digits: Vec<Self> = Vec::with_capacity(3);
        while v > 0 {
            let n = (self % 10) as u8;
            v /= 10;
            digits.push(n);
        }
        digits.iter().sum()
    }
}

impl<T> CrossSum<T> for u16 {
    fn cross_sum(&self) -> Self {
        let mut v: Self = *self;
        let mut digits: Vec<Self> = Vec::with_capacity(5);
    
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n);
        }
        digits.iter().sum()
    }
}

impl<T> CrossSum<T> for u32 {
    fn cross_sum(&self) -> Self {
        let mut v: Self = *self;
        let mut digits: Vec<Self> = Vec::with_capacity(10);
    
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n);
        }
        digits.iter().sum()
    }
}

impl<T> CrossSum<T> for u64 {
    fn cross_sum(&self) -> Self {
        let mut v:      Self      = *self;
        let mut digits: Vec<Self> = Vec::with_capacity(20);
    
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n);
        }
        digits.iter().sum()
    }
}

impl<T> CrossSum<T> for u128 {
    fn cross_sum(&self) -> Self {
        let mut v:      Self      = *self;
        let mut digits: Vec<Self> = Vec::with_capacity(39);
    
        while v > 0 {
            let n: Self = v % 10;
            v /= 10;
            digits.push(n);
        }
        digits.iter().sum()
    }
}

impl<T: std::ops::Add<Output = T> + 
        std::ops::Sub<Output = T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Div<Output = T> + 
        Copy> MapToNumRange<T> for T {
            fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T {
                (start2 + (end2 - start2)) * ((*self - start1) / end1 - start1)
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