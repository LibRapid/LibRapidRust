/**
The conversion algorithm to be chosen. Used by `temp_conversion`.
*/
pub enum TempConversion {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,

    FahrenheitToKelvin,
    KelvinToFahrenheit,

    CelsiusToKelvin,
    KelvinToCelsius,
}

/**
Trait for left-shifting decimal-numbers.
*/
pub trait DecimalLeftShift<T> {
    /**
    Multiplies by 10 (shifts the decimal places to the left by 1) while being more efficient.

    # Arguments
    * `n` - The number to be multiplied by 10.

    # Returns
    The new shifted number.
    */
    fn dec_lshift(&self) -> T;
}

/**
 Trait for different mathematical means.
 */ 
pub trait Means<T> {
    /**
     The arithmetic mean of a given set of values.
    */ 
    fn arithmetic_mean(&self) -> T;
    /**
     The mode of a given set of values.
    */ 
    fn mode(&self) -> T;
    /**
     The median of a given set of values.
    */ 
    fn median(&self) -> f64;
}

/**
Trait for mapping numbers to another number range.
*/
pub trait MapToNumRange<T> {
    /**
    Maps a given number of a range onto another range.

    # Arguments
    * `start1` - The original start value of the number range.
    * `end1` - The original end value of the number range.
    * `start2` - The new start value of the number range.
    * `end2` - The new start value of the number range.

    # Returns
    A number containing the new mapped value.

    # Examples
    ```
    use lib_rapid::math::rapidmath::MapToNumRange;

    let result: f32 = 5f32.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    std::println!("{}", result.to_string()) // Prints "0.5"
    ```
    */
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T;
}

impl<T: std::ops::Add<Output = T> + 
        std::ops::Sub<Output = T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Div<Output = T> + 
        Copy> MapToNumRange<T> for T {
            fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T {
                (start2 + (end2 - start2)) * ((*self - start1) / end1 - start1)
            }
        }

impl<T: std::ops::Add<Output = T> + 
        std::ops::Shl<usize,Output = T> + 
        Copy> DecimalLeftShift<T> for T {
            fn dec_lshift (&self) -> T {
                (*self << 1) + (*self << 3)
            }
}

impl<T: std::ops::AddAssign +
        std::ops::Add<Output = T> +
        std::ops::Sub<Output = T> +
        std::ops::DivAssign +
        std::ops::Div<Output = f64> +
        Copy + 
        From<usize> +
        Into<f64> +
        std::cmp::Ord>
        Means<T> for Vec<T> {
    fn arithmetic_mean(&self) -> T {
        let mut res: T = self[0] - self[0];
        for i in self {
            res += *i;
        }

        res /= self.len().into();
        res
    }

    fn mode(&self) -> T {
        let mut temp: Vec<T> = self.clone();
        temp.sort_by(|a, b| b.cmp(a));
        temp[0]
    }

    fn median(&self) -> f64 {
        let mut temp: Vec<T> = self.clone();
        temp.sort();
        if temp.len() & 1 == 0 {
            return (temp[temp.len() / 2] + temp[temp.len() / 2 - 1]).into() / 2f64;
        } else {
            return temp[temp.len() / 2].into();
        }
    }
}

/**
Performs a temperature conversion.

# Arguments
* `mode` - The mode ( e.g. CelsiusToFahrenheit ).
* `value` - The value to be converted.

# Returns
A `f64` containing the result.
*/
pub fn temp_conversion(mode: TempConversion, value: &f64) -> f64 {
    match mode {
        TempConversion::CelsiusToFahrenheit => { return value * 1.8 + 32.0; }
        TempConversion::CelsiusToKelvin     => { return value + 273.15; }
        TempConversion::FahrenheitToCelsius => { return (value - 32.0) / 1.8; }
        TempConversion::FahrenheitToKelvin  => { return (value - 32.0) / 1.8 + 273.15; }
        TempConversion::KelvinToCelsius     => { return value - 273.15; }
        TempConversion::KelvinToFahrenheit  => { return (value - 273.15) * 1.8 + 32.0; }
    }
}