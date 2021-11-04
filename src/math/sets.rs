/*
Brings mathematical sets into Rust.
*/
#[derive(Debug, Clone)]
pub struct Set<'a, T> {
    elements:        Vec<T>,
    cardinality:     usize,
    parent:          Option<&'a Set<'a, T>>,
}

// Main impl
impl<'a, T: PartialEq + Copy + Ord> Set<'a, T> {

    pub fn new(values: Vec<T>) -> Set<'a, T> {
        Set { elements:    values.clone(),
              cardinality: values.len(),
              parent:      None,
            }
    }
    /**
    Creates a new Set using a parent-Set to which it applies a closure.

    # Arguments
    * `parent` - The Set from which the new set emerges.
    * `f` - The closure after which the new set is created.

    # Returns
    A child Set.
    */
    pub fn new_from_parent<F: Fn(T) -> bool>(parent: &'a Set<T>, f: F) -> Set<'a, T> {
            let mut res: Set<T> = Set { elements:    Vec::new(),
                                        cardinality: 0,
                                        parent:      Option::Some(parent),
            };
            for elem in &parent.elements {
                if f(*elem) {
                    res.elements.push(elem.clone());
                }
            }
            res.cardinality = res.elements.len();
            res
    }
    /**
    Does a mathematical union on two sets.

    # Arguments
    * `self` - The first set.
    * `other` - The second set.

    # Returns
    A new set containing the union of both sets.
    */
    pub fn union(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = Set {elements:    Vec::new(),
                                   cardinality: 0,
                                   parent:      None,
        };

        res.elements.append(&mut self.elements.clone());
        res.elements.append(&mut other.elements.clone());

        res.elements.sort(); 
        res.elements.dedup();
        res.cardinality = res.elements.len();
        res
    }
    /**
    Does a mathematical intersection on two sets.

    # Arguments
    * `self` - The first set.
    * `other` - The second set.

    # Returns
    A new set containing the intersection of both sets.
    */
    pub fn intersection(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = self.clone();

        for e in &self.elements {
            res.elements.retain(|_| other.elements.contains(&e));
        }
        res.cardinality = res.elements.len();
        res
    }
    /**
    Lets you check for an element in a set.

    # Arguments
    * `elem` - The element to check for.

    # Returns
    A boolean value which determines if the element is in the set.
    */ 
    pub fn has_element(&self, elem: &T) -> bool {
        self.elements.contains(elem)
    }
}

impl<T> Set<'_, T> {
    /**
    Lets you check wether a set has a parent or not.

    # Returns
    A boolean value which determines if the set has a value.
    */ 
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
    /* I know that getters and setters are VERY controversial.
       I'm not going to change it because the cardinality is only something
       You'll need to read, not set.
    */
    pub fn cardinality(&self) -> &usize {
        &self.cardinality
    }
    /**
    Lets you set the elements of a set.

    # Arguments
    * `vals` - The Vec to change the values to.

    # Returns
    Nothing.
    */ 
    pub fn set_elements(&mut self, vals: Vec<T>) {
        self.elements    = vals;
        self.cardinality = self.elements.len();
    }
    /**
    Lets you get the elements of a set.

    # Arguments
    * none

    # Returns
    A `&Vec<T>` containing all elements.
    */ 
    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }
}

impl<T: ToString> Set<'_, T> {
    /**
    Lets you print a set with all its parents recursively.

    # Returns
    Nothing.
    */ 
    pub fn full_print(&self) {
        print!("{}", self.backend_full_print(&mut String::new()));
    }
    /**
    Converts a set with all subsets to a string.

    # Returns
    A String containing the result.
    */ 
    pub fn to_full_string(&self) -> String {
        self.backend_full_print(&mut String::new())
    }

    fn backend_full_print(&self, string: &mut String) -> String {
        string.push_str(&self.to_string()); // The child-set at the bottom
        match self.parent.is_some() {
            true  => { string.push_str(" ⊆ "); // Add subset-character
                       self.parent.unwrap().backend_full_print(string); } // Recursively append parent sets
            false => { string.push_str("\n"); }
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
        let mut res: String = "{ ".to_owned();
        for elem in &self.elements {
            res = res + "" + &*elem.to_string() + "; ";
        }
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