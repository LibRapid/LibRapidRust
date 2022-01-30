//! General purpose functionalities for computer science. Got any wishes? Tell us on GitHub or our Discord.
use crate::math::general::Increment;
/// Trait for `binary_insert`.
pub trait BinaryInsert<T> {
    /// Insert an element into a ***sorted*** `vec` whilst maintaining the order, consuming `other`.
    ///
    /// # Arguments
    /// * `value` - The value which needs to be inserted.
    ///
    /// # Returns
    /// Nothing.
    ///
    /// # Warning
    /// This function ***will not*** check if the parsed `vec` is sorted.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 1, 2, 3, 4, 5];
    /// v.binary_insert(3);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 3, 4, 5], v);
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 2, 3, 4, 5];
    /// v.binary_insert(1);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 4, 5], v);
    /// ```
    fn binary_insert(&mut self, value: T);
    /// The same as `binary_insert`, but doesn't insert a value that is already present.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 1, 2, 3, 4, 5];
    /// v.binary_insert_no_dup(3);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 4, 5], v);
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 1, 2, 3, 4, 5];
    /// v.binary_insert(6);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], v);
    /// ```
    fn binary_insert_no_dup(&mut self, value: T);
}
/// Trait for a kind of indexing Strings in Rust.
pub trait StringIndex {
    /// Get the `char` at a given index from a `String` or `&str`.
    /// If only dealing with ASCII-characters, `byte_at` is recommended.
    /// 
    /// # Arguments
    /// * `index` - The index of the character
    ///
    /// # Returns
    /// A `Option<char>`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.char_at(0).unwrap());
    /// assert_eq!('e', s.char_at(1).unwrap());
    /// assert_eq!('l', s.char_at(2).unwrap());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = "Hello";
    /// assert_eq!('H', s.char_at(0).unwrap());
    /// assert_eq!('e', s.char_at(1).unwrap());
    /// assert_eq!('l', s.char_at(2).unwrap());
    /// ```
    #[must_use]
    fn char_at(&self, index: usize) -> Option<char>;
    /// Get the byte at a given index from a `String` or `&str`.
    /// If dealing with Unicode characters, this function is *not* recommended, as it only returns one byte even though Unicode can be a multi byte encoding.
    /// 
    /// # Arguments
    /// * `index` - The index of the byte
    ///
    /// # Returns
    /// A `u8`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = "Hello";
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    #[must_use]
    fn byte_at(&self, index: usize) -> u8;
}
/// Trait for functions related to brackets.
pub trait Brackets {
    /// Determines whether the given `&str` or `String` has valid brackets.
    /// # Returns
    /// `Ok(true)` if all brackets were closed, otherwise `Err(usize)`, where `usize` is the index of the String at which the error occured.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{Some text won't throw it off}])";
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = String::from("([{}])");
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{}]";
    /// 
    /// assert_eq!(Err(5), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{Some text won't throw it off}]))";
    /// 
    /// assert_eq!(Err(34), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{]}))";
    /// 
    /// assert_eq!(Err(3), s.validate_brackets());
    /// ```
    fn validate_brackets(&self) -> Result<bool, usize>;
}
/// Get bits and bytes from a floating point number.
pub trait FloatMagic {
    /// Get the raw binary mantissa (fractional part) data.
    /// # Returns
    /// A `u64`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(2570638229164439, 3.1415927_f64.raw_mantissa());
    /// ```
    #[must_use]
    fn raw_mantissa(&self) -> u64;
    /// Get the raw binary exponent data.
    /// # Returns
    /// A `u16`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(1024, 3.1415927_f64.raw_exponent());
    /// ```
    #[must_use]
    fn raw_exponent(&self) -> u16;
    /// Get the actual mantissa part.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(1.57079635, 3.1415927_f64.real_mantissa());
    /// ```
    #[must_use]
    fn real_mantissa(&self) -> f64;
    /// Get the actual exponent part.
    /// # Returns
    /// A `i32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(1, 3.1415927_f64.real_exponent());
    /// ```
    #[must_use]
    fn real_exponent(&self) -> i32;
}

