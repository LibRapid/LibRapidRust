use crate::math::fast_primes::fjprime64::fjprime64;
use crate::math::fast_primes::fjprime32::fjprime32;

/*
  This function branches into the fastest checks for each range
  
  Note that the small list and divisiblity are included in the checks for u8,u16,and all values over 2^64
  however the FJprime32/64 checks are only valid for a certain range. 
  
  This currently utilizes a compiler optimization trick, so trying to execute on unoptimized executable will result in overflow error. 

*/
#[allow(non_upper_case_globals)]
pub fn primality_128(x: u128) -> bool {

        const PRIMELIST: [u8; 54] = [// list of all primes less than 2^8
           2,  3,  5,   7,  11,  13,  17,  19,  23,  29,  31,
          37, 41, 43,  47,  53,  59,  61,  67,  71,  73,  79, 
          83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 
         139,149,151, 157, 163, 167, 173, 179, 181, 191, 193,
         197,199,211, 223, 227, 229, 233, 239, 241, 251
        ];
        
        const u8bound  : u128 = 0x100;
        const u16bound : u128 = 0x10000;
        const u32bound : u128 = 0x100000000;
        const u64bound : u128 = 0x10000000000000000;
        
        if x == 1 || x == 0
        { return false; }
         
        if x < u16bound || x >= u64bound {

       		for i in PRIMELIST { // you can't truncate x as multiple elements in 0;2^128  mod 2^8 are congruent to any element in 0;2^8 
    		    if x == i as u128 
                { return true; }
       		}
		   
        	if x < u8bound // If x < 2^8 end checks
            { return false; }

        	for i in PRIMELIST { // All x are checked for divisibility, majority of composites will fail 
        	  	if x % i as u128 == 0
                { return false; }
        	}

        	if x < u16bound // if less than 2^16 then checks stop here
            { return true; }
		
        } // ends the initial check for 0;2^16 and 2^64;2^128-1
        // checks are split there for the interval of 0;2^32 and 0;2^64 and then resumed for 0;2^128, so numbers greater than 2^64 never receive these checks
        
        if (x >= u16bound) && (x < u32bound)
        { return fjprime32(x as u32); }
        
        else if (x > u32bound) && (x < u64bound)
		{ return fjprime64(x as u64); }
        // numbers greater than 2^64 are checked here in addition to the previous trial division, but not the SPRP tests.
        else {
            const PRIMORIAL: u128 = 210;
            let supremum = ( ( (x as f64).sqrt() as u128 + 103u128) / PRIMORIAL) + 1u128; // Else perform trial division using the 11-rough numbers (higher-density of primes)
            for i in 1..supremum { // starts at 
            
                if x % (PRIMORIAL * i - 1) == 0 ||
                   x % (PRIMORIAL * i + 1) == 0
                { return false; } 
            
                for j in PRIMELIST[4..27].iter(){
                    if x % (PRIMORIAL * i - *j as u128) == 0 ||
                       x % (PRIMORIAL * i + *j as u128) == 0
                    { return false; }
                }
            }
            return true;
        }
}
    
