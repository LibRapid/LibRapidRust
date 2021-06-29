#![allow(dead_code)]
use num_traits::NumOps;
/**
Creates a mathematical vector in up to 4 dimensions.

# Arguments
* `x` - The x dimensional value.
* `y` - The y dimensional value.
* `z` - The z dimensional value.
* `w` - The w dimensional value.

# Examples
```
```
*/
struct Vector<T: NumOps + Copy> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: NumOps + Copy> Vector<T> {
    
/**
Gets the length of the vector.

# Returns
The length of the vector.

# Examples
```
```
*/
    fn get_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}