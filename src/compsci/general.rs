    /**
    Insert an element into a ***sorted*** `vec` whilst maintaining the order.

    # Arguments
    * `target` - the sorted target vec.w
    * `value` - The value which needs to be inserted.

    # Returns
    Nothing.

    ## WARNING
    When the given `vec` has multiple identical elements (which are close to the `value` to be inserted), you may need to resort after insertion. This will likely be fixed in future versions.
    
    This function ***will not*** check if the parsed `vec` is sorted. 
    */
pub fn binary_insert<T: Ord + Copy>(target: &mut Vec<T>, value: &T) {
    match target.binary_search(value) {
        Ok(pos)  => target.insert(pos + 1, value.clone()),
        Err(pos) => target.insert(pos, value.clone()),
    }
}