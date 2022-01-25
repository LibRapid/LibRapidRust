#![allow(unused)]
use std::time::Instant;
use crate::compsci::compression::huffman::write_to_file;
use crate::math::constants::SQRT5;
use crate::math::general::Increment;
use crate::math::general::nth_fibonacci;
use crate::math::primes::Primality;
use crate::math::primes::generate_primes;
use crate::math::sets::vec_sets::VecSet;

#[ignore]
#[test]
fn benchmark() {
    let intersection_run = false;
    let primes_run = false;
    let fib_run = true;
    let huffman_run = false;
    let nth_root_run = true;
    println!("Benchmarks in non-optimised mode.");

    if huffman_run {
        println!("\nHuffman Compression benchmark.");
        huffman_bench(100);
    }

    if nth_root_run {
        println!("\nNth root benchmark.");
        nth_root_bench(1_000_000);
    }

    if fib_run {
        println!("\nFibonacci benchmark.");
        fibonacci(75, 1_000_000);
    }

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
fn nth_root_bench(iterations: u128) {
    use crate::math::general::nth_root;

    let mut now = Instant::now();
    for _ in 0..iterations {
        nth_root(3.0, 3.0);
        nth_root(2.0, 2.0);
    }
    let mut el = now.elapsed().as_nanos() / iterations;
    println!("{} ns / iteration (Current approach).", el);
    now = Instant::now();
    for _ in 0..iterations {
        3.0_f64.powf(1.0/3.0);
        2.0_f64.powf(1.0/2.0);
    }
    el = now.elapsed().as_nanos() / iterations;
    println!("{} ns / iteration (New approach).", el);

}
fn fibonacci(n: u128, iterations: u128) {
    let mut now = Instant::now();
    let mut current_number: u128 = 1;

    for _ in 0..iterations {
        let mut first_number: u128 = 0;
        let mut second_number: u128 = 0;
        current_number = 1;
        let mut i: u128 = 1;

        while i < n {
        
            first_number = second_number;
            second_number = current_number;
            current_number = first_number + second_number;
            i.inc();
        }
    }
    let mut el = now.elapsed().as_nanos() / iterations;
    println!("{} ns / iteration (iterative approach).", el);
    println!("{}", &current_number);
    let mut f = 0;
    now = Instant::now();
    for _ in 0..iterations {
        f = nth_fibonacci(n);
    }
    el = now.elapsed().as_nanos() / iterations;
    println!("{} ns / iteration (formula approach).", el);
    println!("{}", f as u128);
}

fn intersection_bench(iterations: u128, set_size: i32) {
    let _v: Vec<i32> = (0..set_size).collect();
    let _v2: Vec<i32> = (set_size / 2..=set_size * 2).collect();
    let s1 = VecSet::new(&_v);
    let s2: VecSet<i32> = VecSet::new(&_v2);
    let mut s3: VecSet<i32>;

    println!("[0; {}] ∩ [{}; {}]:", &set_size, &set_size / 2, &set_size * 2);
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

fn huffman_bench(iters: u128) {
    use crate::compsci::compression::huffman::{Encode, Decode, get_root};
    use std::fs;
    use std::collections::HashMap;

    let mut lorem = String::with_capacity(13_000_000);
    lorem.push_str(&fs::read_to_string("./src/10millionchars-lorem-ipsum.txt").unwrap());
    let now = Instant::now();
    let mut e = lorem.full_encode();
    let mut d = String::new();
    
    for i in 0..iters as u8 {
        lorem.push(i as char);
        e = lorem.full_encode();
        d = e.full_decode();
    }
    let el = now.elapsed().as_millis() / iters;
    println!("{}", d);
    println!("{} milliseconds / iteration.\n", el);
    write_to_file("./src/10millionchars-lorem-ipsum".to_owned(), &e.0, &e.1);
}