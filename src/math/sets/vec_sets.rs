//! Diving deeper into mathematics, huh? In here you'll find mathematical sets - Sometimes pretty handy!
/// Brings mathematical sets into Rust.
#[derive(Debug, Clone)]
pub struct VecSet<'a, T> {
    elements: Vec<T>,
    superset: Option<&'a VecSet<'a, T>>,
}

// Main impl
impl<'a, T: Copy + Clone + Ord> VecSet<'a, T> {
    /// Creates a new `VecSet`.
    ///
    /// # Arguments
    /// * `values` - The values for the `VecSet`.
    ///
    /// # Returns
    /// A new `VecSet`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// 
    /// let set = VecSet::new(&vec![0, 1, 1, 2, 3, 4, 5]);
    /// 
    /// assert_eq!(set.elements(), &vec![0, 1, 2, 3, 4, 5]);
    /// ```
    #[must_use]
    pub fn new(values: &[T]) -> VecSet<'a, T> {        
        let mut res: VecSet<T> = VecSet { elements: values.to_vec(),
                                    superset: None };
        res.elements.sort_unstable();
        res.elements.dedup();
        res
    }
    /// Creates a new `VecSet` using a parent-`VecSet` to which it applies a closure.
    ///
    /// # Arguments
    /// * `parent` - The `VecSet` from which the new `VecSet` emerges.
    /// * `f` - The closure after which the new `VecSet` is created.
    ///
    /// # Returns
    /// A child `VecSet`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// let test1:       VecSet<u8> = VecSet::new(&vec![0,1,2,3,4]);
    /// let from_parent: VecSet<u8> = VecSet::<u8>::new_subset(&test1, |x| x % 2 == 0);
    /// assert_eq!(from_parent, VecSet::new(&vec![0,2,4]));
    /// assert_eq!(test1.elements(), &vec![0,1,2,3,4])
    /// ```
    #[must_use]
    pub fn new_subset<F: Fn(T) -> bool>(parent: &'a VecSet<T>, f: F) -> VecSet<'a, T> {
        let mut res: VecSet<T> = VecSet { elements: Vec::with_capacity(parent.cardinality()),
                                    superset: Some(parent) };
        for elem in &parent.elements {
            if f(*elem) {
                res.elements.push(*elem);
            }
        }
        res
    }
    /// Does a mathematical union on two VecSets.
    /// `self ∪ other`.
    /// # Arguments
    /// * `self` - The first set.
    /// * `other` - The second set.
    ///
    /// # Returns
    /// A new `VecSet<T>`: `self ∪ other`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: VecSet<i32> = VecSet::new(&vec![11,12,13,13,11,0,0,0]);
    /// 
    /// let c:  VecSet<i32> = s.union(&s1);
    /// assert_eq!(c, set!(0,1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    #[must_use]
    pub fn union(&self, other: &VecSet<T>) -> VecSet<T> {
        let mut res: VecSet<T> = VecSet {elements: Vec::new(),
                                         superset: None };

        res.elements.append(&mut self.elements.clone());
        res.elements.append(&mut other.elements.clone());

        res.elements.sort_unstable(); 
        res.elements.dedup();
        res
    }
    /// Does a mathematical intersection on two sets.
    ///
    /// # Arguments
    /// * `self` - The first set.
    /// * `other` - The second set.
    ///
    /// # Returns
    /// A new `VecSet<T>`: `self ∩ other`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// use lib_rapid::compsci::general::BinaryInsert; // Used for "set!"
    /// 
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10,11]);
    /// let s2: VecSet<i32> = VecSet::new(&vec![0,1,2,3,11,0,0,0]);
    /// 
    /// let c:  VecSet<i32> = s.intersection(&s2);
    /// assert_eq!(c, set!(0, 1, 2, 3, 11));
    /// ```
    #[must_use]
    pub fn intersection(&self, other: &VecSet<T>) -> VecSet<T> {
        let mut res: VecSet<T> = self.clone();
        res.elements.retain(|x| other.elements.binary_search(x).is_ok());

        res
    }

    /// Checks for disjointness between `self` and `other`.
    /// # Arguments
    /// * `other` - The other set to be checked for disjointness.
    /// # Returns
    /// A `bool`
    /// # Examples
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    /// let s2 = set!(0, 1, 2, 3, 4, 5, 6,);
    /// let s3 = set!(8, 9, 10, 11, 12, 13, 14);
    /// let s4 = set!(9, 10, 11, 12, 13, 14, 15);
    /// 
    /// assert_eq!(false, s1.is_disjoint_with(&s2));
    /// assert_eq!(false, s1.is_disjoint_with(&s3));
    /// assert_eq!(true, s1.is_disjoint_with(&s4));
    /// ```
    #[must_use]
    pub fn is_disjoint_with(&self, other: &VecSet<T>) -> bool {
        for i in &self.elements {
            if other.has_element(*i)
            { return false; }
        }
        true
    }

    /// Checks for mathematical difference between two sets - `A \ B`.
    /// # Arguments
    /// * `other` - The other set to be checked for.
    /// # Examples
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(1, 2, 3, 4, 5);
    /// let s2 = set!(3, 4, 5);
    /// 
    /// assert_eq!(set!(1, 2), s1.difference_with(&s2));
    ///```
    #[must_use]
    pub fn difference_with(&self, other: &VecSet<T>) -> VecSet<T> {
        let mut res_vec: Vec<T> = Vec::with_capacity(std::cmp::max(self.cardinality(), other.cardinality()));
        for i in &self.elements {
            if !other.has_element(*i)
            { res_vec.push(*i); }
        }

        VecSet { elements: res_vec,
                 superset: None }
    }
    /// Gets the symmetric difference (Elements either in `self` or `other`, but not in both).
    /// # Arguments
    /// * `other` - A `&VecSet<T>`.
    /// # Examples
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(1, 2, 3, 4, 5);
    /// let s2 = set!(3, 4);
    /// 
    /// assert_eq!(set!(1, 2, 5), s1.symmetric_difference_with(&s2));
    ///```
    #[must_use]
    pub fn symmetric_difference_with(&self, other: &VecSet<T>) -> VecSet<T> {
        let diff1 = self.difference_with(other);
        let diff2 = other.difference_with(self);

        VecSet { elements: (diff1.clone().union(&diff2).collect::<Vec<_>>()),
                 superset: None }
    }
    /// Lets you check for an element in a set.
    ///
    /// # Arguments
    /// * `elem` - The element to check for.
    ///
    /// # Returns
    /// A boolean value which determines if `elem ∈ self`. 
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// 
    /// assert_eq!(false, set.has_element(7));
    /// assert_eq!(false, set.has_element(-1));
    /// assert_eq!(true, set.has_element(1));
    /// ```
    #[must_use]
    pub fn has_element(&self, elem: T) -> bool {
        self.elements.binary_search(&elem).is_ok()
    }
    /// Lets you insert an element into a set. Does not insert already present values.
    ///
    /// # Arguments
    /// * `elem` - The element to insert.
    ///
    /// # Returns
    /// Nothing.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// let mut s: VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// 
    /// s.insert(5);
    /// assert_eq!(s.elements(), &vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// ```
    pub fn insert(&mut self, elem: T) {
        self.elements.binary_insert_no_dup(elem)
    }
    /// Lets you check wether a set has a parent (emerged from another set) or not.
    ///
    /// # Returns
    /// A boolean value which determines if the set is a subset of any other set.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// let subset = VecSet::new_subset(&set, |x| x % 2 == 0);
    /// 
    /// assert_eq!(true, subset.has_superset());
    /// assert_eq!(false, set.has_superset());
    /// ```
    #[must_use]
    pub fn has_superset(&self) -> bool {
        self.superset.is_some()
    }
    /// Gets you the optional superset.
    ///
    /// # Returns
    /// A `Option<&VecSet<T>>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// let subset = VecSet::new_subset(&set, |x| x % 2 == 0);
    /// 
    /// assert_eq!(&set, subset.get_superset().unwrap());
    /// ```
    #[must_use]
    pub fn get_superset(&self) -> Option<&VecSet<T>> {
        self.superset
    }
    /// Gets the cardinality of a set.
    ///
    /// # Returns
    /// A `usize`: `|self|`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// 
    /// assert_eq!(7, set.cardinality());
    /// ```
    #[must_use]
    pub fn cardinality(&self) -> usize {
        self.elements.len()
    }
    /// Lets you set the elements of a set.
    ///
    /// # Arguments
    /// * `vals` - The Vec to change the values to.
    ///
    /// # Returns
    /// Nothing. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let mut set = set!(0, 1, 2, 3, 4, 5, 6);
    /// set.set_elements(&vec![0, 2, 4, 6]);
    /// 
    /// assert_eq!(&vec![0, 2, 4, 6], set.elements());
    /// ```
    pub fn set_elements(&mut self, vals: &[T]) {
        self.elements = vals.to_vec();
        self.elements.sort_unstable();
    }
    /// Lets you get the elements of a set.
    ///
    /// # Arguments
    /// * none
    ///
    /// # Returns
    /// A `&[T]` containing all elements. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let mut set = set!(0, 1, 2, 3, 4, 5, 6);
    /// 
    /// assert_eq!(&vec![0, 1, 2, 3, 4, 5, 6], set.elements());
    /// ```
    #[must_use]
    pub fn elements(&self) -> &[T] {
        &self.elements
    }
}

