//! In here you'll find the struct for the classic `RGB` colour-definition and the extended `RGBa`-definition.
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
    /// * `blue: u8` - The blue part.
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
    /// Copies the values from `self` into a new `RGBa` with `alpha = 255`.
    /// # Returns
    /// A new `RGB`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::rgb::*;
    /// 
    /// let rgb  = RGB::SOLID_WHITE;
    /// let rgba = RGBa::SOLID_WHITE;
    /// 
    /// assert_eq!(rgba, rgb.as_rgba());
    /// ```
    #[inline]
    #[must_use]
    pub fn as_rgba(&self) -> RGBa {
        RGBa {
            red:   self.red,
            green: self.green,
            blue:  self.blue,
            alpha: 255
        }
    }
    /// The pure colour red, Hex-Code `FF0000`.
    pub const SOLID_RED: RGB = RGB { red: 255, green: 0, blue: 0};
    /// The pure colour green, Hex-Code `00FF00`.
    pub const SOLID_GREEN: RGB = RGB { red: 0, green: 255, blue: 0};
    /// The pure colour blue, Hex-Code `0000FF`.
    pub const SOLID_BLUE: RGB = RGB { red: 0, green: 0, blue: 255};
    /// The pure colour white, Hex-Code `FFFFFF`.
    pub const SOLID_WHITE: RGB = RGB { red: 255, green: 255, blue: 255};
    /// The pure colour black, Hex-Code `000000`.
    pub const SOLID_BLACK: RGB = RGB { red: 0, green: 0, blue: 0};
    /// LibRapid Blue, Hex-Code `9074FF`.
    pub const LIBRAPID_SOLID_BLUE: RGB = RGB { red: 144, green: 116, blue: 255};
    /// LibRapid Red, Hex-Code `FF3600`.
    pub const LIBRAPID_SOLID_RED: RGB = RGB { red: 255, green: 54, blue: 0};
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

// --------------------------------------------------------------------------
// |------------------------ END RGB ---- BEGIN RGBa -----------------------|
// --------------------------------------------------------------------------

/// A struct for storing RGBa-Values.
#[derive(Clone, Copy, PartialEq)]
pub struct RGBa {
    /// The red part of a `RGBa` colour.
    pub red:   u8,
    /// The green part of a `RGBa` colour.
    pub green: u8,
    /// The blue part of a `RGBa` colour.
    pub blue:  u8,
    /// The alpha part of a `RGBa` colour.
    pub alpha: u8
}

impl RGBa {
    /// Create a new `RGBa` struct.
    /// # Arguments
    /// * `red: u8` - The red part.
    /// * `green: u8` - The green part.
    /// * `blue: u8` - The blue part.
    /// * `alpha: u8` - The alpha part.
    /// # Returns
    /// A new `RGBa` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::RGBa::RGBa;
    /// 
    /// let nice_blue = RGBa::new(128, 191, 255, 255);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> RGBa {
        RGBa { red, green, blue, alpha }
    }
    /// Create a new `RGBa` struct.
    /// # Arguments
    /// * `cmyk: &CMYK` - The CMYK struct.
    /// # Returns
    /// A new `RGBa` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, RGBa::RGBa};
    /// 
    /// let nice_blue_cmyk = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// let nice_blue_RGBa = RGBa::new(128, 191, 255, 255);
    /// 
    /// assert_eq!(RGBa::new_from_cmyk_struct(&nice_blue_cmyk),
    ///            nice_blue_RGBa);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_cmyk_struct(cmyk: &CMYK) -> RGBa {
        RGBa { red:   (255.0 * (1.0 - cmyk.cyan()) * (1.0 - cmyk.black())).round() as u8,
              green: (255.0 * (1.0 - cmyk.magenta()) * (1.0 - cmyk.black())).round() as u8,
              blue:  (255.0 * (1.0 - cmyk.yellow()) * (1.0 - cmyk.black())).round() as u8,
              alpha: 255 }
    }
    /// Determines whethe `self` is transparent or not. `true` if `alpha == 0`, otherwise `false`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, RGBa::RGBa};
    /// let transparent_colour = RGBa::new(128, 191, 255, 0);
    /// 
    /// assert!(transparent_colour.is_transparent());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_transparent(&self) -> bool {
        self.alpha == 0
    }
    /// Determines whethe `self` is opaque or not. `true` if `alpha == 255`, otherwise `false`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgb::RGBa};
    /// let opaque_colour = RGBa::new(128, 191, 255, 255);
    /// 
    /// assert!(opaque_colour.is_opaque());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        self.alpha == 255
    }
    /// Ignores `alpha` and generates a new `RGB` struct.
    /// # Returns
    /// A new `RGB`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::rgb::*;
    /// 
    /// let rgb  = RGB::SOLID_WHITE;
    /// let rgba = RGBa::SOLID_WHITE;
    /// 
    /// assert_eq!(rgb, rgba.as_rgb());
    /// ```
    #[inline]
    #[must_use]
    pub fn as_rgb(&self) -> RGB {
        RGB {
            red:   self.red,
            green: self.green,
            blue:  self.blue
        }
    }
    /// Create a new `RGBa` struct from `CMYK` values.
    /// # Arguments
    /// * `cmyk: &CMYK` - The CMYK struct.
    /// # Returns
    /// A new `RGBa` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::rgba::RGBa;
    /// 
    /// let nice_blue_RGBa = RGBa::new(128, 191, 255, 255);
    /// 
    /// assert_eq!(RGBa::new_from_cmyk_vals(0.5, 0.25, 0.0, 0.0),
    ///            nice_blue_RGBa);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_cmyk_vals(cyan:    f32,
                              magenta: f32,
                              yellow:  f32,
                              black:   f32) -> RGBa {
        RGBa::new_from_cmyk_struct(&CMYK::new(cyan, magenta, yellow, black))
    }
    /// The pure colour red, Hex-Code `FF0000FF`.
    pub const SOLID_RED: RGBa = RGBa { red: 255, green: 0, blue: 0, alpha: 255};
    /// The pure colour green, Hex-Code `00FF00FF`.
    pub const SOLID_GREEN: RGBa = RGBa { red: 0, green: 255, blue: 0, alpha: 255};
    /// The pure colour blue, Hex-Code `0000FFFF`.
    pub const SOLID_BLUE: RGBa = RGBa { red: 0, green: 0, blue: 255, alpha: 255};
    /// The pure colour white, Hex-Code `FFFFFFFF`.
    pub const SOLID_WHITE: RGBa = RGBa { red: 255, green: 255, blue: 255, alpha: 255};
    /// The pure colour black, Hex-Code `000000FF`.
    pub const SOLID_BLACK: RGBa = RGBa { red: 0, green: 0, blue: 0, alpha: 255};
    /// LibRapid Blue, Hex-Code `9074FFFF`.
    pub const LIBRAPID_SOLID_BLUE: RGBa = RGBa { red: 144, green: 116, blue: 255, alpha: 255};
    /// LibRapid Red, Hex-Code `FF3600FF`.
    pub const LIBRAPID_SOLID_RED: RGBa = RGBa { red: 255, green: 54, blue: 0, alpha: 255};
}

impl std::fmt::Debug for RGBa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Red: {}; Blue: {}; Green: {}; Alpha: {}", self.red,
                                                             self.green,
                                                             self.blue,
                                                             self.alpha)
    }
}

impl std::fmt::Display for RGBa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.alpha)
    }
}
/// Iterate over a `RGBa` struct in the order `R, G, B`.
impl IntoIterator for RGBa {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.red, self.green, self.blue, self.alpha].into_iter()
    }
}