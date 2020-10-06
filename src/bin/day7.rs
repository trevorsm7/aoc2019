#[macro_use]
extern crate quick_error;

use itertools::Itertools;
use rayon::prelude::*;

use intcode::Intcode;

use std::borrow::Cow;
use std::env;
use std::io;
use std::num::ParseIntError;

quick_error! {
    #[derive(Debug)]
    pub enum SuperError {
        IoError(err: io::Error) { from() }
        ParseIntError(err: ParseIntError) { from() }
    }
}

type SuperResult<T> = Result<T, SuperError>;

fn main() -> SuperResult<()> {
    let input = {
        let name: Cow<'static, str> = env::args().nth(1)
            .map(|s| s.into()).unwrap_or_else(|| "input".into());
        std::fs::read_to_string(name.as_ref())?
    };

    let memory = input.trim().split(',')
        .map(str::parse)
        .collect::<Result<Vec<isize>, ParseIntError>>()?;

    println!("Part 1: {}", run_part1_rayon(&memory)?);
    println!("Part 2: {}", run_part2_rayon(&memory)?);

    Ok(())
}

#[allow(dead_code)]
fn run_part1(memory: &[isize]) -> SuperResult<isize> {
    (0..5).permutations(5).unique()
        .map(|sequence| run_amplifier(memory, &sequence))
        //.try_fold(0, |a, b| Ok(std::cmp::max(a, b?)))
        .fold_results(0, std::cmp::max) // More convenient alternative to try_fold from Itertools
}

fn run_part1_rayon(memory: &[isize]) -> SuperResult<isize> {
    // Rayon doesn't play nicely with Itertools, so roll our own permutation
    (0..factorial(5)).into_par_iter()
        .map(|i| {
            let mut sequence = [0, 1, 2, 3, 4];
            nth_permutation(&mut sequence, i);
            run_amplifier(memory, &sequence)
        })
        .try_reduce(|| 0, |a, b| Ok(std::cmp::max(a, b)))
}

#[allow(dead_code)]
fn run_part2(memory: &[isize]) -> SuperResult<isize> {
    (5..10).permutations(5).unique()
        .map(|sequence| run_feedback_amplifier(memory, &sequence))
        .fold_results(0, std::cmp::max)
}

fn run_part2_rayon(memory: &[isize]) -> SuperResult<isize> {
    // Rayon doesn't play nicely with Itertools, so roll our own permutation
    (0..factorial(5)).into_par_iter()
        .map(|i| {
            let mut sequence = [5, 6, 7, 8, 9];
            nth_permutation(&mut sequence, i);
            run_feedback_amplifier(memory, &sequence)
        })
        .try_reduce(|| 0, |a, b| Ok(std::cmp::max(a, b)))
}

fn factorial(i: usize) -> usize {
    (1..=i).product()
}

fn to_factoradic(mut value: usize, n: usize) -> Vec<usize> {
    let mut factoradic = vec![0; n];
    for i in 1..=n {
        factoradic[n - i] = value % i;
        value /= i;
    }
    factoradic
}

fn nth_permutation(sequence: &mut [isize], n: usize) {
    // For each digit in the factoradic representation...
    for (i, &offset) in to_factoradic(n, sequence.len()).iter().enumerate() {
        // ...rotate the selected element into the ith position
        sequence[i ..= i + offset].rotate_right(1);
    }
}

#[test]
fn test_permutation() {
    (0..5).permutations(5).unique().enumerate()
        .for_each(|(i, expected)| {
            let mut sequence = [0, 1, 2, 3, 4];
            nth_permutation(&mut sequence, i);
            assert_eq!(&sequence[..], &expected[..]);
        });
}

fn run_amplifier(memory: &[isize], sequence: &[isize]) -> SuperResult<isize> {
    let mut input = 0;
    for i in 0..5 {
        let mut program = Intcode::new(memory);
        program.run()?;
        program.resume(sequence[i])?;
        program.resume(input)?;
        input = *program.output.first().ok_or_else(||
            io::Error::new(io::ErrorKind::Other, "Program failed to produce output"))?;
    }
    Ok(input)
}

#[cfg(test)]
fn test_amplifier_helper(memory: &[isize], input: &[isize], expected: isize) {
    let output = run_amplifier(memory, input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_amplifier() {
    test_amplifier_helper(&[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], &[4,3,2,1,0], 43210);
    test_amplifier_helper(&[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], &[0,1,2,3,4], 54321);
    test_amplifier_helper(&[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], &[1,0,4,3,2], 65210);
}

fn run_feedback_amplifier(memory: &[isize], sequence: &[isize]) -> SuperResult<isize> {
    let mut input = 0;
    let mut amps = Vec::new();
    for i in 0..5 {
        let mut amp = Intcode::new(memory);
        amp.run()?;
        amp.resume(sequence[i])?;
        amps.push(amp);
    }
    let mut halted = false;
    while !halted {
        for i in 0..5 {
            if !amps[i].resume(input)? {
                halted = true;
            }
            input = *amps[i].output.last().ok_or_else(||
                io::Error::new(io::ErrorKind::Other, "Program failed to produce output"))?;
        }
    }
    Ok(input)
}

#[cfg(test)]
fn test_feedback_amplifier_helper(memory: &[isize], input: &[isize], expected: isize) {
    let output = run_feedback_amplifier(memory, input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_feedback_amplifier() {
    test_feedback_amplifier_helper(
        &[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
          27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
        &[9,8,7,6,5], 139629729);
    test_feedback_amplifier_helper(
        &[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
          -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
          53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
        &[9,7,8,5,6], 18216);
}
