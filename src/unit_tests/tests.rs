#[test]
fn test_new_from_parent() {
    use crate::math::sets::Set;
    let test1: Set<u8> = Set::new(vec![0,1,2,3,4]);
    println!("test1: {}", test1);
    let from_parent: Set<u8> = Set::<u8>::new_from_parent(&test1, |x| x % 2 == 0);
    println!("from_parent: {}", from_parent)
}