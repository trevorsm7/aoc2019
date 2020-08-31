#[macro_use]
extern crate quick_error;

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

quick_error! {
    #[derive(Debug)]
    pub enum SuperError {
        IoError(err: std::io::Error) { from() }
        ParseIntError(err: std::num::ParseIntError) { from() }
    }
}

fn main() -> Result<(), SuperError> {
    let reader = {
        let args = env::args();
        let name = args.skip(1).next().unwrap_or("input".to_string());
        let file = File::open(name)?;
        BufReader::new(file)
    };

    let result: u32 = reader.lines()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse().ok())
        .map(compute_fuel)
        .sum();

    println!("{}", result);

    Ok(())
}

/*
To propagate errors while iterating instead of filtering, use this:

let mut err = Ok(());
let result: u32 = reader.lines()
    .map(|line| Ok(compute_fuel(line?.trim().parse()?)))
    .scan(&mut err, |err, res| match res {
        Ok(o) => Some(o),
        Err(e) => {
            **err = Err(e);
            None
        }
    })
    .sum();
err
*/

fn compute_fuel(input: u32) -> u32 {
    input / 3 - 2
}

#[test]
fn test_compute_fuel() {
    assert_eq!(compute_fuel(12), 2);
    assert_eq!(compute_fuel(14), 2);
    assert_eq!(compute_fuel(1969), 654);
    assert_eq!(compute_fuel(100756), 33583);
}
