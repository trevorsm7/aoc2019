#[macro_use]
extern crate quick_error;

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
        let args = env::args();
        let name = args.skip(1).next().unwrap_or("input".to_string());
        std::fs::read_to_string(name)?
    };

    let mut memory = input.trim().split(',')
        .map(str::parse)
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    // "before running the program, replace position 1 with the value 12 and replace position 2 with the value 2"
    memory[1] = 12;
    memory[2] = 2;

    run_program(&mut memory)?;
    println!("Result: {}", memory[0]);

    Ok(())
}

fn run_program(memory: &mut [usize]) -> io::Result<()> {
    let mut pc = 0;
    loop {
        match memory[pc] {
            1 => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let c = memory[pc + 3];
                memory[c] = memory[a] + memory[b];
            },
            2 => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let c = memory[pc + 3];
                memory[c] = memory[a] * memory[b];
            },
            99 => return Ok(()),
            _ => return Err(io::Error::new(io::ErrorKind::Other, format!("Illegal opcode {} at PC {}", memory[pc], pc))),
        };
        pc += 4;
    }
}

#[cfg(test)]
fn test_program(mut input: Vec<usize>, output: Vec<usize>) {
    run_program(&mut input).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_program_examples() {
    test_program(vec![1,9,10,3,2,3,11,0,99,30,40,50], vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    test_program(vec![1,0,0,0,99], vec![2,0,0,0,99]);
    test_program(vec![2,3,0,3,99], vec![2,3,0,6,99]);
    test_program(vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]);
    test_program(vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]);
}
