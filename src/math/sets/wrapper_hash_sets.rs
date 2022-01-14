//! In here you'll find several traits for Rusts own set types.
use std::{collections::{HashSet, hash_map::RandomState}, hash::BuildHasher};
use core::hash::Hash;
/// A wrapper for the HashSet type which implements more functions.
/// Everything you need to do with a HashSet needs to be done using `WrapperHashSet.hs.[function]` for now.
#[derive(Debug)]
pub struct WrapperHashSet<'a, T, S = RandomState> {
    /// The basic Hashset.
    pub hs: HashSet<T, S>,
    parent: Option<&'a WrapperHashSet<'a, T>>
}

impl<'a, T: Clone + std::cmp::Eq + std::hash::Hash> WrapperHashSet<'a, T> {
    /// Wrap a existing HashSet in this Wrapper-struct.
    /// # Arguments
    /// * `existing` - The existing HashSet.
    /// # Returns
    /// A new `WrapperHashSet`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::wrapper_hash_sets::WrapperHashSet;
    /// use std::collections::HashSet;
    /// 
    /// let mut books = HashSet::new();
    /// books.insert("A Dance With Dragons".to_string());
    /// books.insert("1984".to_string());
    /// 
    /// let mut wrapped = WrapperHashSet::wrap(books);
    /// ```
    #[must_use]
    pub fn wrap(existing: HashSet<T>) -> WrapperHashSet<'a, T> {
        WrapperHashSet { hs: existing, parent: None }
    }
    /// Creates a new `WrapperHashSet` from `self` to which it applies a closure.
    ///
    /// # Arguments
    /// * `f` - The closure after which the new `VecSet` is created.
    ///
    /// # Returns
    /// A child `WrapperHashSet`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::wrapper_hash_sets::WrapperHashSet;
    /// use std::collections::HashSet;
    /// 
    /// let mut nums = HashSet::new();
    /// for i in 0..=6 {
    ///     nums.insert(i);
    /// }
    /// 
    /// let wrapped = WrapperHashSet::wrap(nums);
    /// let subset = wrapped.new_subset(|x| x % 2 == 0);
    /// 
    /// assert_eq!(HashSet::from([0, 2, 4, 6]), subset.hs);
    /// ```
    #[must_use]
    pub fn new_subset<F: Fn(&T) -> bool>(&'a self, f: F) -> Self {
        let mut res: HashSet<T> = HashSet::with_capacity(self.hs.capacity());
        for elem in &self.hs {
            if f(elem) {
                res.insert(elem.clone());
            }
        }
        WrapperHashSet { hs: res, parent: Some(self) }
    }
    /// Lets you check wether a set has a parent (emerged from another set) or not.
    ///
    /// # Returns
    /// A boolean value which determines if the set is a subset of any other set.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::wrapper_hash_sets::WrapperHashSet;
    /// use std::collections::HashSet;
    /// 
    /// let mut nums = HashSet::new();
    /// for i in 0..=6 {
    ///     nums.insert(i);
    /// }
    /// 
    /// let wrapped = WrapperHashSet::wrap(nums);
    /// let subset = wrapped.new_subset(|x| x % 2 == 0);
    /// 
    /// assert_eq!(true, subset.has_emerged());
    /// ```
    #[must_use]
    pub fn has_emerged(&self) -> bool {
        self.parent.is_some()
    }

    /// Gets you the optional superset.
    ///
    /// # Returns
    /// A `Option<&VecSet<T>>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::wrapper_hash_sets::WrapperHashSet;
    /// use std::collections::HashSet;
    /// 
    /// let mut nums = HashSet::new();
    /// for i in 0..=6 {
    ///     nums.insert(i);
    /// }
    /// 
    /// let wrapped = WrapperHashSet::wrap(nums);
    /// let subset = wrapped.new_subset(|x| x % 2 == 0);
    /// 
    /// assert_eq!(&wrapped, subset.get_superset().unwrap());
    /// ```
    #[must_use]
    pub fn get_superset(&self) -> Option<&Self> {
        self.parent
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Display for WrapperHashSet<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}, {:?}", self.hs, self.parent))
    }
}

impl<'a, T: std::cmp::Eq + Hash, S: BuildHasher> PartialEq<WrapperHashSet<'a, T, S>> for WrapperHashSet<'a, T, S> {
    fn eq(&self, other: &WrapperHashSet<T, S>) -> bool {

        if self.hs.len() != other.hs.len() {
            return false;
        }

        let b = self.hs.iter().all(|key| other.hs.contains(key));

        b && self.parent == other.parent
    }
}