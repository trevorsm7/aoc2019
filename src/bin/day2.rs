#[macro_use]
extern crate quick_error;

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

fn main() -> Result<(), SuperError> {
    let input = {
        let name: Cow<'static, str> = env::args().nth(1)
            .map(|s| s.into()).unwrap_or_else(|| "input".into());
        std::fs::read_to_string(name.as_ref())?
    };

    let memory = input.trim().split(',')
        .map(str::parse)
        .collect::<Result<Vec<isize>, ParseIntError>>()?;

    // "before running the program, replace position 1 with the value 12 and replace position 2 with the value 2"
    println!("Part 1: {}", run_program_with(&memory, 12, 2)?);

    // for some reason, position 1 and 2 are nouns and verbs and we need to brute force them until we get 19690720?
    'exit: for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program_with(&memory, noun, verb)? == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'exit;
            }
        }
    }

    Ok(())
}

fn run_program_with(memory: &[isize], noun: isize, verb: isize) -> io::Result<isize> {
    let mut program = Intcode::new(memory);
    program.memory[1] = noun;
    program.memory[2] = verb;
    match program.run() {
        Ok(_) => Ok(program.memory[0]),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
fn test_program_helper(input: &[isize], output: &[isize]) {
    let mut program = Intcode::new(input);
    assert_eq!(program.run().unwrap(), false);
    assert_eq!(program.memory, output);
}

#[test]
fn test_program() {
    test_program_helper(&[1,9,10,3,2,3,11,0,99,30,40,50], &[3500,9,10,70,2,3,11,0,99,30,40,50]);
    test_program_helper(&[1,0,0,0,99], &[2,0,0,0,99]);
    test_program_helper(&[2,3,0,3,99], &[2,3,0,6,99]);
    test_program_helper(&[2,4,4,5,99,0], &[2,4,4,5,99,9801]);
    test_program_helper(&[1,1,1,4,99,5,6,0,99], &[30,1,1,4,2,5,6,0,99]);
}
