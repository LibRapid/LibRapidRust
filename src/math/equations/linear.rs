//! Linear functions.

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
/// The struct for storing linear functions of the form `f(x) = mx + c`.
pub struct LinearEquation<T> {
    m:    T,
    c:    T,
    root: Option<T>
}

impl<T: Copy +
        Clone +
        From<u8> +
        std::ops::Div<Output = T> +
        std::ops::Sub<Output = T> +
        std::ops::Neg<Output = T> +
        std::ops::Mul<Output = T> +
        std::ops::Add<Output = T> +
        std::cmp::PartialEq> LinearEquation<T> {
    /// Create a new `LinearEquation`.
    /// # Arguments
    /// * `m: T` - The slope of the function.
    /// * `c: T` - The y-intercept of the function.
    /// # Returns
    /// A new `LinearEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!("f(x) = 1.5x - 2.2".to_owned(), f_x.to_string());
    /// ```
    #[inline]
    #[must_use]
    pub fn new(m: T, c: T) -> LinearEquation<T> {
        if m == T::from(0u8)
        { panic!("m cannot be zero."); }
        LinearEquation { m, c, root: None }
    }
    /// Get the slope of a `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(1.5, f_x.m());
    /// ```
    #[inline]
    #[must_use]
    pub fn m(&self) -> T {
        self.m
    }
    /// Get the y-intercept of a `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(-2.2, f_x.c());
    /// ```
    #[inline]
    #[must_use]
    pub fn c(&self) -> T {
        self.c
    }
    /// set the slope of a `LinearEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(1.5, f_x.m());
    /// 
    /// f_x.set_m(1.1);
    /// 
    /// assert_eq!(1.1, f_x.m());
    /// ```
    #[inline]
    pub fn set_m(&mut self, value: T) {
        if value == T::from(0)
        { panic!("m cannot be zero."); }
        self.m = value
    }
    /// set the y-intercept of a `LinearEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(-2.2, f_x.c());
    /// 
    /// f_x.set_c(3.14);
    /// 
    /// assert_eq!(3.14, f_x.c());
    /// ```
    #[inline]
    pub fn set_c(&mut self, value: T) {
        self.c = value;
    }
    /// Get the x-coordinate of the root of a `LinearEquation`.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.0, -2.0);
    /// 
    /// assert_eq!(2.0, f_x.get_root());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_root(&mut self) -> T {
        if self.root.is_some()
        { return unsafe { self.root.unwrap_unchecked() }; }

        self.root = Some(- self.c / self.m);
        unsafe { self.root.unwrap_unchecked() }
    }
    /// Get the value of a value `x` under the function of the `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.0, -2.0);
    /// 
    /// assert_eq!(-1.0, f_x.get_fun_val_of(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_fun_val_of<S>(&self, x: S) -> T
    where T: From<S> {
        self.m * T::from(x) + self.c
    }
    /// Get the interception point between `self` and `other`.
    /// # Arguments
    /// * `self`.
    /// * `other: &LinearEquation`.
    /// # Returns
    /// A `(T, T)` tuple.
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.5, -2.0);
    /// let mut g_x = LinearEquation::new(-2.0, 5.0);
    /// 
    /// assert_eq!((2.0, 1.0), f_x.intcept_point_with(&g_x));
    #[inline]
    #[must_use]
    pub fn intcept_point_with(&self, other: &LinearEquation<T>) -> (T, T) {
            let x = (other.c - self.c) / (self.m - other.m);
            let y = self.m * x + self.c;
            (x, y)
    }
}

impl<T: From<u8> +
        Display +
        std::ops::Neg<Output = T> +
        PartialOrd +
        Copy> std::fmt::Display for LinearEquation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.c > T::from(0) {
            true  => { write!(f, "f(x) = {}x + {}", self.m, self.c) }
            false => { write!(f, "f(x) = {}x - {}", self.m, - self.c) }
        }
    }
}