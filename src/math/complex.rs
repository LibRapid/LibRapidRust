//! Complex numbers and related functions.
use std::fmt::Display;
use std::ops::*;
use std::cmp::*;

/// The real number `1usize` as a complex number.
pub const REAL_ONE: ComplexNumber<usize> = ComplexNumber { real: 1, complex: 0 };
/// The generic Complex number type.
/// # Examples
/// ```
/// use lib_rapid::math::complex::ComplexNumber;
/// use lib_rapid::math::general::delta;
/// 
/// let mut c1 = ComplexNumber::new(4.0, 2.8);
/// let mut c2 = ComplexNumber::new(2.0, 3.8);
/// let mut complex = c1.clone();
/// let mut expected;
/// 
/// // Addition
/// expected = ComplexNumber::new(6.0, 6.6);
/// assert_eq!(c1 + c2, expected);
/// complex += c2;
/// assert_eq!(complex, expected);
/// 
/// // Subtraction
/// expected = ComplexNumber::new(-2.0, 1.0);
/// complex = c2.clone();
/// complex -= c1;
/// assert_eq!(c2 - c1, expected);
/// assert_eq!(complex, expected);
/// 
/// // Multiplication
/// expected = ComplexNumber::new(-2.64, 20.8);
/// complex = c1.clone();
/// complex *= c2;
/// assert!( delta( (c1 * c2).real,    expected.real    ) < 1e-10);
/// assert!( delta( (c1 * c2).complex, expected.complex ) < 1e-10);
/// assert!( delta( complex.real,      expected.real    ) < 1e-10);
/// assert!( delta( complex.complex,   expected.complex ) < 1e-10);
/// 
/// // Multiplication
/// c1 = ComplexNumber::new(2.0, 2.0);
/// c2 = ComplexNumber::new(4.0, 3.0);
/// expected = ComplexNumber::new(0.56, 0.08);
/// complex = c1.clone();
/// complex /= c2;
/// assert!( delta( (c1 / c2).real,    expected.real    ) < 1e-10);
/// assert!( delta( (c1 / c2).complex, expected.complex ) < 1e-10);
/// assert!( delta( complex.real,      expected.real    ) < 1e-10);
/// assert!( delta( complex.complex,   expected.complex ) < 1e-10);
/// 
/// // Convert from real to complex
/// assert_eq!(ComplexNumber::from(5.0), ComplexNumber::new(5.0, 0.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComplexNumber<T> {
    pub real:    T,
    pub complex: T
}

impl<T: Neg<Output = T> +
        Div<Output = T> +
        Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Copy +
        super::general::NumTools<T> +
        Add<Output = T> +
        From<u8>> ComplexNumber<T>
        where
        f64: From<T> {
    /// Create a new Complex number of the form `a + bi` with `a,b ∈ ℝ`.
    /// # Arguments
    /// * `real: T` - The real part of a complex number.
    /// * `complex: T` - The real part of a complex number.
    /// # Returns
    /// A `ComplexNumber<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// 
    /// let c = ComplexNumber::new(2, -4);
    /// assert_eq!(c.to_string(), "2 - 4i".to_owned());
    /// ```
    #[inline]
    #[must_use]
    pub fn new(real: T, complex: T) -> ComplexNumber<T> {
        ComplexNumber { real,
                        complex }
    }
    /// Create a new Complex number with the values `0 + 1i`.
    /// # Returns
    /// A `ComplexNumber<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// 
    /// let c = ComplexNumber::new(0, 1);
    /// assert_eq!(c, ComplexNumber::new_unitc());
    /// ```
    #[inline]
    #[must_use]
    pub fn new_unitc() -> ComplexNumber<T> {
        ComplexNumber { real:    T::from(0),
                        complex: T::from(1), }
    }
    /// Calculate the reciprocal of a complex number.
    /// # Returns
    /// A `ComplexNumber<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// 
    /// let c = ComplexNumber::new(2.0, -4.0);
    /// assert_eq!(c.recip(), ComplexNumber::new(0.1, 0.2));
    /// ```
    #[inline]
    #[must_use]
    pub fn recip(&self) -> ComplexNumber<T> {
        ComplexNumber {real:       self.real /
                                  (self.real.square() + self.complex.square()),
                       complex: - (self.complex /
                                  (self.real.square() + self.complex.square())) }
    }
    /// Calculate the complex conjugage of a complex number.
    /// # Returns
    /// A `ComplexNumber<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// 
    /// let c = ComplexNumber::new(2, -4);
    /// assert_eq!(c.complex_conjugate(), ComplexNumber::new(2, 4));
    /// ```
    #[inline]
    #[must_use]
    pub fn complex_conjugate(&self) -> ComplexNumber<T> {
        ComplexNumber { real: self.real, complex: - self.complex }
    }
    /// Calculate the absolute value of a complex number.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// use lib_rapid::math::constants::SQRT5;
    /// 
    /// let c = ComplexNumber::new(2, -4);
    /// assert!((2.0 * SQRT5 - c.abs_f64()).abs() < 1e-10);
    /// ```
    #[inline]
    #[must_use]
    pub fn abs_f64(&self) -> f64 {
        (f64::from(self.real.square()) + f64::from(self.complex.square())).sqrt()
    }
    /// Calculate the absolute value of a complex number.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// use lib_rapid::math::constants::SQRT5;
    /// 
    /// let c = ComplexNumber::new(2, -4);
    /// assert!((2.0 * SQRT5 as f32 - c.abs_f32()).abs() < 1e-10);
    /// ```
    #[inline]
    #[must_use]
    pub fn abs_f32(&self) -> f32 {
        self.abs_f64() as f32
    }
    /// Raise `self` to a whole number power `pow`.
    /// # Arguments
    /// * `pow: isize` - the power.
    /// # Returns
    /// A `ComplexNumber<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// 
    /// let c = ComplexNumber::new(2, -4);
    /// assert_eq!(c.powi(2), ComplexNumber::new(-12, -16));
    /// ```
    #[inline]
    #[must_use]
    pub fn powi(&self, pow: isize) -> ComplexNumber<T> {
        if pow == 0
        { return ComplexNumber::new(T::from(1), T::from(0)); }

        match pow > 0 {
            true  => { let mut res = *self; 
                       (0..pow-1).for_each(|_| res *= *self);
                       res
            }
            false => { return ComplexNumber::new(T::from(1), T::from(0)) / self.powi(-pow); }
        }
    }
    /// Raise `e` to a imaginary number `self`.
    /// # Returns
    /// A `ComplexNumber<f64>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::complex::ComplexNumber;
    /// use lib_rapid::math::constants::{PI, E};
    /// 
    /// let c = ComplexNumber::new(PI, PI);
    /// assert!((c.exp_f64().real + E.powf(PI)).abs() < 1e-10);
    /// ```
    #[inline]
    #[must_use]
    pub fn exp_f64(&self) -> ComplexNumber<f64> {
        let ea = super::constants::E.powf(f64::from(self.real));

        ComplexNumber { real:    ea * f64::cos(f64::from(self.complex)),
                        complex: ea * f64::sin(f64::from(self.complex)) }
    }
}