/// Creates a new `VecSet` more elegantly from values.
///
/// # Returns
/// A new `VecSet`.
/// # Examples
/// ```
/// use lib_rapid::set;
/// use lib_rapid::math::sets::vec_sets::VecSet; 
/// 
/// let set: VecSet<i32> = set!(0,1,2,3,4,5,6,-1);
/// assert_eq!(set.to_string(), "{ -1; 0; 1; 2; 3; 4; 5; 6 }");
/// assert_eq!(set.to_full_string(), "{ -1; 0; 1; 2; 3; 4; 5; 6 }");
#[macro_export]
#[must_use]
macro_rules! set {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::compsci::general::BinaryInsert;
            let mut temp = Vec::new();
            $(
                temp.binary_insert_no_dup($a);
            )*
            VecSet::new(&temp)
        }
    };
}
pub use set;

use crate::compsci::general::BinaryInsert;

impl<T: ToString> VecSet<'_, T> {
    /// Lets you print a set with all its parents recursively.
    ///
    /// # Returns
    /// Nothing. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: VecSet<i32> = VecSet::new_subset(&s, |x| x % 2 == 0);
    /// let s2: VecSet<i32> = VecSet::new_subset(&s1, |x| x == 4);
    /// 
    /// s2.full_println(); // Prints this set and the superset, see to_full_string.
    /// println!("{}", s2); // Only prints this set
    /// ```
    pub fn full_println(&self) {
        println!("{}", self.rec_to_string(&mut String::new()));
    }
    /// Converts a set with all subsets to a string.
    ///
    /// # Returns
    /// A String containing the result. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: VecSet<i32> = VecSet::new_subset(&s, |x| x % 2 == 0);
    /// let s2: VecSet<i32> = VecSet::new_subset(&s1, |x| x == 4);
    ///
    /// assert_eq!(s2.to_full_string(), "{ 4 } ⊆ { 0; 2; 4; 6; 8; 10 } ⊆ { 0; 1; 2; 3; 4; 5; 6; 7; 8; 9; 10 }".to_string());
    /// ```
    #[must_use]
    pub fn to_full_string(&self) -> String {
        self.rec_to_string(&mut String::new())
    }

    fn rec_to_string(&self, string: &mut String) -> String {
        string.push_str(&self.to_string()); // The child-set at the bottom
        if let Some(x) = self.superset { string.push_str(" ⊆ "); // Add subset-character
        x.rec_to_string(string); } // Recursively append parent sets
        string.to_string()
    }
}

// Indexing for Sets
impl<T> std::ops::Index<usize> for VecSet<'_, T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

// Implement Printing
impl<T: ToString> std::fmt::Display for VecSet<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::from('{');
        for elem in &self.elements {
            res.push(' ');
            res += &elem.to_string();
            res.push(';');
        }
        res.pop();
        write!(f, "{} }}", res)
    }
}

// Implement Equality
impl<T: PartialEq> PartialEq for VecSet<'_, T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<T: Copy> Iterator for VecSet<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elements.get(0) {
            Some(x) => { Some(*x) }
            None    => { None }
        }
    }
}