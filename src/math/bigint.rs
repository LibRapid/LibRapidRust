//! A custom BigInt library, kindly provided by Rust-CAS. Some functions are marked as unstable, as long as some functions do not function as the LibRapid developers wish.
use std::fmt::Debug;

use number_theory::{arithmetic::mpz::Mpz, NumberTheory};

/// The BigInt structure.

#[derive(Clone)]
pub struct BigInt {
    num: Mpz,
}

impl BigInt {
    /// Initialise a `BigInt` with the value `0`.
    /// # Returns
    /// A `BigInt`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::zero();
    /// assert_eq!("0".to_owned(), n.to_string());
    /// ```
    #[must_use]
    pub fn zero() -> Self {
        BigInt { num: Mpz::zero() }
    }
    /// Initialise a `BigInt` with the value `1`.
    /// # Returns
    /// A `BigInt`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::one();
    /// assert_eq!("1".to_owned(), n.to_string());
    /// ```
    #[must_use]
    pub fn one() -> Self {
        BigInt { num: Mpz::one() }
    }
    /// Create a `BigInt` from a String.
    /// # Arguments
    /// * `x` - `&str`
    /// # Returns
    /// A `BigInt`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::from_string("1010").unwrap();
    /// assert_eq!("1010".to_owned(), n.to_string());
    /// ```
    #[must_use]
    pub fn from_string(x: &str) -> Option<Self> {
        match Mpz::u_from_string(x) {
            Some(num) => Some(BigInt { num }),
            None => None,
        }
    }
    /// Stringify a BigInt.
    /// # Returns
    /// A `String`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::zero();
    /// assert_eq!("0".to_owned(), n.to_string());
    /// ```
    #[must_use]
    pub fn to_string(&self) -> String {
        self.num.to_string()
    }
    /// Take the square root of a BigInt.
    /// # Returns
    /// A `BigInt`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::from_string("4").unwrap();
    /// let x = BigInt::from_string("2").unwrap();
    /// assert_eq!(x, n.sqrt());
    /// ```
    #[cfg(feature="unstable")]
    #[must_use]
    pub fn sqrt(&self) -> Self {
        BigInt {
            num: self.num.sqrt(),
        }
    }
    /// Take the nth-root of a BigInt.
    /// # Returns
    /// A `BigInt`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::from_string("4").unwrap();
    /// let x = BigInt::from_string("2").unwrap();
    /// assert_eq!(x, n.nth_root(2u64));
    /// ```
    #[cfg(feature="unstable")]
    #[must_use]
    pub fn nth_root(&self, n: u64) -> Self {
        BigInt {
            num: self.num.nth_root(n),
        }
    }
    /// Calculate the power of a given number.
    /// # Arguments
    /// * `p` - `u64`, the power.
    /// # Returns
    /// A `BigInt`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::from_string("9223372036854775783").unwrap();
    /// assert_eq!(BigInt::from_string("32244539253885817035223818480471442665012\
    ///                                 48601532064792704105142277000275987293847\
    ///                                 84036165390028700825870551255560183375423\
    ///                                 01452395133796411813963734372582412890578\
    ///                                 50948908231948073400108413006511597583084\
    ///                                 69498465026385174471085878782790046343966\
    ///                                 54070888941076096529").unwrap(), n.pow(14));
    /// ```
    #[must_use]
    pub fn pow(&self, p: u64) -> Self {
        BigInt {
            num: self.num.pow(p),
        }
    }
    /// Calculate the logarithm base `n` of a number.
    /// # Arguments
    /// * `n` - `f64` The logarithm base.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let mut n = BigInt::from_string("1024").unwrap();
    /// assert_eq!(10.0, n.log(2.0));
    /// n = BigInt::from_string("2000").unwrap();
    /// assert_eq!(3.9060911744728372, n.log(7.0));
    /// ```
    #[cfg(feature="unstable")]
    #[must_use]
    pub fn log(&self, log: f64) -> f64 {
        self.num.log(log)
    }
    #[cfg(feature="unstable")]
    #[must_use]
    pub fn factorial(n: u64) -> Self {
        BigInt {
            num: Mpz::sirp(1, n, 1, 0),
        }
    }
    /// Determines if a given number is prime.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::bigint::BigInt;
    /// 
    /// let n = BigInt::from_string("9223372036854775783").unwrap();
    /// assert_eq!(true, n.is_prime());
    /// ```
    #[must_use]
    pub fn is_prime(&self) -> bool {
        self.num.probable_prime()
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BigInt {
            num: self.num.addition(other.num),
        }
    }
}

impl std::ops::Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        BigInt {
            num: self.num.subtraction(other.num),
        }
    }
}

impl std::ops::Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        BigInt {
            num: self.num.product(other.num),
        }
    }
}

impl std::ops::Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        BigInt {
            num: self.num.euclidean(&other.num).0,
        }
    }
}

impl std::ops::Rem for BigInt {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        BigInt {
            num: self.num.euclidean(&other.num).1,
        }
    }
}

impl std::cmp::PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BigInt").field("num", &self.num).finish()
    }
}