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

/// Perform a lychrel test on a number, stopping after max_tests
fn test_lychrel(num: u64, max_tests: usize) -> bool {
    (0..max_tests)
    .scan(num, |current, _| {
        *current = rev_add(*current);
        Some(*current)
    })
    .filter(|curent| is_palindrome(*curent))
    .next()
    .is_none()
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

#[test]
fn expected_lychrels() {
    assert!(test_lychrel(196, 500));
    assert!(!test_lychrel(879, 500));
}

#[test]
fn expected_non_lychrels() {
    assert!(!test_lychrel(1, 500));
    assert!(!test_lychrel(2, 500));
    assert!(!test_lychrel(3, 500));
    assert!(!test_lychrel(4, 500));
}
