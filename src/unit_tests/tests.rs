#[test]
fn test_new_from_parent() {
    use crate::math::sets::Set;
    let test1:       Set<u8> = Set::new(&vec![0,1,2,3,4]);
    let from_parent: Set<u8> = Set::<u8>::new_subset(&test1, |x| x % 2 == 0);
    assert_eq!(from_parent, Set::new(&vec![0,2,4]))
}

#[test]
fn test_map_to() {
    use crate::math::rapidmath::MapToNumRange;

    let result: f32 = 5f32.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    assert_eq!(result, 0.5);
}

#[test]
fn test_rec_printing() {
    use crate::math::sets::Set;

    let s:  Set<i32> = Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    let s1: Set<i32> = Set::new_subset(&s, |x| x % 2 == 0);
    let s2: Set<i32> = Set::new_subset(&s1, |x| x == 4);

    s2.full_print();
    println!("{}", s2);
    assert_eq!(s2.to_full_string(), "{ 4 } ⊆ { 0; 2; 4; 6; 8; 10 } ⊆ { 0; 1; 2; 3; 4; 5; 6; 7; 8; 9; 10 }".to_string());
}

#[test]
fn test_union() {
    use crate::math::sets::Set;

    let s:  Set<i32> = Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    let s1: Set<i32> = Set::new(&vec![11,12,13,13,11,0,0,0]);

    let c:  Set<i32> = s.union(&s1);
    assert_eq!(c, Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13]));
}

#[test]
fn test_intersection() {
    use crate::math::sets::Set;

    let s:  Set<i32> = Set::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    let s2: Set<i32> = Set::new(&vec![0,1,2,3,11,0,0,0]);

    let c:  Set<i32> = s.intersection(&s2);
    assert_eq!(c, Set::new(&vec![0,1,2,3]));
}

#[test]
fn test_vec_len_speed() {
    use crate::math::vectors::MathVector;
    use crate::new_mathvec;
    use std::time::Instant;

    let mut v1: MathVector = new_mathvec!(0.,2.5,4.);
    let now:    Instant    = Instant::now();
    for _ in 0..=10 {
        for _ in 0..=1_000_000 {
            v1.set_values(&vec![0.,2.5,4.]);
            v1.length();
            v1.set_values(&vec![0.,0.,0.]);
        }
    }
    let elapsed: u128 = now.elapsed().as_nanos();
    println!("{} ns Total", elapsed);
    println!("{} ns Avg / iteration", elapsed as f64 / 1_000_000. / 10.);
}

#[test]
fn test_set_macro() {
    use crate::new_set;
    use crate::math::sets::Set;
    let set: Set<i32> = new_set!(0,1,2,3,4,5,6,-1);
    assert_eq!(set.to_string(), "{ 0; 1; 2; 3; 4; 5; 6; -1 }");
    assert_eq!(set.to_full_string(), "{ 0; 1; 2; 3; 4; 5; 6; -1 }");
}

#[test]
fn test_postfix() {
    use crate::math::postfix::eval_postfix;
    assert_eq!(0f32, eval_postfix!(1f32 1f32 + 2f32 %));
    assert_eq!(true, eval_postfix!(1f32 1f32 + 2f32 % 0f32 ==));
}

#[test]
fn test_logger() {
    use crate::compsci::rapidlogging::Logger;
    let mut l: Logger = Logger::new(3, true, true, "log.txt".to_string());
    let _ = l.log(Some(vec!["warning", "low urgency"]), "Test-Log.");
    let _ = l.log(None, "Test-Log.");
    let _ = l.log(None, "Test-Log.");
}

#[test]
fn test_prime() {
    use crate::math::rapidmath::Primality;
    let p:     Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let mut f: Vec<u64> = Vec::new();
    for i in 0..100 {
        if (i as u64).is_prime() { f.push(i as u64); }
    }

    assert_eq!(p, f);
}

#[test]
fn test_prime_sieve() {
    use crate::math::rapidmath::generate_primes;
    let p: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let f: Vec<usize> = generate_primes(100);

    assert_eq!(p, f);
}