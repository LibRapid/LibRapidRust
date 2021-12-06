#[test]
fn test_new_from_parent() {
    use crate::math::sets::Set;
    let test1:       Set<u8> = Set::new(&vec![0,1,2,3,4]);
    let from_parent: Set<u8> = Set::<u8>::new_subset(&test1, |x| x % 2 == 0);
    assert_eq!(from_parent, Set::new(&vec![0,2,4]))
}

#[test]
fn test_speed_dec_lshift() {
    use std::time::Instant;
    use crate::math::rapidmath::DecimalLeftShift;
    let mut i: i32;
    let now_mult: Instant = Instant::now();
    for _ in 0..=8_000_000 {
        i = 123234 * 10;
        print!("{}\r", i);
    }
    let el_mult: f64 = (*&now_mult.elapsed().as_millis() as f64) / 8_000_000f64;
    let mut j: i32;
    let now_lshift: Instant = Instant::now();
    for _ in 0..=8_000_000 {
        j = 123234.dec_lshift();
        print!("{}\r", j);
    }
    let el_lshift: f64 = (*&now_lshift.elapsed().as_millis() as f64) / 8_000_000f64;
    println!("Average over 8 Million iterations (With Print).");
    println!("Multiplication time: {} ms", el_mult);
    println!("dec_lshift time:     {} ms", el_lshift);
    assert_eq!(el_lshift <= el_mult, true);
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