impl<T: Add<Output = T>> Add for ComplexNumber<T> {
    type Output = ComplexNumber<T>;
    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber { real:    self.real    + rhs.real,
                        complex: self.complex + rhs.complex }
    }
}

impl<T: Sub<Output = T>> Sub for ComplexNumber<T> {
    type Output = ComplexNumber<T>;
    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        ComplexNumber { real:    self.real    - rhs.real,
                        complex: self.complex - rhs.complex }
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Copy> Mul for ComplexNumber<T> {
    type Output = ComplexNumber<T>;
    #[inline]
    #[must_use]
    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber { real:    self.real * rhs.real    - self.complex * rhs.complex,
                        complex: self.real * rhs.complex + self.complex * rhs.real }
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Div<Output = T> +
        super::general::NumTools<T> +
        Copy> Div for ComplexNumber<T> {
    type Output = ComplexNumber<T>;
    #[inline]
    #[must_use]
    fn div(self, rhs: Self) -> Self::Output {
        ComplexNumber { real:    (self.real * rhs.real    + self.complex * rhs.complex) /
                                 (rhs.real.square()       + rhs.complex.square()),
                        complex: (self.complex * rhs.real - self.real * rhs.complex) /
                                 (rhs.real.square()       + rhs.complex.square()) }
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Div<Output = T> +
        super::general::NumTools<T> +
        Copy> DivAssign for ComplexNumber<T> {
    #[inline]
    #[must_use]
    fn div_assign(&mut self, rhs: Self) {
        let _res = *self / rhs;
        self.real    = _res.real;
        self.complex = _res.complex;
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Copy> MulAssign for ComplexNumber<T> {
    #[inline]
    #[must_use]
    fn mul_assign(&mut self, rhs: Self) {
        let _res = *self * rhs;
        self.real    = _res.real;
        self.complex = _res.complex;
    }
}

impl<T: Add<Output = T> +
        Copy> AddAssign for ComplexNumber<T> {
    #[inline]
    #[must_use]
    fn add_assign(&mut self, rhs: Self) {
        self.real    = self.real    + rhs.real;
        self.complex = self.complex + rhs.complex;
    }
}

impl<T: Sub<Output = T> +
        Copy> SubAssign for ComplexNumber<T> {
    #[inline]
    #[must_use]
    fn sub_assign(&mut self, rhs: Self) {
        self.real    = self.real    - rhs.real;
        self.complex = self.complex - rhs.complex;
    }
}
impl<T: Display +
        Neg<Output = T> +
        From<u8> +
        PartialOrd +
        Copy> Display for ComplexNumber<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(10);
        res.push_str(&format!("{} ", self.real));
        if self.complex < T::from(0)
        { res.push_str(&format!("- {}i", - self.complex)); }
        else
        { res.push_str(&format!("+ {}i", self.complex)); }
        write!(f, "{}", res)
    }
}

impl<T: From<u8>> std::convert::From<T> for ComplexNumber<T> {
    fn from(_a: T) -> Self {
        ComplexNumber { real: _a, complex: T::from(0) }
    }
}

/// This PartialOrd implementation uses Lexicographical ordering. This means:
/// * Ordered by real part if a₁ ≠ a₂.
/// * Ordered by imaginary part if a₁ = a₂.
/// ```
/// use lib_rapid::math::complex::ComplexNumber;
/// 
/// assert!(ComplexNumber::new(2, 3) < ComplexNumber::new(3, 4));
/// assert!(ComplexNumber::new(2, 1) > ComplexNumber::new(2, 0));
/// ```
impl<T: PartialEq +
        PartialOrd> PartialOrd for ComplexNumber<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.real.partial_cmp(&other.real) {
            Some(core::cmp::Ordering::Equal) =>
            { return self.complex.partial_cmp(&other.complex); }
            ord => return ord,
        }
    }
}