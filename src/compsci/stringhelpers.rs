//! Certain functions for working with Strings.
use crate::math::general::NumTools;

/// Useful String related functions.
pub trait StringUtils {
    /// Get the `char` at a given index from a `String` or `&str`.
    /// If only dealing with ASCII-characters, `byte_at` is recommended.
    /// 
    /// # Arguments
    /// * `index` - The index of the character
    /// # Returns
    /// A `Option<char>`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.char_at(0).unwrap());
    /// assert_eq!('e', s.char_at(1).unwrap());
    /// assert_eq!('l', s.char_at(2).unwrap());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
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
    /// # Returns
    /// A `u8`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "Hello";
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    #[must_use]
    fn byte_at(&self, index: usize) -> u8;
    /// Determines whether the given `&str` or `String` has valid brackets.
    /// # Returns
    /// `Ok(true)` if all brackets were closed, otherwise `Err(usize)`, where `usize` is the index of the String at which the error occured.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{Some text won't throw it off}])";
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = String::from("([{}])");
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{}]";
    /// 
    /// assert_eq!(Err(5), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{Some text won't throw it off}]))";
    /// 
    /// assert_eq!(Err(34), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{]}))";
    /// 
    /// assert_eq!(Err(3), s.validate_brackets());
    /// ```
    fn validate_brackets(&self) -> Result<bool, usize>;
    /// Checks if a String is alphanumeric.
    /// # Returns
    /// A `Result<bool, usize>`: `true` if it is alphanumeric, else the index of the non alpha numeric character.
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!(true, "h3ll0".is_alphanumeric().is_ok());
    /// assert_eq!(true, "30".is_alphanumeric().is_ok());
    /// assert_eq!(false, "äÄ".is_alphanumeric().is_ok());
    /// assert_eq!(2, "Heä10!".is_alphanumeric().err().unwrap());
    /// ```
    fn is_alphanumeric(&self) -> Result<bool, usize>;
    /// Checks if a String is numeric.
    /// # Returns
    /// A `Result<bool, usize>`: `true` if it is alphanumeric, else the index of the non alpha numeric character.
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!(1, "3ll0".is_numeric().err().unwrap());
    /// assert_eq!(true, "30".is_numeric().is_ok());
    /// ```
    fn is_numeric(&self) -> Result<bool, usize>;
}

impl StringUtils for String {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
    fn validate_brackets(&self) -> Result<bool, usize> {
        backend_val_brackets(self)
    }
    fn is_alphanumeric(&self) -> Result<bool, usize> {
        for c in self.chars().enumerate() {
            if !((c.1 >= '0' && c.1 <= '9') ||
                 (c.1 >= 'a' && c.1 <= 'z') || 
                 (c.1 >= 'A' && c.1 <= 'Z'))
            { return Err(c.0); }
        }
        Ok(true)
    }
    fn is_numeric(&self) -> Result<bool, usize> {
        for c in self.chars().enumerate() {
            if !(c.1 as u8).is_in_range(48, 57)
            { return Err(c.0); }
        }
        Ok(true)
    }
}

impl StringUtils for str {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
    fn validate_brackets(&self) -> Result<bool, usize> {
        backend_val_brackets(self)
    }
    fn is_alphanumeric(&self) -> Result<bool, usize> {
        for c in self.chars().enumerate() {
            if !((c.1 >= '0' && c.1 <= '9') ||
                 (c.1 >= 'a' && c.1 <= 'z') || 
                 (c.1 >= 'A' && c.1 <= 'Z'))
            { return Err(c.0); }
        }
        Ok(true)
    }
    fn is_numeric(&self) -> Result<bool, usize> {
        for c in self.chars().enumerate() {
            if !(c.1 as u8).is_in_range(48, 57)
            { return Err(c.0); }
        }
        Ok(true)
    }
}

#[must_use]
fn backend_val_brackets(s: &str) -> Result<bool, usize> {
    let mut res_vec: Vec<char> = Vec::with_capacity(s.len());
    let mut i: usize = 0;
    for c in s.chars() {
        match c {
            '[' | '{' | '(' => { res_vec.push(c); },
            ']' if res_vec.pop() != Some('[') => { return Err(i); }
            '}' if res_vec.pop() != Some('{') => { return Err(i); }
            ')' if res_vec.pop() != Some('(') => { return Err(i); }
            _   => { }
        }
        i.inc();
    }

    if !res_vec.is_empty()
    { return Err(i); }
    Ok(true)
}

/// A Rust implementation of C's `strcmp()` function.
/// # Arguments
/// * `s1` - The String reference to be compared to.
/// * `s2` - The String which is compared with `s1`.
/// # Returns
/// It returns:
/// * `0`, if `s1 == s2`.
/// * `s2[i] - s1[i]`, if `s1[i] < s2[i]` in ASCII.
/// * `s1[i] - s2[i]`, if `s1[i] > s2[i]` in ASCII.
/// # Examples
/// ```
/// use lib_rapid::compsci::stringhelpers::strcmp;
/// let a = "hello";
/// let b = "hEllo";
/// assert!(strcmp(a, b) == 101 - 69);
/// // 101 == 'e' in ASCII.
/// // 69 == `E` in ASCII.
/// ```
/// ```
/// use lib_rapid::compsci::stringhelpers::strcmp;
/// let a = "hello";
/// let b = "hello";
/// assert!(strcmp(a, b) == 0);
/// ```
/// ```
/// use lib_rapid::compsci::stringhelpers::strcmp;
/// let a = "Hello";
/// let b = "hello";
/// assert!(strcmp(a, b) == 72 - 104);
/// // 72 == 'H' in ASCII.
/// // 104 == `h` in ASCII.
/// ```
pub const fn strcmp(s1: &str, s2: &str) -> i16 {
    let mut i: usize = 0;
    let mut flag: i16 = 0;
    while flag == 0 {
        flag = s1.as_bytes()[i] as i16 - s2.as_bytes()[i] as i16;
        if i + 1 == s1.len()
        { break; }

        i += 1;
    }
    flag
}