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
        let mut args = env::args();
        let name: Cow<'static, str> = args.nth(1).map(|s| s.into()).unwrap_or_else(|| "input".into());
        std::fs::read_to_string(name.as_ref())?
    };

    let mut memory = input.trim().split(',')
        .map(str::parse)
        .collect::<Result<Vec<isize>, ParseIntError>>()?;

    let output = run_program(&mut memory, &[1])?;

    println!("{:?}", output);

    Ok(())
}

#[cfg(test)]
fn test_program_helper(mut memory: Vec<isize>, input: &[isize], memory_expected: &[isize], output_expected: &[isize]) {
    let output = run_program(&mut memory, &input).unwrap();
    assert_eq!(memory, memory_expected);
    assert_eq!(output, output_expected);
}

#[test]
fn test_program() {
    test_program_helper(vec![3,0,4,0,99], &[7], &[7,0,4,0,99], &[7]);
    test_program_helper(vec![1002,4,3,4,33], &[], &[1002,4,3,4,99], &[]);
}
