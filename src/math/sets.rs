use std::ops::Index;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct ElemPair<T>(Vec<T>);

#[derive(Debug, Clone, PartialEq)]
pub struct Set<T> {
    values: Vec<T>,
    cardinality: usize,
}

// region impl for Set
pub fn new_set<T: Copy>(values: Vec<T>) -> Set<T> {

    Set { values: values.clone(),
          cardinality: values.len(),
        }
}

// Main impl
impl<T: PartialEq + Copy + Ord> Set<T> {
    pub fn union(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = Set {values: Vec::new(),
                                   cardinality: 0,
        };

        res.values.append(&mut self.values.clone());
        res.values.append(&mut other.values.clone());

        res.values.sort(); // Sorting, because it's faster deduping than without
        res.values.dedup();
        res
    }
}

// Indexing for Sets
impl<T> Index<usize> for Set<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.values.get(index).unwrap()
    }
}

// Implement Printing
impl<T: ToString> std::fmt::Display for Set<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res: String = "{".to_owned();
        for elem in &self.values {
            res = res + " [ " + &*elem.to_string() + " ] ;";
        }
        write!(f, "{}", res)
    }
}

// endregion impl for Set