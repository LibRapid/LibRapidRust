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
    ///
    /// # Arguments
    /// * `index` - The index of the character
    ///
    /// # Returns
    /// A `Option<char>`.
    fn char_at(&self, index: usize) -> Option<char>;
}

impl StringIndex for String {
    fn char_at(&self, index: usize) -> Option<char> {
        return self.chars().nth(index);
    }
}

impl StringIndex for &str {
    fn char_at(&self, index: usize) -> Option<char> {
        return self.chars().nth(index);
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