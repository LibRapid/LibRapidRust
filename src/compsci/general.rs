/**
Trait for `binary_insert`. 
*/
pub trait BinayInsert<T> {
    /**
    Insert an element into a ***sorted*** `vec` whilst maintaining the order.

    # Arguments
    * `value` - The value which needs to be inserted.

    # Returns
    Nothing.

    ## WARNING
    When the given `vec` has multiple identical elements (which are close to the `value` to be inserted), you may need to resort after insertion. This will likely be fixed in future versions.
    
    This function ***will not*** check if the parsed `vec` is sorted. 
    */
    fn binary_insert(&mut self, value: &T);
}

impl<T: Ord + Copy> BinayInsert<T> for Vec<T> {
    fn binary_insert(&mut self, value: &T) {
        match self.binary_search(value) {
            Ok(pos)  => self.insert(pos + 1, value.clone()),
            Err(pos) => self.insert(pos, value.clone()),
        }
    }
}