//! Unit conversions for computer science.
/// Binary Prefixes (e.g. kibi, mebi).
pub enum BinaryPrefix<T> {
    /// x · 2⁰.
    Unity(T),
    /// x · 2¹⁰.
    Kibi(T),
    /// x · 2²⁰.
    Mebi(T),
    /// x · 2³⁰.
    Gibi(T),
    /// x · 2⁴⁰.
    Tebi(T),
    /// x · 2⁵⁰.
    Pebi(T),
    /// x · 2⁶⁰.
    Exbi(T),
    /// x · 2⁷⁰.
    Zebi(T),
    /// x · 2⁸⁰.
    Yobi(T),
}

impl<T: Copy + std::ops::Shl<Output = T> + Into<u128>> BinaryPrefix<T> {
    /// Converts a Binary-Prefix value into a regular `u128`.
    /// # Returns
    /// A `u128`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BinaryPrefix;
    /// assert_eq!(5120u128, BinaryPrefix::Kibi(5u8).to_decimal_u128());
    /// assert_eq!(5u128, BinaryPrefix::Unity(5u8).to_decimal_u128());
    /// assert_eq!(1073741824u128, BinaryPrefix::Gibi(1u8).to_decimal_u128());
    /// ```
    pub fn to_decimal_u128(&self) -> u128 {
        match self {
            BinaryPrefix::Unity(x) => { return (*x).into();       },
            BinaryPrefix::Kibi(x)  => { return (*x).into() << 10; },
            BinaryPrefix::Mebi(x)  => { return (*x).into() << 20; },
            BinaryPrefix::Gibi(x)  => { return (*x).into() << 30; },
            BinaryPrefix::Tebi(x)  => { return (*x).into() << 40; },
            BinaryPrefix::Pebi(x)  => { return (*x).into() << 50; },
            BinaryPrefix::Exbi(x)  => { return (*x).into() << 60; },
            BinaryPrefix::Zebi(x)  => { return (*x).into() << 70; },
            BinaryPrefix::Yobi(x)  => { return (*x).into() << 80; },
        }
    }
}
/// # Tests
/// ```
/// use lib_rapid::compsci::general::BinaryPrefix;
/// assert_eq!(3355443.2f64, BinaryPrefix::Mebi(3.2).into());
/// assert_eq!(4198.4f64, BinaryPrefix::Kibi(4.1).into());
/// ```
impl<T: Copy + Into<f64>> Into<f64> for BinaryPrefix<T> {
    fn into(self) -> f64 {
        match self {
            BinaryPrefix::Unity(x) => { return (x).into();       },
            BinaryPrefix::Kibi(x)  => { return (x).into() * 1024.0; },
            BinaryPrefix::Mebi(x)  => { return (x).into() * 1048576.0; },
            BinaryPrefix::Gibi(x)  => { return (x).into() * 1073741824.0; },
            BinaryPrefix::Tebi(x)  => { return (x).into() * 1099511627776.0; },
            BinaryPrefix::Pebi(x)  => { return (x).into() * 1125899906842624.0; },
            BinaryPrefix::Exbi(x)  => { return (x).into() * 1152921504606846976.0; },
            BinaryPrefix::Zebi(x)  => { return (x).into() * 1180591620717411303424.0; },
            BinaryPrefix::Yobi(x)  => { return (x).into() * 1208925819614629174706176.0; },
        }
    }
}
/// # Tests
/// ```
/// use lib_rapid::compsci::general::BinaryPrefix;
/// assert_eq!(3355443.2f32, BinaryPrefix::Mebi(3.2).into());
/// assert_eq!(4198.4f32, BinaryPrefix::Kibi(4.1).into());
/// ```
impl<T: Copy + Into<f32>> Into<f32> for BinaryPrefix<T> {
    fn into(self) -> f32 {
        match self {
            BinaryPrefix::Unity(x) => { return (x).into();       },
            BinaryPrefix::Kibi(x)  => { return (x).into() * 1024.0; },
            BinaryPrefix::Mebi(x)  => { return (x).into() * 1048576.0; },
            BinaryPrefix::Gibi(x)  => { return (x).into() * 1073741824.0; },
            BinaryPrefix::Tebi(x)  => { return (x).into() * 1099511627776.0; },
            BinaryPrefix::Pebi(x)  => { return (x).into() * 1125899906842624.0; },
            BinaryPrefix::Exbi(x)  => { return (x).into() * 1152921504606846976.0; },
            BinaryPrefix::Zebi(x)  => { return (x).into() * 1180591620717411303424.0; },
            BinaryPrefix::Yobi(x)  => { return (x).into() * 1208925819614629174706176.0; },
        }
    }
}
/// # Tests
/// ```
/// use lib_rapid::compsci::general::BinaryPrefix;
/// assert_eq!(3145728u128, BinaryPrefix::Mebi(3 as u128).into());
/// assert_eq!(4096u128, BinaryPrefix::Kibi(4 as u128).into());
/// ```
impl<T: Copy + std::ops::Shl<Output = T> + Into<u128>> Into<u128> for BinaryPrefix<T> {
    fn into(self) -> u128 {
        match self {
            BinaryPrefix::Unity(x) => { return (x).into();       },
            BinaryPrefix::Kibi(x)  => { return (x).into() << 10; },
            BinaryPrefix::Mebi(x)  => { return (x).into() << 20; },
            BinaryPrefix::Gibi(x)  => { return (x).into() << 30; },
            BinaryPrefix::Tebi(x)  => { return (x).into() << 40; },
            BinaryPrefix::Pebi(x)  => { return (x).into() << 50; },
            BinaryPrefix::Exbi(x)  => { return (x).into() << 60; },
            BinaryPrefix::Zebi(x)  => { return (x).into() << 70; },
            BinaryPrefix::Yobi(x)  => { return (x).into() << 80; },
        }
    }
}