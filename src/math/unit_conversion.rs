//! Traits and functions for converting between units.
use super::constants;
/// SI-Prefixes as used in several fields.
#[derive(PartialEq, Clone, Copy)]
pub enum SIPrefix<T> {
    /// x · 10⁻²⁴.
    Yocto(T),
    /// x · 10⁻²¹.
    Zepto(T),
    /// x · 10⁻¹⁸.
    Atto(T),
    /// x · 10⁻¹⁵.
    Femto(T),
    /// x · 10⁻¹².
    Pico(T),
    /// x · 10⁻⁹.
    Nano(T),
    /// x · 10⁻⁶.
    Micro(T),
    /// x · 10⁻³.
    Milli(T),
    /// x · 10⁻².
    Centi(T),
    /// x · 10⁻¹.
    Deci(T),
    /// x · 10⁰.
    Unity(T),
    /// x · 10¹.
    Deca(T),
    /// x · 10².
    Hecto(T),
    /// x · 10³.
    Kilo(T),
    /// x · 10⁶.
    Mega(T),
    /// x · 10⁹.
    Giga(T),
    /// x · 10¹².
    Tera(T),
    /// x · 10¹⁵.
    Peta(T),
    /// x · 10¹⁸.
    Exa(T),
    /// x · 10²¹.
    Zetta(T),
    /// x · 10²⁴.
    Yotta(T)
}
/// The conversion algorithm to be chosen. Used by `temp_conversion`.
pub enum TempConversion {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,

    FahrenheitToKelvin,
    KelvinToFahrenheit,

    CelsiusToKelvin,
    KelvinToCelsius,
}
/// The conversion algorithm to be chosen. Used by `angle_conversion`.
pub enum AngleConversion {
    DegreesToRadians,
    RadiansToDegrees,
}
/// Trait for angle conversion.
pub trait AngleConversionTrait {
    /// Performs a angle conversion.
    /// # Arguments
    /// * `&self` - The value to be converted.
    /// * `mode` - The mode ( e.g. RadiansToDegrees ).
    /// # Returns
    /// A `Self` containing the result.
    #[must_use]
    fn angle_conversion(&self, mode: AngleConversion) -> Self;
}

/// Trait for temperature conversion.
pub trait TempConversionTrait {
    /// Performs a temperature conversion.
    /// # Arguments
    /// * `&self` - The value to be converted.
    /// * `mode` - The mode ( e.g. CelsiusToFahrenheit ).
    /// # Returns
    /// A `Self` containing the result.
    #[must_use]
    fn temp_conversion(&self, mode: TempConversion) -> Self;
}

impl TempConversionTrait for f64 {
    fn temp_conversion(&self, mode: TempConversion) -> Self {
        match mode {
            TempConversion::CelsiusToFahrenheit => {  self * 1.8 + 32.0 }
            TempConversion::CelsiusToKelvin     => {  self + 273.15 }
            TempConversion::FahrenheitToCelsius => { (self - 32.0) / 1.8 }
            TempConversion::FahrenheitToKelvin  => { (self - 32.0) / 1.8 + 273.15 }
            TempConversion::KelvinToCelsius     => {  self - 273.15 }
            TempConversion::KelvinToFahrenheit  => { (self - 273.15) * 1.8 + 32.0 }
        }
    }
}

impl TempConversionTrait for f32 {
    fn temp_conversion(&self, mode: TempConversion) -> Self {
        match mode {
            TempConversion::CelsiusToFahrenheit => {  self * 1.8 + 32.0 }
            TempConversion::CelsiusToKelvin     => {  self + 273.15 }
            TempConversion::FahrenheitToCelsius => { (self - 32.0) / 1.8 }
            TempConversion::FahrenheitToKelvin  => { (self - 32.0) / 1.8 + 273.15 }
            TempConversion::KelvinToCelsius     => {  self - 273.15 }
            TempConversion::KelvinToFahrenheit  => { (self - 273.15) * 1.8 + 32.0 }
        }
    }
}

impl AngleConversionTrait for f64 {
    fn angle_conversion(&self, mode: AngleConversion) -> Self {
        match mode {
            AngleConversion::RadiansToDegrees => { self * constants::RADDEGRATE }
            AngleConversion::DegreesToRadians => { self * constants::DEGRADRATE }
        }
    }
}

