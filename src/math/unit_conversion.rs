//! Traits and functions for converting between units.
use super::constants;
/// SI-Prefixes.
pub enum SIPrefix<T> {
    /// 10⁻²⁴.
    Yocto(T),
    /// 10⁻²¹.
    Zepto(T),
    /// 10⁻¹⁸.
    Atto(T),
    /// 10⁻¹⁵.
    Femto(T),
    /// 10⁻¹².
    Pico(T),
    /// 10⁻⁹.
    Nano(T),
    /// 10⁻⁶.
    Micro(T),
    /// 10⁻³.
    Milli(T),
    /// 10⁻².
    Centi(T),
    /// 10⁻¹.
    Deci(T),
    /// 10¹.
    Deca(T),
    /// 10².
    Hecto(T),
    /// 10³.
    Kilo(T),
    /// 10⁶.
    Mega(T),
    /// 10⁹.
    Giga(T),
    /// 10¹².
    Tera(T),
    /// 10¹⁵.
    Peta(T),
    /// 10¹⁸.
    Exa(T),
    /// 10²¹.
    Zetta(T),
    /// 10²⁴.
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
    ///
    /// # Arguments
    /// * `&self` - The value to be converted.
    /// * `mode` - The mode ( e.g. RadiansToDegrees ).
    ///
    /// # Returns
    /// A `Self` containing the result.
    fn angle_conversion(&self, mode: AngleConversion) -> Self;
}

/// Trait for temperature conversion.
pub trait TempConversionTrait {
    /// Performs a temperature conversion.
    ///
    /// # Arguments
    /// * `&self` - The value to be converted.
    /// * `mode` - The mode ( e.g. CelsiusToFahrenheit ).
    ///
    /// # Returns
    /// A `Self` containing the result.
    fn temp_conversion(&self, mode: TempConversion) -> Self;
}

impl TempConversionTrait for f64 {
    fn temp_conversion(&self, mode: TempConversion) -> Self {
        match mode {
            TempConversion::CelsiusToFahrenheit => { return self * 1.8 + 32.0; }
            TempConversion::CelsiusToKelvin     => { return self + 273.15; }
            TempConversion::FahrenheitToCelsius => { return (self - 32.0) / 1.8; }
            TempConversion::FahrenheitToKelvin  => { return (self - 32.0) / 1.8 + 273.15; }
            TempConversion::KelvinToCelsius     => { return self - 273.15; }
            TempConversion::KelvinToFahrenheit  => { return (self - 273.15) * 1.8 + 32.0; }
        }
    }
}

