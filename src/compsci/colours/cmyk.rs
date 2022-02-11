//! In here you'll find the struct for the `CMYK` colour-definition.
use crate::math::general::IsInRange;
use crate::compsci::colours::rgb::RGB;

const ARG_ERR: &str = "Arguments must range from 0 to 1.";
/// A struct for storing CMYK-Values.
#[derive(Clone, Copy, PartialEq)]
pub struct CMYK {
    cyan:    f32,
    magenta: f32,
    yellow:  f32,
    black:   f32
}

impl CMYK {
    /// Create a new `CMYK` struct.
    /// # Arguments
    /// * `cyan: f32` - The cyan part.
    /// * `magenta: f32` - The magenta part.
    /// * `yellow: f32` - The yellow part.
    /// * `black: f32` - The black part.
    /// # Returns
    /// A new `CMYK` struct.
    /// # Panics
    /// Panics if one of the arguments is not in the interval `[0; 1]`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// 
    /// let nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(cyan:    f32,
               magenta: f32,
               yellow:  f32,
               black:   f32) -> CMYK {
        if !cyan.is_in_range(0.0, 1.0) ||
           !magenta.is_in_range(0.0, 1.0) ||
           !yellow.is_in_range(0.0, 1.0) ||
           !black.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        CMYK { cyan, magenta, yellow, black }
    }
    /// Create a new `CMYK` struct from an existing `RGB` struct.
    /// # Arguments
    /// * `rgb: &RGB` - The `RGB` struct.
    /// # Returns
    /// A new `CMYK` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgb::RGB};
    /// 
    /// let nice_blue_rgb = RGB::new(128, 191, 255);
    /// let nice_blue_cmyk = CMYK::new_from_rgb_vals(128, 191, 255);
    /// 
    /// assert_eq!(CMYK::new_from_rgb_struct(&nice_blue_rgb),
    ///            nice_blue_cmyk);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_rgb_struct(rgb: &RGB) -> CMYK {
        let r: f32 = rgb.red as f32 / 255.0;
        let g: f32 = rgb.green as f32 / 255.0;
        let b: f32 = rgb.blue as f32 / 255.0;

        let k: f32 = 1.0 - r.min(g).min(b);
        let c: f32 = (1.0 - r - k) / (1.0 - k);
        let m: f32 = (1.0 - g - k) / (1.0 - k);
        let y: f32 = (1.0 - b - k) / (1.0 - k);

        CMYK { cyan:    c,
               magenta: m,
               yellow:  y,
               black:   k }
    }
    /// Create a new `CMYK` struct from `RGB` values.
    /// # Arguments
    /// * `red: u8` - The red part.
    /// * `green: u8` - The green part.
    /// * `blue: u8` - The blue part.
    /// # Returns
    /// A new `CMYK` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgb::RGB};
    /// 
    /// let nice_blue_rgb = RGB::new(128, 191, 255);
    /// let nice_blue_cmyk = CMYK::new_from_rgb_vals(128, 191, 255);
    /// 
    /// assert_eq!(RGB::new_from_cmyk_struct(&nice_blue_cmyk),
    ///            nice_blue_rgb);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_rgb_vals(red: u8, green: u8, blue: u8) -> CMYK {
        CMYK::new_from_rgb_struct(&RGB::new(red, green, blue))
    }
    /// Get the reference to the `cyan` field.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.5, nice_blue.cyan());
    /// ```
    #[inline]
    #[must_use]
    pub fn cyan(&self) -> f32 {
        self.cyan
    }
    /// Get the reference to the `magenta` field.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.25, nice_blue.magenta());
    /// ```
    #[inline]
    #[must_use]
    pub fn magenta(&self) -> f32 {
        self.magenta
    }
    /// Get the reference to the `yellow` field.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.0, nice_blue.yellow());
    /// ```
    #[inline]
    #[must_use]
    pub fn yellow(&self) -> f32 {
        self.yellow
    }
    /// Get the reference to the `black` field.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.0, nice_blue.black());
    /// ```
    #[inline]
    #[must_use]
    pub fn black(&self) -> f32 {
        self.black
    }
    /// Set the `cyan` field.
    /// # Arguments
    /// * `value: f32` - The new value.
    /// # Panics
    /// Panics if `value` is not in the interval `[0; 1]`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let mut nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.5, nice_blue.cyan());
    /// nice_blue.set_cyan(1.0);
    /// assert_eq!(1.0, nice_blue.cyan());
    /// ```
    #[inline]
    #[must_use]
    pub fn set_cyan(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.cyan = value;
    }
    /// Set the `magenta` field.
    /// # Arguments
    /// * `value: f32` - The new value.
    /// # Panics
    /// Panics if `value` is not in the interval `[0; 1]`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let mut nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.25, nice_blue.magenta());
    /// nice_blue.set_magenta(1.0);
    /// assert_eq!(1.0, nice_blue.magenta());
    /// ```
    #[inline]
    #[must_use]
    pub fn set_magenta(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.magenta = value;
    }
    /// Set the `yellow` field.
    /// # Arguments
    /// * `value: f32` - The new value.
    /// # Panics
    /// Panics if `value` is not in the interval `[0; 1]`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let mut nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.0, nice_blue.yellow());
    /// nice_blue.set_yellow(0.5);
    /// assert_eq!(0.5, nice_blue.yellow());
    /// ```
    #[inline]
    #[must_use]
    pub fn set_yellow(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.yellow = value;
    }
    /// Set the `black` field.
    /// # Arguments
    /// * `value: f32` - The new value.
    /// # Panics
    /// Panics if `value` is not in the interval `[0; 1]`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::cmyk::CMYK;
    /// let mut nice_blue = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// 
    /// assert_eq!(0.0, nice_blue.black());
    /// nice_blue.set_black(0.5);
    /// assert_eq!(0.5, nice_blue.black());
    /// ```
    #[inline]
    #[must_use]
    pub fn set_black(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.black = value;
    }
}

impl std::fmt::Display for CMYK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}; {}; {}; {})", self.cyan(),
                                      self.magenta(),
                                      self.yellow(),
                                      self.black())
    }
}

impl std::fmt::Debug for CMYK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(C: {}; M: {}; Y: {}; K: {})", self.cyan(),
                                                  self.magenta(),
                                                  self.yellow(),
                                                  self.black())
    }
}
/// Iterate over a `CMYK` struct in the order `C, M, Y, K`.
impl IntoIterator for CMYK {
    type Item = f32;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.cyan(), self.magenta(), self.yellow(), self.black()].into_iter()
    }
}