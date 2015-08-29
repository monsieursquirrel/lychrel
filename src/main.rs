extern crate num;
use num::FromPrimitive;
use num::bigint::BigInt;

extern crate clap;
use clap::{Arg, App};

use std::collections::HashSet;

/// Reverse a number then add it to the original.
fn rev_add(num: &BigInt) -> BigInt {
    let rev_string: String = num.to_string().chars().rev().collect();
    // should be safe, our string is guaranteed to be a number
    let rev_val: BigInt = rev_string.parse().unwrap();
    num + rev_val
}

/// Check if a number is a palindrome when written in base 10.
fn is_palindrome(num: &BigInt) -> bool {
    let num_string = num.to_string();
    let rev_string: String = num_string.chars().rev().collect();
    let comp_len = num_string.len() / 2;
    num_string[0..comp_len] == rev_string[0..comp_len]
}

/// Perform a lychrel test on a number, stopping after max_tests
/// Returns the sequence of numbers if this number is a lychrel, None otherwise.
fn test_lychrel(num: &BigInt, max_tests: usize) -> Option<Vec<BigInt>> {
    let mut sequence = Vec::<BigInt>::new();

    let is_lychrel = (0..max_tests)
        .scan(num.clone(), |current, _| {
            *current = rev_add(current);
            Some(current.clone())
        })
        .inspect(|current| sequence.push(current.clone()))
        .filter(|curent| is_palindrome(curent))
        .next()
        .is_none();

    if is_lychrel {
        Some(sequence)
    }
    else {
        None
    }
}

/// Determine if the sequence for a lychrel number is related to a previously seen sequence
fn is_related(seq: &Vec<BigInt>, lychrel_seq_numbers: &HashSet<BigInt>) -> bool {
    seq.iter().filter(|num| lychrel_seq_numbers.contains(num)).next().is_some()
}

/// Find the lychrel numbers up to max_num (inclusive).
/// Returns a tuple (lychrel numbers, related numbers, palindrome lychrel/related numbers)
fn find_lychrels(max_num: u64, max_tests: usize) -> (Vec<BigInt>, Vec<BigInt>, Vec<BigInt>) {
    // storage for various outputs
    let mut lychrels = Vec::<BigInt>::new();
    let mut relateds = Vec::<BigInt>::new();
    let mut palindrome_lychrels = Vec::<BigInt>::new();

    let mut lychrel_seq_numbers: HashSet<BigInt> = HashSet::new();

    for i in (1..(max_num + 1)) {
        let num = FromPrimitive::from_u64(i).unwrap();
        let maybe_lychrel = test_lychrel(&num, max_tests);

        if let Some(lychrel_seq) = maybe_lychrel {
            // it's a lychrel - check if it's a related number
            let related = is_related(&lychrel_seq, &lychrel_seq_numbers);

            // update our sequences
            for seq_num in lychrel_seq.into_iter() {
                lychrel_seq_numbers.insert(seq_num);
            }

            if !related {
                // the number has a new lychrel sequence, store it
                lychrels.push(num.clone());
            }
            else {
                // just count it as a related number
                relateds.push(num.clone());
            }

            if is_palindrome(&num) {
                // doesn't matter if palindromes are related or not
                palindrome_lychrels.push(num.clone());
            }
        }
    }

    (lychrels, relateds, palindrome_lychrels)
}

fn print_nums(before: &str, numbers: &Vec<BigInt>) {
    print!("{}", before);
    for (i, current) in numbers.iter().enumerate() {
        print!("{}", current);
        if i + 1 < numbers.len() {
            print!(", ");
        }
    }
    println!("");
}

fn main() {
    let args = App::new("Lychrels")
    .version("0.1")
    .author("monsieursquirrel")
    .about("Finds lychrel numbers in a given range.")
    .arg(Arg::with_name("RANGE")
        .help("Sets the maximum limit of the range to be serached")
        .required(false)
        .takes_value(true)
        .validator(|value| value.parse::<u64>().map(|_| () ).map_err(|_| "not a number".to_owned() ))
        .long("range"))
    .arg(Arg::with_name("TESTS")
        .help("Sets the maximum number of tests performed on each number")
        .required(false)
        .takes_value(true)
        .validator(|value| value.parse::<usize>().map(|_| () ).map_err(|_| "not a number".to_owned() ))
        .long("tests"))
    .get_matches();

    let max_num: u64 = args.value_of("RANGE").and_then(|arg_str| arg_str.parse().ok() ).unwrap_or(10_000);
    let max_tests: usize = args.value_of("TESTS").and_then(|arg_str| arg_str.parse().ok() ).unwrap_or(500);

    println!("Calculations using n = 1..{} and limiting each search to {} reverse-digits-and-adds",
        max_num, max_tests);

    let (lychrels, relateds, palindrome_lychrels) = find_lychrels(max_num, max_tests);

    println!("Number of Lychrel numbers: {}", lychrels.len());
    print_nums("Lychrel numbers: ", &lychrels);
    println!("Number of Lychrel related: {}", relateds.len());
    println!("Number of Lychrel palindromes: {}", palindrome_lychrels.len());
    print_nums("Lychrel palindromes: ", &palindrome_lychrels);
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
    assert!(is_palindrome(&FromPrimitive::from_u64(1221).unwrap()));
    assert!(is_palindrome(&FromPrimitive::from_u64(12321).unwrap()));
    assert!(is_palindrome(&FromPrimitive::from_u64(123321).unwrap()));
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
    assert!(test_lychrel(&FromPrimitive::from_u64(196).unwrap(), 500).is_some());
    assert!(test_lychrel(&FromPrimitive::from_u64(879).unwrap(), 500).is_some());
}

#[test]
fn expected_non_lychrels() {
    assert!(test_lychrel(&FromPrimitive::from_u64(1).unwrap(), 500).is_none());
    assert!(test_lychrel(&FromPrimitive::from_u64(2).unwrap(), 500).is_none());
    assert!(test_lychrel(&FromPrimitive::from_u64(3).unwrap(), 500).is_none());
    assert!(test_lychrel(&FromPrimitive::from_u64(4).unwrap(), 500).is_none());
}
