//! In here you'll find the struct for the classic `RGB` colour-definition.
use crate::compsci::colours::cmyk::CMYK;
/// A struct for storing RGB-Values.
#[derive(Clone, Copy, PartialEq)]
pub struct RGB {
    /// The red part of a `RGB` colour.
    pub red:   u8,
    /// The green part of a `RGB` colour.
    pub green: u8,
    /// The blue part of a `RGB` colour.
    pub blue:  u8
}

impl RGB {
    /// Create a new `RGB` struct.
    /// # Arguments
    /// * `red: u8` - The red part.
    /// * `green: u8` - The green part.
    /// * `blue: u8` - THe blue part.
    /// # Returns
    /// A new `RGB` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::rgb::RGB;
    /// 
    /// let nice_blue = RGB::new(128, 191, 255);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB { red, green, blue }
    }
    /// Create a new `RGB` struct.
    /// # Arguments
    /// * `cmyk: &CMYK` - The CMYK struct.
    /// # Returns
    /// A new `RGB` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgb::RGB};
    /// 
    /// let nice_blue_cmyk = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// let nice_blue_rgb = RGB::new(128, 191, 255);
    /// 
    /// assert_eq!(RGB::new_from_cmyk_struct(&nice_blue_cmyk),
    ///            nice_blue_rgb);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_cmyk_struct(cmyk: &CMYK) -> RGB {
        RGB { red:   (255.0 * (1.0 - cmyk.cyan()) * (1.0 - cmyk.black())).round() as u8,
              green: (255.0 * (1.0 - cmyk.magenta()) * (1.0 - cmyk.black())).round() as u8,
              blue:  (255.0 * (1.0 - cmyk.yellow()) * (1.0 - cmyk.black())).round() as u8 }
    }
    /// Create a new `RGB` struct from `CMYK` values.
    /// # Arguments
    /// * `cmyk: &CMYK` - The CMYK struct.
    /// # Returns
    /// A new `RGB` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::rgb::RGB;
    /// 
    /// let nice_blue_rgb = RGB::new(128, 191, 255);
    /// 
    /// assert_eq!(RGB::new_from_cmyk_vals(0.5, 0.25, 0.0, 0.0),
    ///            nice_blue_rgb);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_cmyk_vals(cyan:    f32,
                              magenta: f32,
                              yellow:  f32,
                              black:   f32) -> RGB {
        RGB::new_from_cmyk_struct(&CMYK::new(cyan, magenta, yellow, black))
    }
}

impl std::fmt::Debug for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Red: {}; Blue: {}; Green: {}", self.red, self.green, self.blue)
    }
}

impl std::fmt::Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
/// Iterate over a `RGB` struct in the order `R, G, B`.
impl IntoIterator for RGB {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.red, self.green, self.blue].into_iter()
    }
}