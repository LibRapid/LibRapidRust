use super::general::Decrement;

// Binary gcd
pub(crate) fn gcd(mut a: u64, mut b: u64) -> u64 {
    
    if b == 0 
    { return a; } 

    if a == 0
    { return b; }

    let a_two_factor = a.trailing_zeros();  
    let b_two_factor = b.trailing_zeros(); 
    let min_two_factor = std::cmp::min(a_two_factor, b_two_factor);
    a >>= a_two_factor;
	b >>= b_two_factor;
    loop {
         
        if b > a
        { std::mem::swap(&mut b, &mut a); }

        a.dec_by(b);

        if a == 0 
        { return b << min_two_factor; }

        a >>= a.trailing_zeros();
    }
  }


// Extended Euclidean algorithm
pub(crate) fn eea(p: i64 , q: i64) -> (i64, i64, i64) {
    let mut gcd:      i64 = p; 
    let mut new_r:    i64 = q;
    let mut bezout_1: i64 = 1;
    let mut new_s:    i64 = 0;
    let mut bezout_2: i64 = 0;
    let mut new_t:    i64 = 1;
    
    while new_r != 0 {
        let quotient: i64 = gcd / new_r;
        let mut temp: i64 = new_r;
        new_r = gcd - quotient * temp;
        gcd = temp;
        
        temp     = new_s;
        new_s    = bezout_1 - quotient * temp;
        bezout_1 = temp;
        
        temp     = new_t;
        new_t    = bezout_2 - quotient * temp;
        bezout_2 = temp;
    }

    (gcd, bezout_1, bezout_2)
}
