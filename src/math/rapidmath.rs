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
    ///use lib_rapid::math::rapidmath::CommonPowers;
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
/// Trait for prime functions.
pub trait Primality {
    /// Determines whether a number is prime in `O(sqrt n)`.
    /// # Returns
    /// `true` if the number is prime, otherwise `false`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::rapidmath::Primality;
    /// let p:     Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    /// let mut f: Vec<u64> = Vec::new();
    /// for i in 0..100 {
    ///     if (i as u64).is_prime() { f.push(i as u64); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
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

        while i.square() < *self {
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

        while i.square() <= *self {
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

        while i.square() <= *self {
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

        while i.square() <= *self {
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

        while i.square() <= *self {
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

        while i.square() <= *self {
            match self % i {
                0 => { return false; }
                _ => { i += 1; }
            }
        }
        true
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

/// Generate a list of prime numbers in the interval `[2;limit[`.
/// # Arguments
/// * `limit: usize` The limit up to which the function should search for primes.
/// # Returns
/// A `Vec<usize>` containing a list of primes.
/// # Examples
/// ```
/// use lib_rapid::math::rapidmath::generate_primes;
/// let p: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997];
/// let f: Vec<usize> = generate_primes(1000);
/// assert_eq!(p, f);
/// ```
pub fn generate_primes(limit: usize) -> Vec<usize> {
    match limit {
        0 | 1 => { return Vec::new(); }
        2     => { return vec![2]; }
        3     => { return vec![2, 3]; }
        _     => { }
    }

    let mut res: Vec<usize> = Vec::with_capacity(limit);
    res.push(2);
    res.push(3);
    let mut sieve: Vec<bool> = vec![false; limit];
    
    for x in (1..).take_while(|n| n * n < limit) {
        let mut n: usize;
        for y in (1..).take_while(|n| n * n < limit) {

            n = 4 * x.square() + y.square();
            if n <= limit && (n % 12 == 1 || n % 12 == 5)
            { sieve[n] ^= true; }

            n = 3 * x.square() + y.square();
            if n <= limit && n % 12 == 7
            { sieve[n] ^= true; }

            if x > y {
                n = 3 * x.square() - y.square();
                if x > y && n <= limit && n % 12 == 11
                { sieve[n] ^= true; }
            }
        }
    }
    
    for r in (5..).take_while(|n| n.square() < limit) {
        if sieve[r] {
            for i in (r.square()..limit).step_by(r.square()) {
                sieve[i] = false;
            }
        }
    }

    for i in 0..sieve.len() {
        if sieve[i] { res.push(i); }
    }
    res
}