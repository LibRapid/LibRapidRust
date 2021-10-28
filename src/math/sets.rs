#[derive(Debug, Clone, PartialEq)]
pub struct Set<T> {
    values:          Vec<T>,
    cardinality:     usize,
}

pub fn new_set<T: Copy>(values: Vec<T>) -> Set<T> {

    Set { values:      values.clone(),
          cardinality: values.len(),
        }
}

// Main impl
impl<T: PartialEq + Copy + Ord> Set<T> {
    pub fn union(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = Set {values:      Vec::new(),
                                   cardinality: 0,
        };

        res.values.append(&mut self.values.clone());
        res.values.append(&mut other.values.clone());

        res.values.sort();
        res.values.dedup();
        res.cardinality = res.values.len();
        res
    }

    pub fn intersection(&self, other: &Set<T>) -> Set<T> {
        let mut res: Set<T> = self.clone();

        for e in &self.values {
            res.values.retain(|_| other.values.contains(&e));
        }
        res.cardinality = res.values.len();
        res
    }

    // I know that getters and setters are VERY controversial.
    // I'm not going to change it because the cardinality is only something
    // You'll need to read, not set.
    pub fn cardinality(&self) -> &usize {
        &self.cardinality
    }

    pub fn set_values(&mut self, vals: Vec<T>) {
        self.values = vals;
        self.cardinality = self.values.len();
    }

    pub fn values(&self) -> &Vec<T> {
        &self.values
    }
}

// Indexing for Sets
impl<T> std::ops::Index<usize> for Set<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.values.get(index).unwrap()
    }
}

// Implement Printing
impl<T: ToString> std::fmt::Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = "{".to_owned();
        for elem in &self.values {
            res = res + " [ " + &*elem.to_string() + " ] ;";
        }
        write!(f, "{}}}", res)
    }
}