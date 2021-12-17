pub fn modpow(x : u64,mut  pow: u64, modulus: u64)-> u64{  //upgrades to u128 to allow mul-mod

  let mut z = 1u128;
  let mut base = x.clone() as u128;
  let n = modulus as u128;
  if pow ==0 {
    return z as u64
  }

 while pow > 1 {
  
   if pow%2 == 0 {
      base = base*base % n ;
      pow>>=1;
   }
  
  else{
  
   z = base*z % n;
   base = base*base % n;
   pow=(pow-1)>>1;  
   
 }
 }

 (base*z % n) as u64

}

// Strong Fermat Pseudoprime

pub fn sprp(p: u64, base: u64)->bool{// checks if base^p = 1 mod p  or base^(d*2^n)= -1 for some n  
     let zeroes = (p-1).trailing_zeros() as u64; // Breaks number down to p= d*2^n -1
     let d = (p-1)/ (1<<zeroes);
     let mut x = modpow(base,d, p); // base^d mod p
     if x == 1u64 || x==p-1{   // checks if base^p = 1 mod p  or base^(d*2^n)= -1
       return true
       }
    for _ in 0..zeroes-1{// checks for all d*2^zeroes. One is subtracted since d*2^n was already checked above
     x = modpow(x, 2, p);
     if x == p-1 {       // if any d*2^zeroes = p-1  then it passes
       return true
     }
    }
    return false        // otherwise it fails
 }
 
 
