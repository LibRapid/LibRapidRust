use std::fmt::Display;
use std::ops::*;
use std::cmp::*;

/// The real number `1usize` as a complex number.
pub const REAL_ONE: ComplexNumber<usize> = ComplexNumber { real: 1, complex: 0 };

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
    #[inline]
    #[must_use]
    pub fn new(real: T, complex: T) -> ComplexNumber<T> {
        ComplexNumber { real,
                        complex }
    }
    #[inline]
    #[must_use]
    pub fn new_unitc() -> ComplexNumber<T> {
        ComplexNumber { real:    T::from(0),
                        complex: T::from(1), }
    }
    #[inline]
    #[must_use]
    pub fn recip(&self) -> ComplexNumber<T> {
        ComplexNumber {real: self.real /
                             (self.real * self.real + self.complex * self.complex),
                       complex: - (self.complex /
                                  (self.real * self.real + self.complex * self.complex)) }
    }
    #[inline]
    #[must_use]
    pub fn complex_conjugate(&self) -> ComplexNumber<T> {
        ComplexNumber { real: self.real, complex: - self.complex }
    }
    #[inline]
    #[must_use]
    pub fn abs_f64(&self) -> f64 {
        (f64::from(self.real.square()) + f64::from(self.complex.square())).sqrt()
    }
    #[inline]
    #[must_use]
    pub fn abs_f32(&self) -> f32 {
        self.abs_f64() as f32
    }
    #[inline]
    #[must_use]
    pub fn powi(&self, pow: isize) -> ComplexNumber<T> {
        if pow == 0
        { return ComplexNumber::new(T::from(1), T::from(0)); }

        match pow > 0 {
            true  => { let mut res = *self; 
                       (0..pow).for_each(|_| res *= *self);
                       res
            }
            false => { return ComplexNumber::new(T::from(1), T::from(0)) / self.powi(-pow); }
        }
    }
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
        self.real    = (self.real * rhs.real    + self.complex * rhs.complex) /
                       (rhs.real.square()       + rhs.complex.square());
        self.complex = (self.complex * rhs.real - self.real * rhs.complex) /
                       (rhs.real.square()       + rhs.complex.square());
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Copy> MulAssign for ComplexNumber<T> {
    #[inline]
    #[must_use]
    fn mul_assign(&mut self, rhs: Self) {
        self.real    = self.real * rhs.real    - self.complex * rhs.complex;
        self.complex = self.real * rhs.complex + self.complex * rhs.real;
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