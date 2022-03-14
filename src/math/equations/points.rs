//! In here you'll find the definition for cartesian points.
#[derive(Copy, Clone, PartialEq)]
/// A cartesian point.
pub struct Point<const N: usize> {
    /// The coordinates of the point.
    pub coordinates: [f64; N]
}

impl<const N: usize> Point<N> {
    /// Creates a new `Point`.
    /// # Arguments
    /// * `vals: [f64; N]` - The slice of values.
    /// # Returns
    /// A new `Point`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::points::Point;
    /// use lib_rapid::new_point;
    /// 
    /// let p1 = new_point!(1.2, 1.4, 1.3);
    /// let p2 = Point::new([1.2, 1.4, 1.3]);
    /// 
    /// assert!(p1 == p2);
    /// ```
    pub fn new(vals: [f64; N]) -> Point<N> {
        Point::<N> {
            coordinates: vals
        }
    }
}

impl<const N: usize> std::ops::Index<usize> for Point<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl<const N: usize> std::fmt::Display for Point<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(N << 2);
        s.push_str("(");
        for v in self.coordinates {
            s.push_str(&v.to_string());
            s.push_str(", ")
        }
        s.pop();
        s.pop();
        write!(f, "{} )", s)
    }
}

impl<const N: usize> std::fmt::Debug for Point<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(N << 2);
        s.push_str("( ");
        for v in self.coordinates {
            s.push_str(&v.to_string());
            s.push_str(", ")
        }
        s.pop();
        s.pop();
        write!(f, "Point {} ) with dimension {}", s, N)
    }
}
/// Creates a new `Point` more elegantly from values.
///
/// # Returns
/// A new `Point`.
/// # Examples
/// ```
/// use lib_rapid::math::equations::points::Point;
/// use lib_rapid::new_point;
/// 
/// let p1 = new_point!(1.2, 1.4, 1.3);
/// let p2 = Point::new([1.2, 1.4, 1.3]);
/// 
/// assert!(p1 == p2);
/// ```
#[macro_export]
#[must_use]
macro_rules! new_point {
    ( $( $a:expr ),* ) => {
        {
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.push($a);
            )*
            Point::new(&res_vec);
        }
    };
}
pub use new_point;