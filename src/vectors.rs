use num_traits::NumOps;

use crate::rapidmath::approx_sqrt_t;

struct Vector<T: NumOps + Copy> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: NumOps + Copy + From<i32>> Vector<T> {
    fn get_length(&self) -> T {
        let x = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;

        approx_sqrt_t(x)
    }
}