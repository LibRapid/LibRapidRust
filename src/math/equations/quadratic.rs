//! Quadratic functions.
use std::fmt::Display;

use crate::math::general::CommonPowers;
/// A struct for storing quadratic equations of the form `f(x) = ax² + bx + c`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticEquation<T> {
    a:         T,
    b:         T,
    c:         T,
    vertex:    Option<(T, T)>,
    solutions: Option<(Option<T>, Option<T>)>
}

impl<T: Copy +
        Clone +
        From<u8> +
        From<f64> +
        PartialEq +
        PartialOrd +
        CommonPowers +
        std::ops::Mul<Output = T> +
        std::ops::Add<Output = T> +
        std::ops::Sub<Output = T> +
        std::ops::Div<Output = T> +
        std::ops::Neg<Output = T>> QuadraticEquation<T>
        where
        f64: From<T> {
    /// Create a new `QuadraticEquation` with the values `a = 1, b = 0, c = 0`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, 0.0);
    /// f_x.get_vertex();
    /// f_x.get_solutions();
    /// 
    /// assert_eq!(QuadraticEquation::new(), f_x);
    /// ```
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, -1.5);
    /// 
    /// assert_eq!("f(x) = 1x^2 + 0x - 1.5", &f_x.to_string());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> QuadraticEquation<T> {
        QuadraticEquation { a:         T::from(1),
                            b:         T::from(0),
                            c:         T::from(0),
                            vertex:    Some((T::from(0), T::from(0))),
                            solutions: Some((Some(T::from(0)), None)) }
    }
    /// Create a new `QuadraticEquation` from coefficients.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, 0.0);
    /// f_x.get_vertex();
    /// f_x.get_solutions();
    /// 
    /// assert_eq!(QuadraticEquation::new(), f_x);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_coefficients(a: T, b: T, c: T) -> QuadraticEquation<T> {
        if a == T::from(0)
        { panic!("a was zero and is thus not allowed."); }
        QuadraticEquation { a,
                            b,
                            c,
                            vertex:    None,
                            solutions: None }
    }
    /// Get `a` of a `QuadraticEquation`.
    /// # Returns
    /// A `f64.`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-2.0, f_x.b());
    /// ```
    #[inline]
    #[must_use]
    pub fn a(&self) -> T {
        self.a
    }
    /// Get `b` of a `QuadraticEquation`.
    /// # Returns
    /// A `f64.`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-2.0, f_x.b());
    /// ```
    #[inline]
    #[must_use]
    pub fn b(&self) -> T {
        self.b
    }
    /// Get `c` of a `QuadraticEquation`.
    /// # Returns
    /// A `f64.`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-3.0, f_x.c());
    /// ```
    #[inline]
    #[must_use]
    pub fn c(&self) -> T {
        self.c 
    }
    /// Set `c` of a `QuadraticEquation`.
    /// # Panics
    /// Panics if `value` is zero.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(1.0, f_x.a());
    /// 
    /// f_x.set_a(-1.0);
    /// 
    /// assert_eq!(-1.0, f_x.a());
    /// ```
    #[inline]
    pub fn set_a(&mut self, value: T) {
        if value == T::from(0)
        { panic!("a was zero and is thus not allowed."); }
        self.a = value;
    }
    /// Set `b` of a `QuadraticEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-2.0, f_x.b());
    /// 
    /// f_x.set_b(-1.0);
    /// 
    /// assert_eq!(-1.0, f_x.b());
    /// ```
    #[inline] 
    pub fn set_b(&mut self, value: T) {
        self.b = value;
    }
    /// Set `c` of a `QuadraticEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-3.0, f_x.c());
    /// 
    /// f_x.set_c(-1.0);
    /// 
    /// assert_eq!(-1.0, f_x.c());
    /// ```
    #[inline]
    pub fn set_c(&mut self, value: T) {
        self.c = value;
    }
    /// Get the solutions of a quadratic equation.
    /// # Returns
    /// A `Option<(Option<f64>, Option<f64>)>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(Some((Some(3.0), Some(-1.0))), f_x.get_solutions());
    /// ```
    pub fn get_solutions(&mut self) -> Option<(Option<T>, Option<T>)> {
        if self.solutions.is_some()
        { return self.solutions; }
        let discriminant = self.b.square() - T::from(4) * self.a * self.c;
        if discriminant < T::from(0)
        { return None; }
        let x_1 = (- self.b + T::from(f64::from(discriminant).sqrt())) / (T::from(2) * self.a);
        let x_2 = (- self.b - T::from(f64::from(discriminant).sqrt())) / (T::from(2) * self.a);
        
        match x_1 == x_2 {
            true  => { self.solutions = Some((Some(x_1), None)); }
            false => { self.solutions = Some((Some(x_1), Some(x_2))); }
        }
        self.solutions
    }
    /// Get the vertex of a quadratic equation.
    /// # Returns
    /// A `(f64, f64)`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, 3.0);
    /// 
    /// assert_eq!((1.0, 2.0), f_x.get_vertex());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_vertex(&mut self) -> (T, T) {
        if self.vertex.is_some()
        { return unsafe { self.vertex.unwrap_unchecked() }; }
        let x = - self. b / (T::from(2) * self.a);
        self.vertex = Some((x, self.a * x.square() + self.b * x + self.c));
        unsafe { self.vertex.unwrap_unchecked() }
    }
    /// Get the value of a value `x` under the function of the `QuadraticEquation`.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, 3.0);
    /// 
    /// assert_eq!(2.0, f_x.get_fun_val_of(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_fun_val_of<S: CommonPowers>(&self, x: S) -> T
    where f64: From<T>,
          T: From<S> {
        let _x = T::from(x);
        self.a * _x.square() + self.b * _x + self.c
    }
}

impl<T: Display +
        std::ops::Neg<Output = T> +
        PartialOrd +
        From<u8> +
        Copy> std::fmt::Display for QuadraticEquation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res;
        let _  = write!(f, "f(x) = {}x^2", self.a);
        if self.b < T::from(0)
        { let _ = write!(f, " - {}x", - self.b); }
        else
        { let _ = write!(f, " + {}x", self.b); }
        if self.c < T::from(0)
        { res = write!(f, " - {}", - self.c); }
        else
        { res = write!(f, " + {}", self.c); }
        res
    }
}