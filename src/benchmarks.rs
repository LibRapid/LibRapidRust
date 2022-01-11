#![allow(unused)]
use std::time::Instant;
use crate::math::primes::Primality;
use crate::math::primes::generate_primes;
use crate::math::sets::Set;

#[ignore]
#[test]
fn benchmark() {
    let intersection_run = true;
    let primes_run = true;
    println!("Benchmarks in non-optimised mode.");

    if intersection_run {
        println!("\nIntersection Benchmark.");
        intersection_bench(10, 1_000_000);
    }
    if primes_run {
        println!("\nPrimes Benchmark.");
        generate_primes_bench(100);
        big_is_prime_bench(1_000_000);
        sieve_is_prime_bench(25);
    }
}

fn intersection_bench(iterations: u128, set_size: i32) {
    let _v: Vec<i32> = (0..set_size).collect();
    let _v2: Vec<i32> = (set_size / 2..=set_size * 2).collect();
    let s1 = Set::new(&_v);
    let s2: Set<i32> = Set::new(&_v2);
    let mut s3: Set<i32>;

    println!("[0; 1_000_000] ∩ [500_000; 2_000_000]:");
    let now = Instant::now();
    for _ in 0..iterations {
        s3 = s1.intersection(&s2);
    }
    let el = now.elapsed().as_millis() / iterations;
    println!("{} milliseconds / iteration.", el);
}

fn generate_primes_bench(iters: u128) {
    let mut _p: Vec<usize>;
    println!("generate_primes(1_000_000):");
    let now = Instant::now();
    
    for _ in 0..iters {
        _p = generate_primes(1_000_000);
    }
    let el = now.elapsed().as_millis() / iters;
    println!("{} milliseconds / iteration.\n", el);
}

fn big_is_prime_bench(iters: u128) {
    println!("9223372036854775783u128.is_prime():");
    let mut now = Instant::now();

    for _ in 0..iters
    { 9223372036854775783u128.is_prime(); }

    let mut el = now.elapsed().as_nanos() / iters;

    println!("{} nanoseconds / iteration.\n", el);
}

fn sieve_is_prime_bench(iters: u128) {
    let mut p: Vec<usize> = Vec::with_capacity(1_000_000);

    println!("is_prime() up to 1_000_000:");
    let now = Instant::now();
    for _ in 0..iters {
        for i in 0..1_000_000 {
            if (i as u128).is_prime() {p.push(i); }
        }
    }
    let el = now.elapsed().as_millis() as u128 / iters;
    println!("{} milliseconds / iteration.\n", el);
}