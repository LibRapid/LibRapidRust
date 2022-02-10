//! Traits and functions for handling primes and primality.

// *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
// | Original implementations by Forisek, Jancina & J.A. Sory (rust-cas.org). |
// | Further modified and optimised by NervousNullPtr.                        |
// *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*

use super::general::*;
use crate::math::constants;
/// Trait for prime functions.
pub trait Primality {
    /// Determines whether a number is prime.
    /// # Returns
    /// `true` if the number is prime, otherwise `false`.
    /// # Warning
    /// There is currently no implementation for `u128`/`i128`, because we want our own BigInt-implementation.
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
    /// assert_eq!(true, 18446744073709551557u64.is_prime());
    /// ```
    #[must_use]
    fn is_prime(&self) -> bool;
}

impl Primality for u8 {
    #[inline]
    fn is_prime(&self) -> bool {
        if self < &2
        { return false; }

        for i in constants::PRIMELIST[..6].iter() {
            if *self == *i as u8
            { return true; }
            if self % *i as u8 == 0
            { return false; }
        }
        true
    }
}

impl Primality for u16 {// Splits 
   fn is_prime(&self) -> bool {
        if *self <= u8::MAX as u16
        { return (*self as u8).is_prime(); }

        constants::PRIMELIST.binary_search(self).is_ok()
    }
}

impl Primality for u32 { 
    fn is_prime(&self) -> bool {
        if *self <= u16::MAX as u32
        { return (*self as u16).is_prime(); }
          
            for i in constants::PRIMELIST[..30].iter() {
                if self % *i as u32 == 0
                { return false; }
            }
        machine_word_prime_32(*self)  
    }
}

impl Primality for i32 {
    fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
        (*self as u32).is_prime()
    }
}

impl Primality for u64 {
   fn is_prime(&self) -> bool {
        if *self <= u32::MAX as u64
        { return (*self as u32).is_prime(); }

        for i in constants::PRIMELIST[..30].iter() {
            if self % *i as u64 == 0
            { return false; }
        }

        machine_word_prime_64(*self)  
   }
}

impl Primality for i64 {
   fn is_prime(&self) -> bool {
        if self <= &0 { return false; }
        (*self as u64).is_prime()
    }
}

impl Primality for u128 {
    fn is_prime(&self) -> bool {
        if *self <= u64::MAX as u128
        { return (*self as u64).is_prime(); }
        unimplemented!("is_prime is currently not implemented for u128/i128 values bigger than 2^64 - 1.")
    }
}

impl Primality for i128 {
    fn is_prime(&self) -> bool {
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
    let mut n: usize;
    let mut sieve: Vec<bool> = vec![false; limit];
    
    for x in (1..).take_while(|n| n.square() < limit) {
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

    for (i, val) in sieve.iter().enumerate() {
        if *val { res.push(i); }
    }
    res
}
#[must_use]
fn machine_word_prime_32(n: u32) -> bool {
    let mut x: u64 = (((n >> 16) ^ n) as u128 * 0x45d9f3b) as u64;
            x      = (((x >> 16) ^ x) as u128 * 0x45d9f3b) as u64;
            x      =  ((x >> 16) ^ x) & 255;

    sprp_32(n, constants::BASES_32[x as usize] as u32)
}
#[must_use]
fn machine_word_prime_64(x: u64) -> bool {

    if !sprp_64(x, 2)
    { return false; }

    let mut h = x;
    h         = (((h >> 32) ^ h) as u128 * 0x45d9f3b3335b369) as u64;
    h         = (((h >> 32) ^ h) as u128 * 0x3335b36945d9f3b) as u64;
    h         =   (h >> 32) ^ h;
    
    let b: u32 = constants::BASES_64[(h & 16383) as usize];
    sprp_64(x, (b & 4095) as u64) && sprp_64(x, (b >> 12) as u64)
}
#[must_use]
fn sprp_32(p: u32, base: u32) -> bool { // if base^p = 1 mod p || base^(d * 2^n)= -1
    let zeroes: u32 = (p - 1).trailing_zeros(); // d * 2^n -1
    let d:      u32 = (p - 1) >> zeroes;
    let mut x:  u32 = mod_pow_32(&base, &d, &p); // base^d mod p

    if x == 1 || x == p - 1 // base^p = 1 mod p || base^(d * 2^n)= -1
    { return true; }

    for _ in 0..zeroes - 1 { // checks for all d * 2^zeroes.
        x = ((x as u64).square() % p as u64) as u32;
        if x == p - 1
        { return true; }
    }
    false
}
#[must_use]
fn sprp_64(p: u64, base: u64) -> bool {
    let zeroes: u32 = (p - 1).trailing_zeros();
    let d:      u64 = (p - 1) >> zeroes;
    let mut x:  u64 = mod_pow_64(&base, &d, &p);

    if x == 1 || x == p - 1
    { return true; }

    for _ in 0..zeroes - 1 {
        x = (((x as u128).square()) % p as u128) as u64;
        if x == p - 1
        { return true; }
    }
    false
}
#[must_use]
fn mod_pow_64(c: &u64, p: &u64, modulus: &u64) -> u64 {  
    if modulus == &0
    { return *modulus; }
    if p == &0 
    { return 1; }
    let mut z:    u128 = 1;
    let mut base: u128 = *c as u128;
    let n:        u128 = *modulus as u128;
    let mut pow:  u64  = *p;

    while pow > 1 {

        if pow & 1 == 0 {
            base = base.square() % n;
            pow  >>= 1;
            continue;
        }
        z    = base * z % n;
        base = base.square() % n;
        pow  = (pow - 1) >> 1;
    }
    (base * z % n) as u64
}
#[must_use]
fn mod_pow_32(c: &u32, p: &u32, modulus: &u32) -> u32 {  
    if modulus == &0
    { return 0; }
    if p == &0 
    { return 1; }
    let mut z:    u64 = 1;
    let mut base: u64 = *c as u64;
    let n:        u64 = *modulus as u64;
    let mut pow:  u32 = *p;

    while pow > 1 {

        if pow & 1 == 0 {
            base = base.square() % n;
            pow  >>= 1;
            continue;
        }
        z = base * z % n;
        base = base.square() % n;
        pow = (pow - 1) >> 1;
    }
    (base * z % n) as u32
}
