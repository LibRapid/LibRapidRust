use num::Num;
use num_traits::NumOps;

struct Vector<T: NumOps + Copy> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: NumOps + Copy> Vector<T> {
    fn get_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}