//! Traits and functions for general purpose, everyday mathematics.
//! Temperature conversion, angle conversion etc. Everything you need.
use super::constants;
use super::ztheory::*;

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
    /// A `Self` containing the result.
    fn cross_sum(&self) -> Self;
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
    /// 
    /// assert_eq!(false, (-2).is_prime());
    /// assert_eq!(true, 2.is_prime());
    /// assert_eq!(false, 1.is_prime());
    /// assert_eq!(false, 0.is_prime());
    /// ```
    /// ```
    /// use lib_rapid::math::rapidmath::Primality;
    /// use lib_rapid::math::rapidmath::generate_primes;
    /// 
    /// let _p: Vec<usize>  = generate_primes(1000);
    /// let p:  Vec<u64>    = _p.into_iter().map(|x: usize| x as u64).collect::<Vec<u64>>();
    /// let mut f: Vec<u64> = Vec::new();
    /// for i in 0..1000 {
    ///     if (i as u64).is_prime() { f.push(i as u64); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
    /// ```
    /// use lib_rapid::math::rapidmath::Primality;
    /// use lib_rapid::math::rapidmath::generate_primes;
    /// 
    /// let _p: Vec<usize> = generate_primes(100);
    /// let p:  Vec<u8>    = _p.into_iter().map(|x: usize| x as u8).collect::<Vec<u8>>();
    /// let mut f: Vec<u8> = Vec::new();
    /// for i in 0..100 {
    ///     if (i as u8).is_prime() { f.push(i as u8); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
    /// ```
    /// use lib_rapid::math::rapidmath::Primality;
    /// use lib_rapid::math::rapidmath::generate_primes;
    /// 
    /// let _p: Vec<usize>  = generate_primes(1000);
    /// let p:  Vec<u16>    = _p.into_iter().map(|x: usize| x as u16).collect::<Vec<u16>>();
    /// let mut f: Vec<u16> = Vec::new();
    /// 
    /// for i in 0..1000 {
    ///     if (i as u16).is_prime() { f.push(i as u16); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
    /// ```
    /// use lib_rapid::math::rapidmath::Primality;
    /// use lib_rapid::math::rapidmath::generate_primes;
    /// 
    /// let _p: Vec<usize>  = generate_primes(1000);
    /// let p:  Vec<u32>    = _p.into_iter().map(|x: usize| x as u32).collect::<Vec<u32>>();
    /// let mut f: Vec<u32> = Vec::new();
    /// for i in 0..1000 {
    ///     if (i as u32).is_prime() { f.push(i as u32); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
    /// ```
    /// use lib_rapid::math::rapidmath::Primality;
    /// use lib_rapid::math::rapidmath::generate_primes;
    /// 
    /// let _p: Vec<usize>  = generate_primes(1000);
    /// let p:  Vec<u128>    = _p.into_iter().map(|x: usize| x as u128).collect::<Vec<u128>>();
    /// let mut f: Vec<u128> = Vec::new();
    /// for i in 0..1000 {
    ///     if (i as u128).is_prime() { f.push(i as u128); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
    fn is_prime(&self) -> bool;
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
        let mut v: Self = *self;
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
        let mut v: Self = *self;
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

impl Primality for u8 { // promoted to u16 for a more optimized check
    fn is_prime(&self) -> bool {
        (*self as u16).is_prime()
    }
        
}

impl Primality for u16 {// Splits 
    fn is_prime(&self) -> bool {
        const PRIMELIST: [u16; 54] = [// list of all primes less than 2^8
           2,  3,  5,   7,  11,  13,  17,  19,  23,  29,  31,
          37, 41, 43,  47,  53,  59,  61,  67,  71,  73,  79, 
          83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 
         139,149,151, 157, 163, 167, 173, 179, 181, 191, 193,
         197,199,211, 223, 227, 229, 233, 239, 241, 251
        ];
       
        if self == &1 ||
           self == &0
        { return false; }
       
        if PRIMELIST.contains(self)
        { return true; }
        
        return (*self as u64).is_prime();
    }
}

impl Primality for u32 { // too large to check primality by trial division so promoted to u64 where SPRP is performed
    fn is_prime(&self) -> bool {
        (*self as u64).is_prime()
    }
}

impl Primality for i32 {
    fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
        (self.abs() as u64).is_prime()
    }    
}

impl Primality for u64 {
    fn is_prime(&self) -> bool {
     
        const PRIME_BASES: [u64;12] = [2,3,5,7,11, 13, 17, 19, 23, 29, 31, 37];
            if self == &1 ||
               self == &0
            { return false; }
        
        for i in PRIME_BASES {  // Checks if self is a member of PRIME_BASES or divisible by a member
            if *self == i { return true; }
            if *self % i == 0 { return false; }
        }
        
        for i in PRIME_BASES {        // performs Strong Fermat test using each base. Equivalent to Deterministic Miller Rabin
            if sprp(*self,i) == false {
                return false;
            }
        }
        return true;
    }
}

impl Primality for i64 {
   fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
        (self.abs() as u64).is_prime()
   }
}
// Very slow primality check, I'll work out how to do BPSW for 128-bit
impl Primality for u128 {
    fn is_prime(&self) -> bool {
        if *self == 1 { return false; }
        const PRIMES:    [u64;27] = [2,3,5,7,11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103];
        const PRIMORIAL: u128     = 210;
        let x = *self;  // too lazy to dereference at every point
     
        for i in PRIMES {
            if x == i as u128 { return true; }
            if x % i as u128 == 0 { return false; }
        } 
     
        if x.leading_zeros() > 63 {  // if x is smaller than 2^64 call deterministic miller rabin
            for i in PRIMES[..12].iter(){
                if sprp(x as u64,*i) == false {
                return false;
                }
            }
            return true;
        }
     
        let supremum = ( ( (x as f64).sqrt() as u128 + 103u128) / PRIMORIAL) + 1u128; // Else perform trial division using the 11-rough numbers (higher-density of primes)
        for i in 1..supremum {
      
            if x % (PRIMORIAL * i - 1) == 0 ||
               x % (PRIMORIAL * i + 1) == 0 {
                return false;
            } 
     
            for j in PRIMES[4..].iter(){
                if x % (PRIMORIAL * i - *j as u128) == 0 ||
                   x % (PRIMORIAL * i + *j as u128) == 0 {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Primality for i128{
    fn is_prime(&self)-> bool {
        if self <= &0 { return false; }
        (self.abs() as u128).is_prime()
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
/// let p: Vec<usize> = vec![2, 3, 5, 7, 11, 13];
/// let f: Vec<usize> = generate_primes(15);
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