impl TempConversionTrait for f32 {
    fn temp_conversion(&self, mode: TempConversion) -> Self {
        match mode {
            TempConversion::CelsiusToFahrenheit => { return self * 1.8 + 32.0; }
            TempConversion::CelsiusToKelvin     => { return self + 273.15; }
            TempConversion::FahrenheitToCelsius => { return (self - 32.0) / 1.8; }
            TempConversion::FahrenheitToKelvin  => { return (self - 32.0) / 1.8 + 273.15; }
            TempConversion::KelvinToCelsius     => { return self - 273.15; }
            TempConversion::KelvinToFahrenheit  => { return (self - 273.15) * 1.8 + 32.0; }
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

impl<T: Into<f32> + Copy> SIPrefix<T> {
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
        match self {
            SIPrefix::Yocto(x) => { return (*x).into() * SIRATES[0]  as f32; },
            SIPrefix::Zepto(x) => { return (*x).into() * SIRATES[1]  as f32; },
            SIPrefix::Atto(x)  => { return (*x).into() * SIRATES[2]  as f32; },
            SIPrefix::Femto(x) => { return (*x).into() * SIRATES[3]  as f32; },
            SIPrefix::Pico(x)  => { return (*x).into() * SIRATES[4]  as f32; },
            SIPrefix::Nano(x)  => { return (*x).into() * SIRATES[5]  as f32; },
            SIPrefix::Micro(x) => { return (*x).into() * SIRATES[6]  as f32; },
            SIPrefix::Milli(x) => { return (*x).into() * SIRATES[7]  as f32; },
            SIPrefix::Centi(x) => { return (*x).into() * SIRATES[8]  as f32; },
            SIPrefix::Deci(x)  => { return (*x).into() * SIRATES[9]  as f32; },
            SIPrefix::Deca(x)  => { return (*x).into() * SIRATES[10] as f32; },
            SIPrefix::Hecto(x) => { return (*x).into() * SIRATES[11] as f32; },
            SIPrefix::Kilo(x)  => { return (*x).into() * SIRATES[12] as f32; },
            SIPrefix::Mega(x)  => { return (*x).into() * SIRATES[13] as f32; },
            SIPrefix::Giga(x)  => { return (*x).into() * SIRATES[14] as f32; },
            SIPrefix::Tera(x)  => { return (*x).into() * SIRATES[15] as f32; },
            SIPrefix::Peta(x)  => { return (*x).into() * SIRATES[16] as f32; },
            SIPrefix::Exa(x)   => { return (*x).into() * SIRATES[17] as f32; },
            SIPrefix::Zetta(x) => { return (*x).into() * SIRATES[18] as f32; },
            SIPrefix::Yotta(x) => { return (*x).into() * SIRATES[19] as f32; },
        }
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
            SIPrefix::Yocto(x) => { return (*x).into() * SIRATES[0]; },
            SIPrefix::Zepto(x) => { return (*x).into() * SIRATES[1]; },
            SIPrefix::Atto(x)  => { return (*x).into() * SIRATES[2]; },
            SIPrefix::Femto(x) => { return (*x).into() * SIRATES[3]; },
            SIPrefix::Pico(x)  => { return (*x).into() * SIRATES[4]; },
            SIPrefix::Nano(x)  => { return (*x).into() * SIRATES[5]; },
            SIPrefix::Micro(x) => { return (*x).into() * SIRATES[6]; },
            SIPrefix::Milli(x) => { return (*x).into() * SIRATES[7]; },
            SIPrefix::Centi(x) => { return (*x).into() * SIRATES[8]; },
            SIPrefix::Deci(x)  => { return (*x).into() * SIRATES[9]; },
            SIPrefix::Deca(x)  => { return (*x).into() * SIRATES[10]; },
            SIPrefix::Hecto(x) => { return (*x).into() * SIRATES[11]; },
            SIPrefix::Kilo(x)  => { return (*x).into() * SIRATES[12]; },
            SIPrefix::Mega(x)  => { return (*x).into() * SIRATES[13]; },
            SIPrefix::Giga(x)  => { return (*x).into() * SIRATES[14]; },
            SIPrefix::Tera(x)  => { return (*x).into() * SIRATES[15]; },
            SIPrefix::Peta(x)  => { return (*x).into() * SIRATES[16]; },
            SIPrefix::Exa(x)   => { return (*x).into() * SIRATES[17]; },
            SIPrefix::Zetta(x) => { return (*x).into() * SIRATES[18]; },
            SIPrefix::Yotta(x) => { return (*x).into() * SIRATES[19]; },
        }
    }
}

pub(crate) const SIRATES: [f64; 20] = [0.000_000_000_000_000_000_000_001,
                                       0.000_000_000_000_000_000_001,
                                       0.000_000_000_000_000_001,
                                       0.000_000_000_000_001,
                                       0.000_000_000_001,
                                       0.000_000_001,
                                       0.000_001,
                                       0.001,
                                       0.01,
                                       0.1,
                                       10.0,
                                       100.0,
                                       1_000.0,
                                       1_000_000.0,
                                       1_000_000_000.0,
                                       1_000_000_000_000.0,
                                       1_000_000_000_000_000.0,
                                       1_000_000_000_000_000_000.0,
                                       1_000_000_000_000_000_000_000.0,
                                       1_000_000_000_000_000_000_000_000.0];