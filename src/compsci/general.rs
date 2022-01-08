//! General purpose functionalities for computer science. Got any wishes? Tell us on GitHub or our Discord.
/// Trait for `binary_insert`.
pub trait BinayInsert<T> {
    /// Insert an element into a ***sorted*** `vec` whilst maintaining the order.
    ///
    /// # Arguments
    /// * `value` - The value which needs to be inserted.
    ///
    /// # Returns
    /// Nothing.
    ///
    /// # Warning
    /// This function ***will not*** check if the parsed `vec` is sorted.
    fn binary_insert(&mut self, value: &T);
    /// The same as `binary_insert`, but doesn't insert a value that is already present.
    fn binary_insert_no_dup(&mut self, value: T);
}
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

impl<T: Ord + Copy> BinayInsert<T> for Vec<T> {
    fn binary_insert(&mut self, value: &T) {
        match self.binary_search(value) {
            Ok(pos)  => self.insert(pos + 1, *value),
            Err(pos) => self.insert(pos, *value),
        }
    }

    fn binary_insert_no_dup(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(_)    => { },
            Err(pos) => self.insert(pos, value),
        }
    }
}

impl<T: Copy + std::ops::Shl<Output = T> + Into<u128>> BinaryPrefix<T> {
    /// Converts a Binary-Prefix value into a regular `u128`.
    /// # Returns
    /// A `u128`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::unit_conversion::BinaryPrefix;
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
/// WARNING: This ***does not*** check for overflow.
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
/// WARNING: This ***does not*** check for overflow.
impl<T: Copy + std::ops::Shl<Output = T> + Into<u64>> Into<u64> for BinaryPrefix<T> {
    fn into(self) -> u64 {
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
/// WARNING: This ***does not*** check for overflow.
impl<T: Copy + std::ops::Shl<Output = T> + Into<u32>> Into<u32> for BinaryPrefix<T> {
    fn into(self) -> u32 {
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
/// WARNING: This ***does not*** check for overflow.
impl<T: Copy + std::ops::Shl<Output = T> + Into<u16>> Into<u16> for BinaryPrefix<T> {
    fn into(self) -> u16 {
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
/// WARNING: This ***does not*** check for overflow.
impl<T: Copy + std::ops::Shl<Output = T> + Into<u8>> Into<u8> for BinaryPrefix<T> {
    fn into(self) -> u8 {
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