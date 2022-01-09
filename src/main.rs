#![allow(dead_code)]
//! # LibRapid - A general purpose, optimised library for mathematics and computer science.
//! This crate allows for faster calculations for mathematics, physics, or computer science.
//! All implementations are optmised for speed, meaning your computations will be faster and more efficient.
//! Some implementations include mathematical sets or text compression.
//! 
pub mod math;
pub mod compsci;
use crate::math::primes::Primality;
use crate::math::primes::generate_primes;
use crate::math::bigint::BigInt;
fn main(){
let mut count = 0u64;
let k = BigInt::from_string("340282366920938463463374607431768211456");
let time = std::time::Instant::now();
let t = k.clone()% k.clone();
let stop = time.elapsed();
println!("{} primes counted in {:?}",t.to_string(), stop)
}
