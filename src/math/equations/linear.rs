//! Linear functions.

#[derive(Clone, Copy, Debug, PartialEq)]
/// The struct for storing linear functions of the form `f(x) = mx + c`.
pub struct LinearEquation {
    m:    f64,
    c:    f64,
    root: Option<f64>
}

impl LinearEquation {
    /// Create a new `LinearEquation`.
    /// # Arguments
    /// * `m: f64` - The slope of the function.
    /// * `c: f64` - The y-intercept of the function.
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
    pub fn new(m: f64, c: f64) -> LinearEquation {
        if m == 0.0
        { panic!("m cannot be zero."); }
        LinearEquation { m, c, root: None }
    }
    /// Get the slope of a `LinearEquation`.
    /// # Returns
    /// A `f64`.
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
    pub const fn m(&self) -> f64 {
        self.m
    }
    /// Get the y-intercept of a `LinearEquation`.
    /// # Returns
    /// A `f64`.
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
    pub const fn c(&self) -> f64 {
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
    pub fn set_m(&mut self, value: f64) {
        if value == 0.0
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
    pub fn set_c(&mut self, value: f64) {
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
    pub fn get_root(&mut self) -> f64 {
        if self.root.is_some()
        { return unsafe { self.root.unwrap_unchecked() }; }

        self.root = Some(- self.c / self.m);
        unsafe { self.root.unwrap_unchecked() }
    }
    /// Get the value of a value `x` under the function of the `LinearEquation`.
    /// # Returns
    /// A `f64`.
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
    pub fn get_fun_val_of<T>(&self, x: T) -> f64
    where f64: From<T> {
        self.m * f64::from(x) + self.c
    }
}

impl std::fmt::Display for LinearEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.c > 0.0 {
            true  => { write!(f, "f(x) = {}x + {}", self.m, self.c) }
            false => { write!(f, "f(x) = {}x - {}", self.m, - self.c) }
        }
    }
}