impl FloatMagic for f32 {
    #[inline]
    fn raw_mantissa(&self) -> u64 {
        let _t: u32 = unsafe { std::intrinsics::transmute(*self) };
        (_t & 0b00000000011111111111111111111111) as u64
    }
    #[inline]
    fn raw_exponent(&self) -> u16 {
        let _t: u32 = unsafe { std::intrinsics::transmute(*self) };
        ((_t & 0b01111111100000000000000000000000) >> 23) as u16
    }
    #[inline]
    fn real_mantissa(&self) -> f64 {
        let mut _t: u32 = unsafe { std::intrinsics::transmute(*self) };
        _t &= !0b01111111100000000000000000000000;
        _t |=  0b00111111100000000000000000000000;
        let _m: f32 = unsafe { std::intrinsics::transmute(_t) };

        _m as f64
    }
    #[inline]
    fn real_exponent(&self) -> i32 {
        self.raw_exponent() as i32 - 127
    }
}

impl FloatMagic for f64 {
    #[inline]
    fn raw_mantissa(&self) -> u64 {
        let _t: u64 = unsafe { std::intrinsics::transmute(*self) };
        (_t &  0b0000000000001111111111111111111111111111111111111111111111111111) as u64
    }
    #[inline]
    fn raw_exponent(&self) -> u16 {
        let _t: u64 = unsafe { std::intrinsics::transmute(*self) };
        ((_t & 0b0111111111110000000000000000000000000000000000000000000000000000) >> 52) as u16
    }
    #[inline]
    fn real_mantissa(&self) -> f64 {
        let mut _t: u64 = unsafe { std::intrinsics::transmute(*self) };
        _t &= !0b0111111111110000000000000000000000000000000000000000000000000000;
        _t |=  0b0011111111110000000000000000000000000000000000000000000000000000;
        unsafe { std::intrinsics::transmute(_t) }
    }
    #[inline]
    fn real_exponent(&self) -> i32 {
        self.raw_exponent() as i32 - 1023
    }
}

impl Brackets for &str {
    fn validate_brackets(&self) -> Result<bool, usize> {
        let mut s: Vec<char> = Vec::with_capacity(self.len());
        let mut i: usize = 0;
        for c in self.chars() {
            match c {
                '[' => { s.push(c); },
                '{' => { s.push(c); },
                '(' => { s.push(c); },
                ']' => { 
                         if s.pop() != Some('[')
                         { return Err(i); }
                        },
                '}' => { if s.pop() != Some('{')
                         { return Err(i); }
                        },
                ')' => { if s.pop() != Some('(')
                         { return Err(i); }
                        },
                _   => { }
            }
            i.inc();
        }

        if !s.is_empty()
        { return Err(i); }
        Ok(true)
    }
}

impl Brackets for String {
    fn validate_brackets(&self) -> Result<bool, usize> {
        let mut s: Vec<char> = Vec::with_capacity(self.len());
        let mut i: usize = 0;
        for c in self.chars() {
            match c {
                '[' => { s.push(c); },
                '{' => { s.push(c); },
                '(' => { s.push(c); },
                ']' => { if s.pop() != Some('[')
                         { return Err(i); }
                        },
                '}' => { if s.pop() != Some('{')
                         { return Err(i); }
                        },
                ')' => { if s.pop() != Some('(')
                         { return Err(i); }
                        },
                _   => { }
            }
            i.inc();
        }

        if !s.is_empty()
        { return Err(i); }
        Ok(true)
    }
}

impl StringIndex for String {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
}

impl StringIndex for &str {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
}

impl<T: Ord + Copy> BinaryInsert<T> for Vec<T> {
    #[inline]
    fn binary_insert(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(pos)  => self.insert(pos + 1, value),
            Err(pos) => self.insert(pos, value),
        }
    }
    #[inline]
    fn binary_insert_no_dup(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(_)    => { },
            Err(pos) => self.insert(pos, value),
        }
    }
}