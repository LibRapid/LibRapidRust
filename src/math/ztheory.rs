//! Functions for the whole number set, ℤ.
pub(crate) fn modpow(x: u64, mut pow: u64, modulus: u64) -> u64 {  //upgrades to u128 to allow mul-mod
	let mut z:    u128 = 1;
  	let mut base: u128 = x.clone() as u128;
	let n = modulus as u128;
  	if pow == 0
    { return z as u64; }

	while pow > 1 {

  		if pow % 2 == 0 {
      		base = base * base % n;
      		pow  >>= 1;
  		}
		else {
			z    = base * z % n;
  			base = base * base % n;
  			pow  = (pow - 1) >> 1;
  		}
  	}

  	(base * z % n) as u64
}

// Strong Fermat Pseudoprime
pub(crate) fn sprp(p: u64, base: u64) -> bool { // checks if base^p = 1 mod p  or base^(d*2^n)= -1 for some n  
	let zeroes = (p - 1).trailing_zeros() as u64; // Breaks number down to p= d*2^n -1
	let d = (p - 1) / (1 << zeroes);
	let mut x = modpow(base, d, p); // base^d mod p

	if x == 1 || x == p - 1 // checks if base^p = 1 mod p  or base^(d*2^n)= -1
	{ return true; }

	for _ in 0..zeroes - 1 { // checks for all d*2^zeroes. One is subtracted since d*2^n was already checked above
		x = modpow(x, 2, p);

		if x == p - 1 // if any d*2^zeroes = p-1  then it passes
        { return true; }
	}
	return false; // otherwise it fails
}



// Binary gcd
pub(crate) fn gcd(mut a : u64, mut b: u64) -> u64{
    
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

        a -= b;

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
    
    while new_r !=0 {
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
