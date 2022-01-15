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
    /// `Ok(true)` if all brackets were closed, otherwise `Err(isize)`, where `isize` is the index of the String at which the error occured.
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
    #[must_use]
    fn validate_brackets(&self) -> Result<bool, usize>;
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

        if s.len() != 0
        { return Err(i); }
        return Ok(true);
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

        if s.len() != 0
        { return Err(i); }
        return Ok(true);
    }
}

impl StringIndex for String {
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }

    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
}

impl StringIndex for &str {
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }

    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
}

impl<T: Ord + Copy> BinaryInsert<T> for Vec<T> {
    fn binary_insert(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(pos)  => self.insert(pos + 1, value),
            Err(pos) => self.insert(pos, value),
        }
    }

    fn binary_insert_no_dup(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(_)    => { },
            Err(pos) => self.insert(pos, value),
        }
    }
}