//! Traits and functions for handling primes and primality.
use super::ztheory::*;
use super::general::*;

/// Trait for prime functions.
pub trait Primality {
    /// Determines whether a number is prime.
    /// # Returns
    /// `true` if the number is prime, otherwise `false`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::primes::Primality;
    /// 
    /// assert_eq!(false, (-2).is_prime());
    /// assert_eq!(true, 2.is_prime());
    /// assert_eq!(false, 1.is_prime());
    /// assert_eq!(false, 0.is_prime());
    /// ```
    /// ```
    /// use lib_rapid::math::primes::{Primality, generate_primes};
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
    /// use lib_rapid::math::primes::{Primality, generate_primes};
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
    /// use lib_rapid::math::primes::{Primality, generate_primes};
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
    /// use lib_rapid::math::primes::{Primality, generate_primes};
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
    /// use lib_rapid::math::primes::{Primality, generate_primes};
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
        (*self as u64).is_prime()
    }    
}

impl Primality for u64 {
    fn is_prime(&self) -> bool {
     
        const PRIME_BASES: [u64;12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
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
        (*self as u64).is_prime()
   }
}
// Very slow primality check, I'll work out how to do BPSW for 128-bit
impl Primality for u128 {
    fn is_prime(&self) -> bool {
        if *self == 1 { return false; }
        const PRIMES:    [u64;27] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103];
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
        (*self as u128).is_prime()
    }
}

/// Generate a list of prime numbers in the interval `[2;limit[`.
/// # Arguments
/// * `limit: usize` The limit up to which the function should search for primes.
/// # Returns
/// A `Vec<usize>` containing a list of primes.
/// # Examples
/// ```
/// use lib_rapid::math::primes::generate_primes;
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