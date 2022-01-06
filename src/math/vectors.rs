//! Vectors can be really handy, sometimes. Do everything you want with your favorite direction pointing data type from mathematics.

use super::general::{CommonPowers, Increment};
const INV_DIM: &str = "Error: Dimensions did not match.";

/// Mathematical Vectors in Rust.
pub struct MathVector {
    values: Vec<f64>,
    length: Option<f64>,
}

impl MathVector {
    /// Creates a new `MathVector`.
    ///
    /// # Arguments
    /// * `values` - The values for the new MathVector.
    ///
    /// # Returns
    /// A new MathVector.
    pub fn new(values: &[f64]) -> MathVector {        
        MathVector { values: values.to_owned(),
                     length: None }
    }
    /// Creates a new `MathVector` with the specified capacity.
    ///
    /// # Arguments
    /// * `dim` - The dimension for the new MathVector.
    ///
    /// # Returns
    /// A new MathVector with length 0.
    pub fn new_with_dimension(dim: usize) -> MathVector {

        MathVector { values: vec![0f64; dim],
                     length: None }
    }
    /// Gets the dimension in which a `MathVector` lives.
    ///
    /// # Returns
    /// A `usize`.
    pub fn dimension(&self) -> usize {
        self.values.len()
    }
    /// Gets the length of a `MathVector`.
    ///
    /// # Returns
    /// A `f64`.
    pub fn length(&mut self) -> f64 {
        match self.length {
            None      => { let mut len: f64 = 0f64; 
                           for i in &self.values {
                               len.inc_by(i.square());
                           }
                           len         = len.sqrt();
                           self.length = Some(len);
                           return len;
                         }
            Some(len) => { return len; }

        }
    }
    /// Gets the values of a `MathVector`.
    ///
    /// # Returns
    /// A `&Vec<f64>`.
    pub fn get_values(&self) -> &Vec<f64> {
        &self.values
    }
    /// Sets the values of a `MathVector`.
    ///
    /// # Arguments
    /// * `vals` - The Vector of the new values.
    ///
    /// # Panic
    /// Panics if the values don't have the same dimension as before.
    pub fn set_values(&mut self, vals: &[f64]) {
        match vals.len() == self.dimension() {
            true  => { self.values = vals.to_owned();
                       self.length = None; }
            false => { core::panic!("{}", INV_DIM); } 
        }
    }

    /// Normalises a `MathVector`.
    ///
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// use lib_rapid::math::vectors::new_mathvec;
    /// let mut v = new_mathvec!(2,2,2);
    /// v.normalise(); // Also sets the Length to 1.0.
    /// assert_eq!(v.length(), 1.0);
    /// assert_eq!(v.get_values(), &vec!(0.5773502691896258; 3));
    /// ```
    pub fn normalise(&mut self) {
        *self = scalar_mul(1.0 / self.length(), self);
        self.length = Some(1.0);
    }
}

impl std::ops::Add for MathVector {
    type Output = Self;
    fn add(self, other: Self) -> MathVector {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals: Vec<f64> = Vec::with_capacity(self.dimension());
                for i in 0..self.dimension() {
                    vals.push(self.values[i] + other.values[i]);
                }
                MathVector { values: vals,
                             length: None }
            }
            false => { core::panic!("{}", INV_DIM) }
        }
    }
}

impl std::ops::Sub for MathVector {
    type Output = Self;
    fn sub(self, other: Self) -> MathVector {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals: Vec<f64> = Vec::with_capacity(self.dimension());
                for i in 0..self.dimension() {
                    vals.push(self.values[i] - other.values[i]);
                }
                MathVector { values: vals,
                             length: None }
            }
            false => { core::panic!("{}", INV_DIM) }
        }
    }
}
    /// Multiplies a `MathVector` with a scalar product.
    ///
    /// # Arguments
    /// * `scalar` - The scalar product.
    /// * `other` - The `MathVector` for the calculation.
    ///
    /// # Returns
    /// A new `MathVector`.
pub fn scalar_mul(scalar: f64, other: &MathVector) -> MathVector {

    let mut vals: Vec<f64> = Vec::with_capacity(other.dimension());

    for i in 0..other.dimension() {
        vals.push(scalar * other.values[i]);
    }
    MathVector { values: vals,
                 length: None }
}

/// Creates a new `MathVector` more elegantly from values.
///
/// # Returns
/// A new `MathVector`.
#[macro_export]
macro_rules! new_mathvec {
    ( $( $a:expr ),* ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($a as f64);
            )*
            MathVector::new(&temp)
        }
    };
}

pub use new_mathvec;

impl std::fmt::Display for MathVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut finstring: String = String::from('(');
        for i in 0..self.dimension() {
            finstring.push(' ');
            finstring += &self.values[i].to_string();
            finstring.push_str("; ");
        }
        finstring.drain(finstring.len()-2..finstring.len());
        write!(f, "{} )", finstring)
    }
}

impl PartialEq for MathVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}