extern crate num;

use num::FromPrimitive;
use num::bigint::BigInt;

/// Reverse a number then add it to the original
fn rev_add(num: &BigInt) -> BigInt {
    let rev_string: String = num.to_string().chars().rev().collect();
    // should be safe, our string is guaranteed to be a number
    let rev_val: BigInt = rev_string.parse().unwrap();
    num + rev_val
}

/// Check if a number is a palindrome when written in base 10
fn is_palindrome(num: &BigInt) -> bool {
    let num_string = num.to_string();
    let rev_string: String = num_string.chars().rev().collect();
    let comp_len = num_string.len() / 2;
    num_string[0..comp_len] == rev_string[0..comp_len]
}

/// Perform a lychrel test on a number, stopping after max_tests
fn test_lychrel(num: u64, max_tests: usize) -> bool {
    let num = FromPrimitive::from_u64(num).unwrap();
    (0..max_tests)
    .scan(num, |current, _| {
        *current = rev_add(current);
        Some(current.clone())
    })
    .filter(|curent| is_palindrome(curent))
    .next()
    .is_none()
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn simple_rev_adds() {
    assert!(rev_add(&FromPrimitive::from_u64(1).unwrap()) == FromPrimitive::from_u64(2).unwrap());
    assert!(rev_add(&FromPrimitive::from_u64(12).unwrap()) == FromPrimitive::from_u64(33).unwrap());
    assert!(rev_add(&FromPrimitive::from_u64(55).unwrap()) == FromPrimitive::from_u64(110).unwrap());
    assert!(rev_add(&FromPrimitive::from_u64(123).unwrap()) == FromPrimitive::from_u64(444).unwrap());
}

#[test]
fn simple_palindromes() {
    assert!(is_palindrome(&FromPrimitive::from_u64(1).unwrap()) );
    assert!(is_palindrome(&FromPrimitive::from_u64(11).unwrap()));
    assert!(is_palindrome(&FromPrimitive::from_u64(121).unwrap()));
    assert!(is_palindrome(&FromPrimitive::from_u64(12321).unwrap()));
}

#[test]
fn not_palindromes() {
    assert!(!is_palindrome(&FromPrimitive::from_u64(12).unwrap()));
    assert!(!is_palindrome(&FromPrimitive::from_u64(21).unwrap()));
    assert!(!is_palindrome(&FromPrimitive::from_u64(1231).unwrap()));
    assert!(!is_palindrome(&FromPrimitive::from_u64(124321).unwrap()));
}

#[test]
fn expected_lychrels() {
    assert!(test_lychrel(196, 500));
    assert!(test_lychrel(879, 500));
}

#[test]
fn expected_non_lychrels() {
    assert!(!test_lychrel(1, 500));
    assert!(!test_lychrel(2, 500));
    assert!(!test_lychrel(3, 500));
    assert!(!test_lychrel(4, 500));
}
