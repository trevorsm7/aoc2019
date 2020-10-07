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

    // TODO

    Ok(())
}

#[cfg(test)]
fn test_intcode_helper(input: &[isize], output: &[isize]) {
    let mut program = Intcode::new(input);
    program.run().unwrap();
    assert_eq!(output, &program.output[..]);
}

#[test]
fn test_intcode() {
    let quine = &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    test_intcode_helper(quine, quine);
    test_intcode_helper(&[1102,34915192,34915192,7,4,7,99,0], &[1219070632396864]);
    test_intcode_helper(&[104,1125899906842624,99], &[1125899906842624]);
}