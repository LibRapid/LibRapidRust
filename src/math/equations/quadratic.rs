//! Quadratic functions.
use crate::math::general::CommonPowers;
/// A struct for storing quadratic equations of the form `f(x) = ax² + bx + c`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticEquation {
    a:         f64,
    b:         f64,
    c:         f64,
    vertex:    Option<(f64, f64)>,
    solutions: Option<(Option<f64>, Option<f64>)>
}

impl QuadraticEquation {
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
    pub fn new() -> QuadraticEquation {
        QuadraticEquation { a:         1.0,
                            b:         0.0,
                            c:         0.0,
                            vertex:    Some((0.0, 0.0)),
                            solutions: Some((Some(0.0), None)) }
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
    pub fn new_from_coefficients(a: f64, b: f64, c: f64) -> QuadraticEquation {
        if a == 0.0
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
    pub fn a(&self) -> f64 {
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
    pub fn b(&self) -> f64 {
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
    pub fn c(&self) -> f64 {
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
    pub fn set_a(&mut self, value: f64) {
        if value == 0.0
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
    pub fn set_b(&mut self, value: f64) {
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
    pub fn set_c(&mut self, value: f64) {
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
    pub fn get_solutions(&mut self) -> Option<(Option<f64>, Option<f64>)> {
        if self.solutions.is_some()
        { return self.solutions; }
        let discriminant = self.b.square() - 4.0 * self.a * self.c;
        if discriminant < 0.0
        { return None; }
        let x_1 = (- self.b + discriminant.sqrt()) / (2.0 * self.a);
        let x_2 = (- self.b - discriminant.sqrt()) / (2.0 * self.a);
        
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
    pub fn get_vertex(&mut self) -> (f64, f64) {
        if self.vertex.is_some()
        { return unsafe { self.vertex.unwrap_unchecked() }; }
        let x = - self. b / (2.0 * self.a);
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
    pub fn get_fun_val_of<T>(&self, x: T) -> f64
    where f64: From<T> {
        let _x = f64::from(x);
        self.a * _x.square() + self.b * _x + self.c
    }
}

impl std::fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res;
        let _  = write!(f, "f(x) = {}x^2", self.a);
        if self.b < 0.0
        { let _ = write!(f, " - {}x", - self.b); }
        else
        { let _ = write!(f, " + {}x", self.b); }
        if self.c < 0.0
        { res = write!(f, " - {}", - self.c); }
        else
        { res = write!(f, " + {}", self.c); }
        res
    }
}