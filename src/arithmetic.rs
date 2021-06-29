/**
Divides a given 32 bit integer by a power of 2. If the result is a fraction, the result is rounded.

# Arguments
* `divident` - A `i32` which is to be divided.
* `power` - `i32` which will be the divisor. (2^k where k is the power)

# Returns
a i32 containing the result of the calculation.

# Examples
```
use lib_rapid::arithmetic;

let result: i32 = arithmetic::div_pow2(10, 1); // 10 divided by 2
std::println!("{}", result.to_string()) // Prints "5"
```
*/
pub fn div_pow2(divident: i32, power: i32) -> i32 {
    &divident >> &power
}

pub fn mult_pow2(multiplier: i32, power: i32) -> i32 {
    &multiplier << &power
}