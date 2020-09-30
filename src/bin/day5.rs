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

    let part1 = {
        let mut program = Intcode::new(&memory);
        assert_eq!(program.run()?, true);
        assert_eq!(program.resume(1)?, false);
        // expect all zeros except for last output
        assert!(program.output.iter().rev().skip(1).all(|&i| i == 0));
        *program.output.last().unwrap()
    };

    let part2 = {
        let mut program = Intcode::new(&memory);
        assert_eq!(program.run()?, true);
        assert_eq!(program.resume(5)?, false);
        *program.output.last().unwrap()
    };

    println!("Part 1: {}, Part 2: {}", part1, part2);

    Ok(())
}

#[cfg(test)]
fn test_program_helper(memory: &[isize], input: &[isize], memory_expected: &[isize], output_expected: &[isize]) {
    let mut program = Intcode::new(memory);
    let mut iter = input.iter().cloned();
    let mut yielding = program.run().unwrap();
    while yielding {
        yielding = program.resume(iter.next().unwrap()).unwrap();
    }
    assert_eq!(program.memory, memory_expected);
    assert_eq!(program.output, output_expected);
}

#[cfg(test)]
fn test_program_io(memory: &[isize], input: &[isize], output_expected: &[isize]) {
    let mut program = Intcode::new(memory);
    let mut iter = input.iter().cloned();
    let mut yielding = program.run().unwrap();
    while yielding {
        yielding = program.resume(iter.next().unwrap()).unwrap();
    }
    assert_eq!(program.output, output_expected);
}

#[test]
fn test_program() {
    test_program_helper(&[3,0,4,0,99], &[7], &[7,0,4,0,99], &[7]);
    test_program_helper(&[1002,4,3,4,33], &[], &[1002,4,3,4,99], &[]);
    // position mode equality
    test_program_io(&[3,9,8,9,10,9,4,9,99,-1,8], &[7], &[0]);
    test_program_io(&[3,9,8,9,10,9,4,9,99,-1,8], &[8], &[1]);
    test_program_io(&[3,9,7,9,10,9,4,9,99,-1,8], &[7], &[1]);
    test_program_io(&[3,9,7,9,10,9,4,9,99,-1,8], &[8], &[0]);
    // immediate mode equality
    test_program_io(&[3,3,1108,-1,8,3,4,3,99], &[7], &[0]);
    test_program_io(&[3,3,1108,-1,8,3,4,3,99], &[8], &[1]);
    test_program_io(&[3,3,1107,-1,8,3,4,3,99], &[7], &[1]);
    test_program_io(&[3,3,1107,-1,8,3,4,3,99], &[8], &[0]);
    // jump
    test_program_io(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0], &[0]);
    test_program_io(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[7], &[1]);
    test_program_io(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0], &[0]);
    test_program_io(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[7], &[1]);
    // larger example
    let larger_ex = &[
        3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    test_program_io(larger_ex, &[6], &[999]);
    test_program_io(larger_ex, &[8], &[1000]);
    test_program_io(larger_ex, &[10], &[1001]);
}
