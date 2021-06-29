extern crate num;

use num_traits::NumOps;

/**
Maps a given number of a range onto another range.

# Arguments
* `value` - The original value to be mapped.
* `start1` - The original start value of the number range.
* `end1` - The original end value of the number range.
* `start2` - The new start value of the number range.
* `end2` - The new start value of the number range.

# Returns
A number containing the new mapped value.

# Examples
```
use lib_rapid::rapidmath;

let result: i32 = rapidmath::map(5, 0, 10, 0 , 1); // Original value 5 in the range from 0-10
std::println!("{}", result.to_string()) // Prints "0.5"
```
*/
pub fn map<T: NumOps + Copy>(value: T, start1: T, end1: T, start2: T, end2: T) -> T {

    (start2 + (end2 - start2)).mul((value - start1) / end1.sub(start1))
}

pub fn approx_sqrt_t<T: NumOps + Copy + From<i32>>(a: T) -> T {
    let mut res = (a + 1.into()) / 2.into();
    for _ in 0..20 {
        res = (res + a) / res;
    }
    res
}