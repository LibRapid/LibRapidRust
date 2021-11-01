#[derive(Debug, Clone, PartialEq)]
pub struct Set<T> {
    elements:        Vec<T>,
    cardinality:     usize,
}

// Main impl
impl<T: PartialEq + Copy + Ord> Set<T> {

    pub fn new(values: Vec<T>) -> Set<T> {
        Set { elements:    values.clone(),
              cardinality: values.len(),
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
    pub fn new_from_parent<F: Fn(T) -> bool>(parent: &Set<T>, f: F) -> Set<T>
    where
        F: Fn(T) -> bool {
            let mut res: Set<T> = Set {elements:    Vec::new(),
                                       cardinality: 0,
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

    // I know that getters and setters are VERY controversial.
    // I'm not going to change it because the cardinality is only something
    // You'll need to read, not set.
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
        self.elements = vals;
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

// Indexing for Sets
impl<T> std::ops::Index<usize> for Set<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.elements.get(index).unwrap()
    }
}

// Implement Printing
impl<T: ToString> std::fmt::Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = "{ ".to_owned();
        for elem in &self.elements {
            res = res + "" + &*elem.to_string() + "; ";
        }
        write!(f, "{} }}", res)
    }
}