#![allow(unused)]

#[ignore]
#[test]
fn benchmark() {
    println!("Benchmarks in non-optimised mode.");
    println!("\nIntersection Benchmark.");
    intersection_benchmark();
    println!("\nPrimes Benchmark.");
    primes_benchmark();
}

fn intersection_benchmark() {
    use std::time::Instant;
    use crate::math::sets::Set;

    let iterations: i32 = 10;
    let _v: Vec<i32> = (0..=1_000_000).collect();
    let _v2: Vec<i32> = (500_000..=2_000_000).collect();
    let s1 = Set::new(&_v);
    let s2: Set<i32> = Set::new(&_v2);
    let mut s3: Set<i32>;

    println!("[0; 1_000_000] ∩ [500_000; 2_000_000]:");
    let now = Instant::now();
    for _ in 0..iterations {
        s3 = s1.intersection(&s2);
    }
    let el = now.elapsed().as_millis() / iterations as u128;
    println!("{} milliseconds / iteration.", el);
}

#[ignore]
#[test]
fn primes_benchmark() {
    use std::time::Instant;
    use crate::math::primes::Primality;
    use crate::math::primes::generate_primes;

    println!("9223372036854775783u128.is_prime():");
    let mut now = Instant::now();

    for _ in 0..=1_000_000
    { 9223372036854775783u128.is_prime(); }

    let mut el = now.elapsed().as_nanos() / 1_000_000;

    println!("{} nanoseconds / iteration.\n", el);

    let mut _p: Vec<usize>;
    println!("generate_primes(1_000_000):");
    now = Instant::now();
    
    for _ in 0..=100 {
        _p = generate_primes(1_000_000);
    }
    el = now.elapsed().as_millis() / 100;
    println!("{} milliseconds / iteration.\n", el);
    let mut p: Vec<usize> = Vec::with_capacity(1_000_000);

    println!("is_prime() up to 1_000_000:");
    now = Instant::now();
    for _ in 0..=25 {
        for i in 0..1_000_000 {
            if (i as u128).is_prime() {p.push(i); }
        }
    }
    el = now.elapsed().as_millis() as u128 / 25;
    println!("{} milliseconds / iteration.\n", el);
}