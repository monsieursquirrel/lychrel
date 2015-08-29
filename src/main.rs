/// Reverse a number then add it to the original
fn rev_add(num: u64) -> u64 {
    let rev_string: String = num.to_string().chars().rev().collect();
    // should be safe, our string is guaranteed to be a number
    let rev_val: u64 = rev_string.parse().unwrap();
    num + rev_val
}

/// Check if a number is a palindrome when written in base 10
fn is_palindrome(num: u64) -> bool {
    let num_string = num.to_string();
    let rev_string: String = num_string.chars().rev().collect();
    let comp_len = num_string.len() / 2;
    num_string[0..comp_len] == rev_string[0..comp_len]
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn simple_rev_adds() {
    assert!(rev_add(1) == 2);
    assert!(rev_add(12) == 33);
    assert!(rev_add(55) == 110);
    assert!(rev_add(123) == 444);
}

#[test]
fn simple_palindromes() {
    assert!(is_palindrome(1));
    assert!(is_palindrome(11));
    assert!(is_palindrome(121));
    assert!(is_palindrome(12321));
}

#[test]
fn not_palindromes() {
    assert!(!is_palindrome(12));
    assert!(!is_palindrome(21));
    assert!(!is_palindrome(1231));
    assert!(!is_palindrome(124321));
}
