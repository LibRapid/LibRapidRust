#![allow(dead_code)]
#![allow(unused_macros)]

const INV_DIM: &str = "Error: Dimensions did not match.";

/**
Mathematical Vectors in Rust.
*/
pub struct MathVector {
    values:    Vec<f64>,
    dimension: usize,
    length:    f64,
}

impl MathVector {
    /**
    Creates a new `MathVector`.

    # Arguments
    * `values` - The values for the new MathVector.

    # Returns
    A new MathVector.
    */
    pub fn new(values: &Vec<f64>) -> MathVector {        
        MathVector { values:    values.clone(),
                     dimension: values.len(),
                     length:    -1f64 }
    }
    /**
    Creates a new `MathVector` with the specified capacity.

    # Arguments
    * `dim` - The dimension for the new MathVector.

    # Returns
    A new MathVector with length 0.
    */
    pub fn new_with_dimension(dim: usize) -> MathVector {        
        let mut vals: Vec<f64> = Vec::with_capacity(dim);
        (0..=dim).for_each(|_| { vals.push(0f64) });

        MathVector { values:    vals,
                     dimension: dim,
                     length:    -1f64 }
    }
    /**
    Gets the dimension in which a `MathVector` lives.

    # Returns
    A `usize`.
    */
    pub fn dimension(self: &Self) -> usize {
        self.dimension
    }
    /**
    Gets the length of a `MathVector`.

    # Returns
    A `f64`.
    */
    pub fn length(self: &mut Self) -> f64 {
        match self.length < 0f64 {
            true  => { let mut len: f64 = 0f64; 
                       for i in &self.values {
                           len += i * i;
                       }
                       self.length = len.sqrt();
                       return self.length;
                     }
            false => { return self.length; }

        }
    }
    /**
    Gets the values of a `MathVector`.

    # Returns
    A `Vec<f64>`.
    */
    pub fn get_values(self: &Self) -> &'_ Vec<f64> {
        &self.values
    }
    /**
    Sets the values of a `MathVector`.

    # Arguments
    * `vals` - The Vector of the new values.

    # Panic
    Panics if the values don't have the same dimension as before.
    */
    pub fn set_values(self: &mut Self, vals: &Vec<f64>) {
        match vals.len() == self.dimension {
            true  => { self.values = vals.clone();
                       let mut len: f64 = 0f64; 
                       for i in &self.values {
                           len += i * i;
                       }
                       self.length = len.sqrt(); }
            false => { core::panic!("{}", INV_DIM); } 
        }
    }
}

impl std::ops::Add for MathVector {
    type Output = Self;
    fn add(self, other: Self) -> MathVector {
        match self.dimension() == other.dimension {
            true  =>  { 
                let mut vals: Vec<f64> = Vec::with_capacity(self.dimension);
                for i in 0..self.dimension {
                    vals.push(self.values[i] + other.values[i]);
                }
                MathVector { values:    vals,
                             dimension: self.dimension,
                             length:    -1f64 }
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
                let mut vals: Vec<f64> = Vec::with_capacity(self.dimension);
                for i in 0..self.dimension() {
                    vals.push(self.values[i] - other.values[i]);
                }
                MathVector { values:    vals,
                             dimension: self.dimension,
                             length:    -1f64 }
            }
            false => { core::panic!("{}", INV_DIM) }
        }
    }
}
    /**
    Multiplies a `MathVector` with a scalar product.

    # Arguments
    * `scalar` - The scalar product.
    * `other` - The `MathVector` for the calculation.

    # Returns
    A new `MathVector`.
    */
pub fn scalar_mul(scalar: f64, other: &MathVector) -> MathVector {

    let mut vals: Vec<f64> = Vec::with_capacity(other.dimension);

    for i in 0..other.dimension {
        vals.push(scalar * other.values[i]);
    }
    MathVector { values:    vals,
                 dimension: other.dimension,
                 length:    -1f64 }
}

    /**
    Creates a new `MathVector` more elegantly from values..

    # Returns
    A new `MathVector`.
    */
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

#[test]
fn test_macro() {
    println!("{}", new_mathvec!(2,2,2));
}

impl std::fmt::Display for MathVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut finstring: String = "( ".to_owned();
        for i in 0..self.dimension {
            finstring =  " ".to_owned() + &finstring + &self.values[i].to_string() + "; ";
        }
        finstring.pop();
        finstring.pop();
        finstring = finstring + " )";
        write!(f, "{}", finstring)
    }
}

impl PartialEq for MathVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}