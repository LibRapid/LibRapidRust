//! Vectors can be really handy, sometimes. Do everything you want with your favorite direction pointing data type from mathematics.

use super::general::Increment;
const INV_DIM: &str = "Error: Dimensions did not match.";

/// Mathematical Vectors in Rust.
pub struct MathVector<T> {
    values: Vec<T>,
    length: Option<f64>,
}

impl<T: Copy +
        super::general::CommonPowers +
        From<f64> +
        std::ops::Mul<Output = T>> MathVector<T>
        where
        f64: From<T> {
    /// Creates a new `MathVector`.
    ///
    /// # Arguments
    /// * `values` - The values for the new MathVector.
    ///
    /// # Returns
    /// A new MathVector.
    #[must_use]
    pub fn new(values: &[T]) -> MathVector<T> {        
        MathVector { values: values.to_owned(),
                     length: None }
    }
    /// Creates a new `MathVector` with the specified capacity.
    ///
    /// # Arguments
    /// * `dim` - The dimension for the new MathVector.
    ///
    /// # Returns
    /// A new `MathVector<f64>` with length 0.
    #[must_use]
    pub fn new_with_dimension(dim: usize) -> MathVector<f64> {

        MathVector { values: vec![0.0; dim],
                     length: None }
    }
    /// Gets the dimension in which a `MathVector` lives.
    ///
    /// # Returns
    /// A `usize`.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.values.len()
    }
    /// Gets the length of a `MathVector`.
    ///
    /// # Returns
    /// A `f64`.
    #[must_use]
    pub fn length(&mut self) -> f64 {
        match self.length {
            None      => { let mut len: f64 = 0.0; 
                           for i in &self.values {
                               len.inc_by(i.square().into());
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
    /// A `&Vec<T>`.
    #[must_use]
    pub fn get_values(&self) -> &Vec<T> {
        &self.values
    }
    /// Sets the values of a `MathVector`.
    ///
    /// # Arguments
    /// * `vals` - The Vector of the new values.
    ///
    /// # Panic
    /// Panics if the values don't have the same dimension as before.
    pub fn set_values(&mut self, vals: &[T]) {
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
        *self = scalar_mul(self.length().recip(), self);
        self.length = Some(1.0);
    }
}

impl<T: Copy +
        super::general::CommonPowers +
        From<f64> + Into<f64> +
        std::ops::Mul<Output = T> +
        std::ops::Add<Output = T>>
        std::ops::Add for MathVector<T>
        where
        f64: From<T> {
    type Output = Self;
    fn add(self, other: Self) -> MathVector<T> {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals: Vec<T> = Vec::with_capacity(self.dimension());
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

impl<T: Copy +
        super::general::CommonPowers +
        From<f64> +
        Into<f64> +
        std::ops::Mul<Output = T> +
        std::ops::Sub<Output = T>>
        std::ops::Sub for MathVector<T>
        where
        f64: From<T> {
    type Output = Self;
    fn sub(self, other: Self) -> MathVector<T> {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals: Vec<T> = Vec::with_capacity(self.dimension());
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
/// A new `MathVector<T>`.
#[must_use]
pub fn scalar_mul<T: Copy +
                     super::general::CommonPowers +
                     From<f64> +
                     std::ops::Mul<Output = T>>
                     (scalar: f64, other: &MathVector<T>) -> MathVector<T> 
                     where 
                     f64: From<T> {

    let mut vals: Vec<T> = Vec::with_capacity(other.dimension());

    for i in 0..other.dimension() {
        vals.push(T::from(scalar) * other.values[i]);
    }
    MathVector { values: vals,
                 length: None }
}

/// Creates a new `MathVector` more elegantly from values.
///
/// # Returns
/// A new `MathVector<T>`.
#[macro_export]
#[must_use]
macro_rules! new_mathvec {
    ( $( $a:expr ),* ) => {
        {
            let mut temp: Vec<T> = Vec::new();
            $(
                temp.push($a);
            )*
            MathVector::new(&temp)
        }
    };
}

pub use new_mathvec;

impl<T: Copy +
        super::general::CommonPowers +
        Into<f64> +
        std::fmt::Display + From<f64> +
        std::ops::Mul<Output = T>>
        std::fmt::Display for MathVector<T> 
        where 
        f64: From<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut finstring: String = String::from("( ");
        for i in 0..self.dimension() {
            finstring += &self.values[i].to_string();
            finstring.push_str("; ");
        }
        finstring.drain(finstring.len()-2..finstring.len());
        write!(f, "{} )", finstring)
    }
}

impl<T: std::cmp::PartialEq> PartialEq for MathVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}