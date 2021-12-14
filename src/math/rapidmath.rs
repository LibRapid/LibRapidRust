//! Traits and functions for general purpose, everyday mathematics.
//! Temperature conversion, angle conversion etc. Everything you need.
use super::constants;

/// The conversion algorithm to be chosen. Used by `temp_conversion`.
pub enum TempConversion {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,

    FahrenheitToKelvin,
    KelvinToFahrenheit,

    CelsiusToKelvin,
    KelvinToCelsius,
}
/// The conversion algorithm to be chosen. Used by `angle_conversion`.
pub enum AngleConversion {
    DegreesToRadians,
    RadiansToDegrees,
}
/// Trait for angle conversion.
pub trait AngleConversionTrait {
    /// Performs a angle conversion.
    ///
    /// # Arguments
    /// * `&self` - The value to be converted.
    /// * `mode` - The mode ( e.g. RadiansToDegrees ).
    ///
    /// # Returns
    /// A `Self` containing the result.
    fn angle_conversion(&self, mode: AngleConversion) -> Self;
}
/// Trait for the cross sum of a given number.
pub trait CrossSum<T> {
    /// Calculates the cross sum of a number.
    ///
    /// # Returns
    /// A `usize` containing the result.
    fn cross_sum(&self) -> usize;
}
/// Trait for temperature conversion.
pub trait TempConversionTrait {
    /// Performs a temperature conversion.
    ///
    /// # Arguments
    /// * `&self` - The value to be converted.
    /// * `mode` - The mode ( e.g. CelsiusToFahrenheit ).
    ///
    /// # Returns
    /// A `Self` containing the result.
    fn temp_conversion(&self, mode: TempConversion) -> Self;
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
    /// use lib_rapid::math::rapidmath::MapToNumRange;
    ///
    /// let result: f32 = 5f32.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    /// std::println!("{}", result.to_string()) // Prints "0.5"
    /// ```
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T;
}

/// Square a number. Pretty edgy, isn't it?
pub trait Square {
    /// Square a number. Pretty edgy, isn't it?
    /// # Returns
    /// The square of the number.
    /// # Caution
    /// This function does not check if overflow occurs.
    fn square(&self) -> Self;
}
/// Trait for prime functions.
pub trait Primality {
    /// Determines whether a number is prime in `O(sqrt n)`.
    /// # Returns
    /// `true` if the number is prime, otherwise `false`.
    fn is_prime(&self) -> bool;
}

impl<T: std::fmt::Display> CrossSum<T> for T {
    fn cross_sum(&self) -> usize {
        let self_str: String = self.to_string();
        self_str.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() as usize
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

impl TempConversionTrait for f64 {
    fn temp_conversion(&self, mode: TempConversion) -> Self {
        match mode {
            TempConversion::CelsiusToFahrenheit => { return self * 1.8 + 32.0; }
            TempConversion::CelsiusToKelvin     => { return self + 273.15; }
            TempConversion::FahrenheitToCelsius => { return (self - 32.0) / 1.8; }
            TempConversion::FahrenheitToKelvin  => { return (self - 32.0) / 1.8 + 273.15; }
            TempConversion::KelvinToCelsius     => { return self - 273.15; }
            TempConversion::KelvinToFahrenheit  => { return (self - 273.15) * 1.8 + 32.0; }
        }
    }
}

impl TempConversionTrait for f32 {
    fn temp_conversion(&self, mode: TempConversion) -> Self {
        match mode {
            TempConversion::CelsiusToFahrenheit => { return self * 1.8 + 32.0; }
            TempConversion::CelsiusToKelvin     => { return self + 273.15; }
            TempConversion::FahrenheitToCelsius => { return (self - 32.0) / 1.8; }
            TempConversion::FahrenheitToKelvin  => { return (self - 32.0) / 1.8 + 273.15; }
            TempConversion::KelvinToCelsius     => { return self - 273.15; }
            TempConversion::KelvinToFahrenheit  => { return (self - 273.15) * 1.8 + 32.0; }
        }
    }
}

impl AngleConversionTrait for f64 {
    fn angle_conversion(&self, mode: AngleConversion) -> Self {
        match mode {
            AngleConversion::RadiansToDegrees => { self * constants::RADDEGRATE }
            AngleConversion::DegreesToRadians => { self * constants::DEGRADRATE }
        }
    }
}

impl AngleConversionTrait for f32 {
    fn angle_conversion(&self, mode: AngleConversion) -> Self {
        match mode {
            AngleConversion::RadiansToDegrees => { self * constants::RADDEGRATE as f32 }
            AngleConversion::DegreesToRadians => { self * constants::DEGRADRATE as f32 }
        }
    }
}

impl Primality for u8 {
    fn is_prime(&self) -> bool {
        if self < &2 { return false; }
        let mut i = 2;

        while i*i < *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
    }
}

impl Primality for u16 {
    fn is_prime(&self) -> bool {
        if self < &2 { return false; }
        let mut i = 2;

        while i*i <= *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
    }
}

impl Primality for u32 {
    fn is_prime(&self) -> bool {
        if self < &2 { return false; }
        let mut i = 2;

        while i*i <= *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
    }
}

impl Primality for i32 {
    fn is_prime(&self) -> bool {
        if self < &2 { return false; }
        let mut i = 2;

        while i*i <= *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
    }
}

impl Primality for u64 {
    fn is_prime(&self) -> bool {
        if self < &2 { return false; }
        let mut i = 2;

        while i*i <= *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
    }
}

impl Primality for u128 {
    fn is_prime(&self) -> bool {
        if self < &2 { return false; }
        let mut i = 2;

        while i*i <= *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> Square for T {
    fn square(&self) -> Self {
        *self * *self
    }
}

pub fn generate_primes(limit: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = vec![false; limit];
    for x in (1..).take_while(|n| n * n < limit) {
        for y in (1..).take_while(|n| n * n < limit) {
            let n = 4 * x.square() + y.square();
            if n <= limit && (n % 12 == 1 || n % 12 == 5) { sieve[n] ^= true; }

            let n = 3 * x.square() + y.square();
            if n <= limit && n % 12 == 7 { sieve[n] ^= true; }

            if x > y {
                let n = 3 * x.square() - y.square();
                if x > y && n <= limit && n % 12 == 11 { sieve[n] ^= true; }
            }
        }
    }
    
    (5..).take_while(|n| n.square() < limit).for_each(|r| {
        if sieve[r] {
            (r.square() .. limit).step_by(r.square())
            .for_each(|i| sieve.get_mut(i)
                                 .map(|b| *b = false)
            .unwrap());
        }
    });

    let mut res: Vec<usize> = Vec::with_capacity(sieve.len());
    res.push(2);
    res.push(3);
    for i in 0..sieve.len() {
        if sieve[i] { res.push(i); }
    }
    res
}