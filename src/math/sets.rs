//! Diving deeper into mathematics, huh? In here you'll find mathematical sets - Sometimes pretty handy!
/// Brings mathematical sets into Rust.
#[derive(Debug, Clone)]
pub struct Set<'a, T> {
    elements:    Vec<T>,
    superset:    Option<&'a Set<'a, T>>,
}

// Main impl
impl<'a, T: PartialEq + Copy + Ord> Set<'a, T> {
    /// Creates a new Set.
    ///
    /// # Arguments
    /// * `values` - The values for the set.
    ///
    /// # Returns
    /// A new set.
    pub fn new(values: &Vec<T>) -> Set<'a, T> {
        Set { elements:    values.clone(),
              superset:    None,
            }
    }
    /// Creates a new Set using a parent-Set to which it applies a closure.
    ///
    /// # Arguments
    /// * `parent` - The Set from which the new set emerges.
    /// * `f` - The closure after which the new set is created.
    ///
    /// # Returns
    /// A child Set.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::Set;
    /// let test1:       Set<u8> = Set::new(&vec![0,1,2,3,4]);
    /// let from_parent: Set<u8> = Set::<u8>::new_subset(&test1, |x| x % 2 == 0);
    /// assert_eq!(from_parent, Set::new(&vec![0,2,4]))
    /// ```
    pub fn new_subset<F: Fn(T) -> bool>(parent: &'a Set<T>, f: F) -> Set<'a, T> {
            let mut res: Set<T> = Set { elements:    Vec::new(),
                                        superset:    Option::Some(parent),
            };
            for elem in &parent.elements {
                if f(*elem) {
                    res.elements.push(elem.clone());
                }
            }
            res
    }
    /// Does a mathematical union on two sets.
    ///
    /// # Arguments
    /// * `self` - The first set.
    /// * `other` - The second set.
    ///
    /// # Returns
    /// A new set containing the union of both sets.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::Set;
    /// use lib_rapid::math::sets::new_set;
    /// let s:  Set<i32> = Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: Set<i32> = Set::new(&vec![11,12,13,13,11,0,0,0]);
    /// 
    /// let c:  Set<i32> = s.union(&s1);
    /// assert_eq!(c, new_set!(0,1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    pub fn union(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = Set {elements:    Vec::new(),
                                   superset:    None,
        };

        res.elements.append(&mut self.elements.clone());
        res.elements.append(&mut other.elements.clone());

        res.elements.sort(); 
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
    /// A new set containing the intersection of both sets.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::Set;
    /// use lib_rapid::math::sets::new_set;
    /// let s:  Set<i32> = Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s2: Set<i32> = Set::new(&vec![0,1,2,3,11,0,0,0]);
    /// 
    /// let c:  Set<i32> = s.intersection(&s2);
    /// assert_eq!(c, new_set!(0, 1, 2, 3));
    /// ```
    pub fn intersection(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = self.clone();

        for _ in &self.elements {
            res.elements.retain(|x| other.elements.contains(x));
        }
        res
    }
    /// Lets you check for an element in a set.
    ///
    /// # Arguments
    /// * `elem` - The element to check for.
    ///
    /// # Returns
    /// A boolean value which determines if the element is in the set. 
    pub fn has_element(&self, elem: &T) -> bool {
        self.elements.contains(elem)
    }
}

/// Creates a new `Set` more elegantly from values.
///
/// # Returns
/// A new `Set`.
/// # Examples
/// ```
/// use lib_rapid::new_set;
/// use lib_rapid::math::sets::Set;
/// let set: Set<i32> = new_set!(0,1,2,3,4,5,6,-1);
/// assert_eq!(set.to_string(), "{ 0; 1; 2; 3; 4; 5; 6; -1 }");
/// assert_eq!(set.to_full_string(), "{ 0; 1; 2; 3; 4; 5; 6; -1 }");
#[macro_export]
macro_rules! new_set {
    ( $( $a:expr ),* ) => {
        {
        let mut temp = Vec::new();
        $(
            temp.push($a);
        )*
        Set::new(&temp)
        }
    };
}
pub use new_set;

impl<T> Set<'_, T> {
    /// Lets you check wether a set has a parent or not.
    ///
    /// # Returns
    /// A boolean value which determines if the set has a value. 
    pub fn has_superset(&self) -> bool {
        self.superset.is_some()
    }
    /// Gets the cardinality of a set.
    ///
    /// # Returns
    /// A `usize`. 
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
    pub fn set_elements(&mut self, vals: Vec<T>) {
        self.elements    = vals;
    }
    /// Lets you get the elements of a set.
    ///
    /// # Arguments
    /// * none
    ///
    /// # Returns
    /// A `&Vec<T>` containing all elements. 
    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }
}

impl<T: ToString> Set<'_, T> {
    /// Lets you print a set with all its parents recursively.
    ///
    /// # Returns
    /// Nothing. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::Set;
    /// let s:  Set<i32> = Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: Set<i32> = Set::new_subset(&s, |x| x % 2 == 0);
    /// let s2: Set<i32> = Set::new_subset(&s1, |x| x == 4);
    /// 
    /// s2.full_print();
    /// println!("{}", s2);
    /// assert_eq!(s2.to_full_string(), "{ 4 } ⊆ { 0; 2; 4; 6; 8; 10 } ⊆ { 0; 1; 2; 3; 4; 5; 6; 7; 8; 9; 10 }".to_string());
    /// ```
    pub fn full_print(&self) {
        print!("{}\n", self.rec_to_string(&mut String::new()));
    }
    /// Converts a set with all subsets to a string.
    ///
    /// # Returns
    /// A String containing the result. 
    /// # Examples
    /// See `full_print()`.
    pub fn to_full_string(&self) -> String {
        self.rec_to_string(&mut String::new())
    }

    fn rec_to_string(&self, string: &mut String) -> String {
        string.push_str(&self.to_string()); // The child-set at the bottom
        match self.superset.is_some() {
            true  => { string.push_str(" ⊆ "); // Add subset-character
                       self.superset.unwrap().rec_to_string(string); } // Recursively append parent sets
            false => { }
        }
        string.to_string()
    }
}

// Indexing for Sets
impl<T> std::ops::Index<usize> for Set<'_, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.elements.get(index).unwrap()
    }
}

// Implement Printing
impl<T: ToString> std::fmt::Display for Set<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = "{".to_owned();
        for elem in &self.elements {
            res += &(" ".to_owned() + &elem.to_string() + ";");
        }
        res.pop();
        res.push(' ');
        write!(f, "{}}}", res)
    }
}

// Implement Equality
impl<T: PartialEq> PartialEq for Set<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }

    fn ne(&self, other: &Self) -> bool {
        self.elements != other.elements
    }
}