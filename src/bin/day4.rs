use std::env;
use std::cmp::Ordering;

fn main() {
    let input = env::args().nth(1).expect("Expected argument ######-######")
        .trim().split('-')
        .map(|s| {
            s.chars().map(|c| {
                c.to_digit(10).unwrap_or_else(|| panic!("Invalid digit {}", c))
            }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();

    let end = &input[1];
    let mut current = first_monotonic_sequence(&input[0]);
    let mut repeating_count = 0;
    let mut doubled_count = 0;
    while is_sequence_le(&current, end) {
        if is_sequence_repeating(&current) {
            repeating_count += 1;
        }
        if is_sequence_doubled(&current) {
            doubled_count += 1;
        }
        increment_monotonic_sequence(&mut current);
    }

    println!("{} {}", repeating_count, doubled_count);
}

fn first_monotonic_sequence(input: &[u32]) -> Vec<u32> {
    let mut max = 0;
    let mut increasing = true;
    input.iter().cloned()
        .map(|i| {
            if i < max {
                increasing = false;
            }
            if increasing && i > max {
                max = i;
            }
            max
        }).collect()
}

#[test]
fn test_first_monotonic_sequence() {
    assert_eq!(first_monotonic_sequence(&[1, 2, 3]), &[1, 2, 3]);
    assert_eq!(first_monotonic_sequence(&[1, 0, 3]), &[1, 1, 1]);
    assert_eq!(first_monotonic_sequence(&[1, 2, 0]), &[1, 2, 2]);
    assert_eq!(first_monotonic_sequence(&[2, 6, 4, 7, 9, 3]), &[2, 6, 6, 6, 6, 6]);
}

fn is_sequence_le(a: &[u32], b: &[u32]) -> bool {
    for (da, db) in a.iter().zip(b.iter()) {
        match da.cmp(&db) {
            Ordering::Less => return true,
            Ordering::Greater => return false,
            _ => (),
        }
    }
    true
}

#[test]
fn test_is_sequence_le() {
    assert_eq!(is_sequence_le(&[1, 2, 3], &[1, 2, 3]), true);
    assert_eq!(is_sequence_le(&[1, 2, 3], &[1, 2, 4]), true);
    assert_eq!(is_sequence_le(&[1, 2, 3], &[2, 0, 0]), true);
    assert_eq!(is_sequence_le(&[1, 2, 3], &[1, 2, 2]), false);
}

fn increment_monotonic_sequence(sequence: &mut [u32]) {
    let mut replacement = 0;
    for &digit in sequence.iter().rev() {
        if digit < 9 {
            replacement = digit + 1;
            break;
        }
    }
    for digit in sequence.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return;
        } else {
            *digit = replacement;
        }
    }
}

#[cfg(test)]
fn test_increment_monotonic_sequence_helper(input: &[u32], output: &[u32]) {
    let mut copy = Vec::from(input);
    increment_monotonic_sequence(&mut copy);
    assert_eq!(copy, output);
}

#[test]
fn test_increment_monotonic_sequence() {
    test_increment_monotonic_sequence_helper(&[1, 7, 9], &[1, 8, 8]);
    test_increment_monotonic_sequence_helper(&[1, 8, 8], &[1, 8, 9]);
    test_increment_monotonic_sequence_helper(&[1, 8, 9], &[1, 9, 9]);
    test_increment_monotonic_sequence_helper(&[1, 9, 9], &[2, 2, 2]);
    test_increment_monotonic_sequence_helper(&[8, 9, 9], &[9, 9, 9]);
    test_increment_monotonic_sequence_helper(&[9, 9, 9], &[0, 0, 0]);
}

fn is_sequence_repeating(sequence: &[u32]) -> bool {
    let mut prev = sequence[0];
    let mut repeated = false;
    for &digit in sequence.iter().skip(1) {
        if digit == prev {
            repeated = true;
        }
        prev = digit;
    }
    repeated
}

#[test]
fn test_is_sequence_repeating() {
    assert_eq!(is_sequence_repeating(&[1, 1, 1, 1, 1, 1]), true);
    assert_eq!(is_sequence_repeating(&[2, 2, 3, 4, 5, 0]), true);
    assert_eq!(is_sequence_repeating(&[1, 2, 3, 7, 8, 9]), false);
}

fn is_sequence_doubled(sequence: &[u32]) -> bool {
    let mut prev = sequence[0];
    let mut repeat_count = 0;
    let mut doubled = false;
    for &digit in sequence.iter().skip(1) {
        if digit == prev {
            repeat_count += 1;
        } else {
            if repeat_count == 1 {
                doubled = true;
            }
            repeat_count = 0;
        }
        prev = digit;
    }
    doubled || repeat_count == 1
}

#[test]
fn test_is_sequence_doubled() {
    assert_eq!(is_sequence_doubled(&[1, 1, 2, 2, 3, 3]), true);
    assert_eq!(is_sequence_doubled(&[1, 2, 3, 4, 4, 4]), false);
    assert_eq!(is_sequence_doubled(&[1, 1, 1, 1, 2, 2]), true);
}

#[cfg(test)]
fn is_sequence_monotonic(sequence: &[u32]) -> bool {
    let mut max = sequence[0];
    for &digit in sequence.iter().skip(1) {
        if digit < max {
            return false;
        } else {
            max = digit;
        }
    }
    true
}

#[test]
fn test_is_sequence_monotonic() {
    assert_eq!(is_sequence_monotonic(&[1, 1, 1, 1, 1, 1]), true);
    assert_eq!(is_sequence_monotonic(&[2, 2, 3, 4, 5, 0]), false);
    assert_eq!(is_sequence_monotonic(&[1, 2, 3, 7, 8, 9]), true);
}
