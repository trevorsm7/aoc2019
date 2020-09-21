#[macro_use]
extern crate quick_error;

use intcode::run_program;

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

fn main() -> Result<(), SuperError> {
    let input = {
        let name: Cow<'static, str> = env::args().nth(1)
            .map(|s| s.into()).unwrap_or_else(|| "input".into());
        std::fs::read_to_string(name.as_ref())?
    };

    let mut memory = input.trim().split(',')
        .map(str::parse)
        .collect::<Result<Vec<isize>, ParseIntError>>()?;

    let part1 = {
        let mut memory = memory.clone();
        let output = run_program(&mut memory, &[1])?;
        // expect all zeros except for last output
        assert!(output.iter().rev().skip(1).all(|&i| i == 0));
        *output.last().unwrap()
    };

    let part2 = {
        //let mut memory = memory.clone();
        *run_program(&mut memory, &[5])?.last().unwrap()
    };

    println!("Part 1: {}, Part 2: {}", part1, part2);

    Ok(())
}

#[cfg(test)]
fn test_program_helper(mut memory: Vec<isize>, input: &[isize], memory_expected: &[isize], output_expected: &[isize]) {
    let output = run_program(&mut memory, input).unwrap();
    assert_eq!(memory, memory_expected);
    assert_eq!(output, output_expected);
}

#[cfg(test)]
fn test_program_io(mut memory: Vec<isize>, input: &[isize], output_expected: &[isize]) {
    let output = run_program(&mut memory, input).unwrap();
    assert_eq!(output, output_expected);
}

#[test]
fn test_program() {
    test_program_helper(vec![3,0,4,0,99], &[7], &[7,0,4,0,99], &[7]);
    test_program_helper(vec![1002,4,3,4,33], &[], &[1002,4,3,4,99], &[]);
    // position mode equality
    test_program_io(vec![3,9,8,9,10,9,4,9,99,-1,8], &[7], &[0]);
    test_program_io(vec![3,9,8,9,10,9,4,9,99,-1,8], &[8], &[1]);
    test_program_io(vec![3,9,7,9,10,9,4,9,99,-1,8], &[7], &[1]);
    test_program_io(vec![3,9,7,9,10,9,4,9,99,-1,8], &[8], &[0]);
    // immediate mode equality
    test_program_io(vec![3,3,1108,-1,8,3,4,3,99], &[7], &[0]);
    test_program_io(vec![3,3,1108,-1,8,3,4,3,99], &[8], &[1]);
    test_program_io(vec![3,3,1107,-1,8,3,4,3,99], &[7], &[1]);
    test_program_io(vec![3,3,1107,-1,8,3,4,3,99], &[8], &[0]);
    // jump
    test_program_io(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0], &[0]);
    test_program_io(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[7], &[1]);
    test_program_io(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0], &[0]);
    test_program_io(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[7], &[1]);
    // larger example
    let larger_ex = vec![
        3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    test_program_io(larger_ex.clone(), &[6], &[999]);
    test_program_io(larger_ex.clone(), &[8], &[1000]);
    test_program_io(larger_ex.clone(), &[10], &[1001]);
}
