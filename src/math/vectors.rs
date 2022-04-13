//! Vectors can be really handy, sometimes. Do everything you want with your favorite direction pointing data type from mathematics.
use std::{ops::*, convert::TryInto};
use super::general::NumTools;
const INV_DIM: &str = "Error: Dimensions did not match.";

/// Mathematical Vectors in Rust.
#[derive(Clone, Debug)]
pub struct MathVector<const C: usize, T> {
    values: [T; C],
    length: Option<f64>,
}

impl<T: Copy +
        NumTools<T> +
        From<f64> +
        Mul<Output = T>, const C: usize> MathVector<C, T>
        where
        f64: From<T> {
    /// Creates a new `MathVector`.
    /// # Arguments
    /// * `values` - The values for the new MathVector.
    /// # Returns
    /// A new MathVector.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v = MathVector::new([2.0, 2.0, 2.0]);
    /// 
    /// assert_eq!(v.length(), 3.4641016151377544);
    /// ```
    #[must_use]
    pub fn new(values: [T; C]) -> MathVector<C, T> {        
        MathVector { values: values.try_into().unwrap(),
                     length: None }
    }
    /// Creates a new `MathVector` with the specified capacity.
    /// # Arguments
    /// * `dim` - The dimension for the new MathVector.
    /// # Returns
    /// A new `MathVector<f32>` with length 0.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v: MathVector<3, f64> = MathVector::new([0.0, 0.0, 0.0]);
    /// 
    /// assert_eq!(v, MathVector::new_with_dimension());
    /// ```
    #[inline]
    #[must_use]
    pub fn new_with_dimension() -> MathVector<C, T> {

        MathVector { values: [0.0.into(); C],
                     length: None }
    }
    /// Gets the dimension in which a `MathVector` lives.
    /// # Returns
    /// A `usize`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v = MathVector::new([2.0, 2.0, 2.0]);
    /// 
    /// assert_eq!(v.dimension(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.values.len()
    }
    /// Gets the length of a `MathVector`.
    /// # Returns
    /// A `f64`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v = MathVector::new([2.0, 2.0, 2.0]);
    /// 
    /// assert_eq!(v.length(), 3.4641016151377544);
    /// ```
    #[inline]
    #[must_use]
    pub fn length(&mut self) -> f64 {
        match self.length {
            None      => { let mut len: f64 = 0.0; 
                           self.values.iter().for_each(|x| len.inc_by(f64::from(*x).square()));
                           len         = len.sqrt();
                           self.length = Some(len);
                           len
                         }
            Some(len) => { len }

        }
    }
    /// Gets the values of a `MathVector`.
    /// # Returns
    /// A `&Vec<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v = MathVector::new([2.0, 2.0, 2.0]);
    /// 
    /// assert_eq!(v.get_values(), &vec!(2.0; 3));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_values(&self) -> &[T] {
        &self.values
    }
    /// Sets the values of a `MathVector`.
    /// # Arguments
    /// * `vals` - The Vector of the new values.
    /// # Panic
    /// Panics if the values don't have the same dimension as before.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v = MathVector::new([2.0, 2.0, 2.0]);
    /// v.set_values([1.0; 3]);
    /// 
    /// assert_eq!(v.get_values(), &vec!(1.0; 3));
    /// ```
    pub fn set_values(&mut self, vals: [T; C]) {
        match vals.len() == self.dimension() {
            true  => { self.values = vals;
                       self.length = None; }
            false => { panic!("{}", INV_DIM); } 
        }
    }

    /// Normalises a `MathVector`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::vectors::MathVector;
    /// 
    /// let mut v = MathVector::new([2.0, 2.0, 2.0]);
    /// v.normalise(); // Also sets the Length to 1.0.
    /// 
    /// assert_eq!(v.length(), 1.0);
    /// assert_eq!(v.get_values(), &vec!(0.5773502691896258; 3));
    /// ```
    #[inline]
    pub fn normalise(&mut self) {
        *self = scalar_mul(T::from(self.length().recip()), self);
        self.length = Some(1.0);
    }
}

impl<T: Copy +
        super::general::NumTools<T> +
        From<f64> + Into<f64> +
        Mul<Output = T> +
        Add<Output = T>, const C: usize>
        Add for MathVector<C, T>
        where
        f64: From<T> {
    type Output = Self;
    fn add(self, other: Self) -> MathVector<C, T> {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals = [0.0.into(); C];
                for i in 0..self.dimension() {
                    vals[i] = self.values[i] + other.values[i];
                }
                MathVector { values: vals,
                             length: None }
            }
            false => { panic!("{}", INV_DIM) }
        }
    }
}

impl<T: Copy +
        super::general::NumTools<T> +
        From<f64> +
        Into<f64> +
        Mul<Output = T> +
        Sub<Output = T>, const C: usize>
        Sub for MathVector<C, T>
        where
        f64: From<T> {
    type Output = Self;
    fn sub(self, other: Self) -> MathVector<C, T> {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals = [0.0.into(); C];
                for i in 0..self.dimension() {
                    vals[i] = self.values[i] - other.values[i];
                }
                MathVector { values: vals,
                             length: None }
            }
            false => { panic!("{}", INV_DIM) }
        }
    }
}
/// Multiplies a `MathVector` with a scalar product.
/// # Arguments
/// * `scalar` - The scalar product.
/// * `other` - The `MathVector` for the calculation.
/// # Returns
/// A new `MathVector<T>`.
/// # Examples
/// ```
/// use lib_rapid::math::vectors::MathVector;
/// use lib_rapid::math::vectors::scalar_mul;
/// 
/// let v = MathVector::new([1.0, 1.0, 1.0]);
/// 
/// assert_eq!(MathVector::new([2.0, 2.0, 2.0]), scalar_mul(2.0, &v));
/// ```
#[inline]
#[must_use = "This returns the result of the operation, without modifying the original."]
pub fn scalar_mul<T: Copy +
                     super::general::NumTools<T> +
                     From<f64> +
                     Mul<Output = T>, const C: usize>
                     (scalar: T, other: &MathVector<C, T>) -> MathVector<C, T> 
                     where 
                     f64: From<T> {

    let mut vals = [0.0.into(); C];

    for x in other.values.iter().enumerate() {
        vals[x.0] = *x.1 * scalar;
    }
    MathVector { values: vals,
                 length: None }
}

impl<T: Copy +
        super::general::NumTools<T> +
        Into<f64> +
        std::fmt::Display + From<f64> +
        Mul<Output = T>, const C: usize>
        std::fmt::Display for MathVector<C, T> 
        where 
        f64: From<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut finstring: String = String::from("( ");
        for i in 0..self.dimension() {
            finstring.push_str(&(self.values[i].to_string() + "; "));
        }
        finstring.drain(finstring.len()-2..finstring.len());
        write!(f, "{} )", finstring)
    }
}

impl<T: PartialEq, const C: usize> PartialEq for MathVector<C, T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}