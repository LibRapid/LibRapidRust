//! In here you'll find the struct for the classic `RGB` colour-definition and the extended `RGBa`-definition.
use crate::{compsci::colours::cmyk::CMYK, math::general::Averages};
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
    /// use lib_rapid::compsci::colours::rgba::RGBa;
    /// 
    /// let nice_blue = RGBa::new(128, 191, 255, 255);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> RGBa {
        RGBa { red, green, blue, alpha }
    }
    /// Create a new `RGBa` struct from a `&str` of the form `00000000`.
    /// # Arguments
    /// * `s: &str` - The string from which the struct should be created.
    /// # Panics
    /// Panics if the length of `s` is not equal to 8 or if it couldn't parse one of the numbers as such.
    /// # Returns
    /// A new `RGBa` struct.
    /// ```
    /// use lib_rapid::compsci::colours::rgba::RGBa;
    /// 
    /// let c_1 = RGBa::from_str("ffffffff");
    /// let c_2 = RGBa::new(255, 255, 255, 255);
    /// assert_eq!(c_1, c_2);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_str(s: &str) -> RGBa {
        if s.len() != 8
        { panic!("String did not have the required length of 8."); }
        let mut vals: Vec<u8> = Vec::with_capacity(4);
        for _s in s.as_bytes().chunks(2) {
            let mut r = String::with_capacity(2);
            r.push(_s[0] as char);
            r.push(_s[1] as char);
            vals.push(u8::from_str_radix(&r, 16).unwrap());
        }

        RGBa { red:   vals[0],
               green: vals[1],
               blue:  vals[2],
               alpha: vals[3] }
    }
    /// Create a new `RGBa` struct.
    /// # Arguments
    /// * `cmyk: &CMYK` - The CMYK struct.
    /// # Returns
    /// A new `RGBa` struct.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgba::RGBa};
    /// 
    /// let nice_blue_cmyk = CMYK::new(0.5, 0.25, 0.0, 0.0);
    /// let nice_blue_rgba = RGBa::new(128, 191, 255, 255);
    /// 
    /// assert_eq!(RGBa::from_cmyk_struct(&nice_blue_cmyk),
    ///            nice_blue_rgba);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_cmyk_struct(cmyk: &CMYK) -> RGBa {
        RGBa { red:   (255.0 * (1.0 - cmyk.cyan()) * (1.0 - cmyk.black())).round() as u8,
               green: (255.0 * (1.0 - cmyk.magenta()) * (1.0 - cmyk.black())).round() as u8,
               blue:  (255.0 * (1.0 - cmyk.yellow()) * (1.0 - cmyk.black())).round() as u8,
               alpha: 255 }
    }
    /// Determines whethe `self` is transparent or not. `true` if `alpha != 255`, otherwise `false`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgba::RGBa};
    /// let transparent_colour = RGBa::new(128, 191, 255, 0);
    /// 
    /// assert!(transparent_colour.is_transparent());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_transparent(&self) -> bool {
        !self.is_opaque()
    }
    /// Determines whethe `self` is opaque or not. `true` if `alpha == 255`, otherwise `false`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgba::RGBa};
    /// let opaque_colour = RGBa::new(128, 191, 255, 255);
    /// 
    /// assert!(opaque_colour.is_opaque());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        self.alpha == 255
    }
    /// Convert `self` into a string with no alpha value.
    /// # Returns
    /// A `String`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::colours::{cmyk::CMYK, rgba::RGBa};
    /// let opaque_colour = RGBa::SOLID_WHITE;
    /// 
    /// assert_eq!(String::from("#FFFFFF"), opaque_colour.to_string_no_alpha());
    /// ```
    pub fn to_string_no_alpha(&self) -> String {
        let mut res = self.to_string();
        res.drain((res.len() - 2)..res.len());
        res
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
    /// assert_eq!(RGBa::from_cmyk_vals(0.5, 0.25, 0.0, 0.0),
    ///            nice_blue_RGBa);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_cmyk_vals(cyan:    f32,
                          magenta: f32,
                          yellow:  f32,
                          black:   f32) -> RGBa {
        RGBa::from_cmyk_struct(&CMYK::new(cyan, magenta, yellow, black))
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
/// Iterate over a `RGBa` struct in the order `R, G, B, a`.
impl IntoIterator for RGBa {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.red, self.green, self.blue, self.alpha].into_iter()
    }
}

/// Only uses the R, B and G values.
impl<T: std::convert::From<f32>> Averages<T> for RGBa {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        vec![self.red as f32, self.green as f32, self.blue as f32].arithmetic_mean()
    }

    fn harmonic_mean(&self) -> Self::Output {
        vec![self.red as f32, self.green as f32, self.blue as f32].harmonic_mean()
    }

    fn median(&self) -> Self::Output {
        vec![self.red, self.green, self.blue].median()
    }

    fn mode(&self) -> T {
        vec![self.red as f32, self.green as f32, self.blue as f32].mode().into()
    }

    fn mid_range(&self) -> Self::Output {
        vec![self.red as f32, self.green as f32, self.blue as f32].mid_range()
    }
}