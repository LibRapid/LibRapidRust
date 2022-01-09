 use number_theory::arithmetic::mpz::Mpz;
 use number_theory::arithmetic::sign::Sign;
 use number_theory::traits::NumberTheory;
 
 #[derive(Clone)]
 pub struct BigInt{
   num: Mpz,
 }
 
 impl BigInt{
   
   fn zero() -> Self{
      BigInt{num: Mpz::zero()} 
   }
   
   fn one() -> Self{
     BigInt{num: Mpz::one()} 
   }
   
  pub fn from_string(x: &str) -> Self{
     BigInt{num: Mpz::u_from_string(x).unwrap()}
   }
   
  pub fn to_string(&self) -> String{
      self.num.to_string()
   }
 }
 
 
 impl std::ops::Add for BigInt{
    type Output = Self;
  fn add(self, other: Self) -> Self{
  BigInt{num: self.num.addition(other.num)}
 }
 
 }
 
 impl std::ops::Sub for BigInt{
 type Output = Self;
  fn sub(self, other: Self) -> Self{
  BigInt{num: self.num.subtraction(other.num)}
 }
 }
 
 impl std::ops::Mul for BigInt{
 type Output = Self;
  fn mul(self, other: Self) -> Self{
  BigInt{num: self.num.product(other.num)}
 }
 }
 
 impl std::ops::Div for BigInt{
 type Output = Self;
  fn div(self, other: Self) -> Self{
  BigInt{num: self.num.euclidean(&other.num).0}
 }
 }
 
 impl std::ops::Rem for BigInt{
   type Output = Self;
   fn rem(self, other: Self) -> Self{
     BigInt{num: self.num.euclidean(&other.num).1}
   }
 }
