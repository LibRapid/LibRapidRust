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
    fn byte_at(&self, index: usize) -> u8;
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