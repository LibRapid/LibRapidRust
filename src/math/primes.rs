//! Traits and functions for handling primes and primality.
use super::general::*;

use number_theory::traits::NumberTheory;



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
    /// assert_eq!(true, 8191.is_prime());
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
    /// let _p: Vec<usize>   = generate_primes(1000);
    /// let p:  Vec<u128>    = _p.into_iter().map(|x: usize| x as u128).collect::<Vec<u128>>();
    /// let mut f: Vec<u128> = Vec::new();
    /// for i in 0..1000 {
    ///     if (i as u128).is_prime() { f.push(i as u128); }
    /// }
    /// 
    /// assert_eq!(p, f);
    /// ```
    #[must_use]
    fn is_prime(&self) -> bool;
}

impl Primality for u8 { // promoted to u16 for a more optimized check
    fn is_prime(&self) -> bool {
        NumberTheory::is_prime(self)
    }
        
}

impl Primality for u16 {// Splits 
   fn is_prime(&self) -> bool {
       NumberTheory::is_prime(self)
    }
}

impl Primality for u32 { 
    fn is_prime(&self) -> bool {
         NumberTheory::is_prime(self)
    }
}

impl Primality for i32 {
    fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
         NumberTheory::is_prime(self)
    }
}

impl Primality for u64 {
   fn is_prime(&self) -> bool {
     NumberTheory::is_prime(self) 
   }  
}

impl Primality for i64 {
   fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
         NumberTheory::is_prime(self)
   }
}

impl Primality for u128 {
  fn is_prime(&self) -> bool{
     NumberTheory::is_prime(self)
     }
}

impl Primality for i128 {
    fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
         NumberTheory::is_prime(self)
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
#[must_use]
pub fn generate_primes(limit: usize) -> Vec<usize> {
    match limit {
        0 | 1 => { return Vec::new(); }
        2     => { return vec![2]; }
        3     => { return vec![2, 3]; }
        _     => { }
    }

    let mut res: Vec<usize> = Vec::with_capacity(limit >> 2 + 2);
    res.push(2);
    res.push(3);
    let mut sieve: Vec<bool> = vec![false; limit];
    
    for x in (1..).take_while(|n| n.square() < limit) {
        let mut n: usize;
        for y in (1..).take_while(|n| n.square() < limit) {

            n = 4 * x.square() + y.square();
            if n <= limit && (n % 12 == 1 || n % 12 == 5)
            { sieve[n] ^= true; }

            n = 3 * x.square() + y.square();
            if n <= limit && n % 12 == 7
            { sieve[n] ^= true; }

            if x > y {
                n = 3 * x.square() - y.square();
                if n <= limit && n % 12 == 11
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
