use std::ops::Index;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ElemPair<T>([T]);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Set<T> {
    values: Vec<T>,
    cardinality: usize,
}

// region impl for Set
pub fn new_set(values: Vec<T>) -> Set<T> {
    Set { values, cardinality: values.len() }
}

// Indexing for Sets
impl Index<T> for Set<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.values[index]
    }
}

// Implement Printing
impl std::fmt::Display for Set<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = "{";
        for elem in self.values {
            res += " " + elem + ";";
        }
        write!(f, "{}", res)
    }
}

// endregion impl for Set