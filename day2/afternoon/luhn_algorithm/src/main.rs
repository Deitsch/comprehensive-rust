// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use regex::Regex;

pub fn luhn(cc_number: &str) -> bool {

    let re = Regex::new(r"\D").unwrap(); // any non digit
    let cc_number_sanitized = re.replace_all(cc_number, "");

    println!("{cc_number_sanitized}");
    if cc_number_sanitized.len() < 2 {
        return false;
    }

    let mut double_sum: u32 = 0;
    let mut single_sum: u32 = 0;
    for (i, num) in cc_number_sanitized.chars().rev().filter_map(|c| c.to_digit(10)).enumerate() {
        if i % 2 == 0 {
            single_sum += num;
        }
        else {
            let sum: u32 = (num * 2)
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .sum();

            double_sum += sum;
            println!("okko {sum}");
        }
    }
    let sum = single_sum + double_sum;
    return sum % 10 == 0;
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    let valid = luhn("1284");
    // let valid = luhn("4263 9826 4026 9299");
    println!("{valid}");
}