impl AngleConversionTrait for f32 {
    fn angle_conversion(&self, mode: AngleConversion) -> Self {
        match mode {
            AngleConversion::RadiansToDegrees => { self * constants::RADDEGRATE as f32 }
            AngleConversion::DegreesToRadians => { self * constants::DEGRADRATE as f32 }
        }
    }
}

impl<T: Into<f64> + Copy> SIPrefix<T> {
    /// Converts a SI-Prefix value into a regular `f32`.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::unit_conversion::SIPrefix;
    /// assert_eq!(0.000_000_000_000_000_000_000_005, SIPrefix::Yocto(5.0).to_decimal_f32());
    /// assert_eq!(0.5, SIPrefix::Deci(5.0).to_decimal_f32());
    /// assert_eq!(5000.0, SIPrefix::Kilo(5.0).to_decimal_f32());
    /// ```
    pub fn to_decimal_f32(&self) -> f32 {
        self.to_decimal_f64() as f32
    }
}

impl<T: Into<f64> + Copy> SIPrefix<T> {
    /// Converts a SI-Prefix value into a regular `f64`.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::unit_conversion::SIPrefix;
    /// assert_eq!(0.000_000_000_000_000_000_000_005, SIPrefix::Yocto(5.0).to_decimal_f64());
    /// assert_eq!(0.5, SIPrefix::Deci(5.0).to_decimal_f64());
    /// assert_eq!(5000.0, SIPrefix::Kilo(5.0).to_decimal_f64());
    /// ```
    pub fn to_decimal_f64(&self) -> f64 {
        match self {
            SIPrefix::Yocto(x) => { (*x).into() * SIRATES[0]  },
            SIPrefix::Zepto(x) => { (*x).into() * SIRATES[1]  },
            SIPrefix::Atto(x)  => { (*x).into() * SIRATES[2]  },
            SIPrefix::Femto(x) => { (*x).into() * SIRATES[3]  },
            SIPrefix::Pico(x)  => { (*x).into() * SIRATES[4]  },
            SIPrefix::Nano(x)  => { (*x).into() * SIRATES[5]  },
            SIPrefix::Micro(x) => { (*x).into() * SIRATES[6]  },
            SIPrefix::Milli(x) => { (*x).into() * SIRATES[7]  },
            SIPrefix::Centi(x) => { (*x).into() * SIRATES[8]  },
            SIPrefix::Deci(x)  => { (*x).into() * SIRATES[9]  },
            SIPrefix::Unity(x) => { (*x).into()               },
            SIPrefix::Deca(x)  => { (*x).into() * SIRATES[10] },
            SIPrefix::Hecto(x) => { (*x).into() * SIRATES[11] },
            SIPrefix::Kilo(x)  => { (*x).into() * SIRATES[12] },
            SIPrefix::Mega(x)  => { (*x).into() * SIRATES[13] },
            SIPrefix::Giga(x)  => { (*x).into() * SIRATES[14] },
            SIPrefix::Tera(x)  => { (*x).into() * SIRATES[15] },
            SIPrefix::Peta(x)  => { (*x).into() * SIRATES[16] },
            SIPrefix::Exa(x)   => { (*x).into() * SIRATES[17] },
            SIPrefix::Zetta(x) => { (*x).into() * SIRATES[18] },
            SIPrefix::Yotta(x) => { (*x).into() * SIRATES[19] },
        }
    }
}

impl<T: Into<f64> + Copy> Into<f64> for SIPrefix<T> {
    fn into(self) -> f64 {
        self.to_decimal_f64()
    }
}

impl<T: Into<f64> + Copy> Into<f32> for SIPrefix<T> {
    fn into(self) -> f32 {
        self.to_decimal_f32()
    }
}
/// The different rates for converting from a SI prefix to a normalised value.
pub(crate) const SIRATES: [f64; 20] = [1e-24,
                                       1e-21,
                                       1e-18,
                                       1e-15,
                                       1e-12,
                                       1e-9,
                                       1e-6,
                                       1e-3,
                                       0.01,
                                       0.1,
                                       10.0,
                                       1e2,
                                       1e3,
                                       1e6,
                                       1e9,
                                       1e12,
                                       1e15,
                                       1e18,
                                       1e21,
                                       1e24];