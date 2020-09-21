#[macro_use]
extern crate quick_error;

use itertools::Itertools;

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

    // TODO this could be parallelized well
    let mut best_result = 0;
    let mut best_sequence = vec![];
    for sequence in (0..5).permutations(5).unique() {
        let result = run_amplifier(&memory, &sequence)?;
        if result > best_result {
            best_result = result;
            best_sequence = sequence;
        }
    }

    println!("{}, {:?}", best_result, best_sequence);

    Ok(())
}

fn run_amplifier(memory: &[isize], sequence: &[isize]) -> SuperResult<isize> {
    let mut input = 0;
    for i in 0..5 {
        let mut memory = Vec::from(memory);
        let output = run_program(&mut memory, &[sequence[i], input])?;
        input = *output.first().ok_or_else(||
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
