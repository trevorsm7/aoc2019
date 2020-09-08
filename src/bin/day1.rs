use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let reader = {
        let name: Cow<'static, str> = env::args().nth(1)
            .map(|s| s.into()).unwrap_or_else(|| "input".into());
        BufReader::new(File::open(name.as_ref())?)
    };

    let result: (u32, u32) = reader.lines()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse().ok())
        .map(|n| (compute_fuel(n), compute_fuel_fuel(n)))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("{:?}", result);

    Ok(())
}

fn compute_fuel(input: u32) -> u32 {
    if input > 2 * 3 {
        input / 3 - 2
    } else {
        0
    }
}

#[test]
fn test_compute_fuel() {
    assert_eq!(compute_fuel(0), 0);
    assert_eq!(compute_fuel(12), 2);
    assert_eq!(compute_fuel(14), 2);
    assert_eq!(compute_fuel(1969), 654);
    assert_eq!(compute_fuel(100756), 33583);
}

fn compute_fuel_fuel(input: u32) -> u32 {
    let mut total = 0;
    let mut fuel = compute_fuel(input);
    while fuel > 0 {
        total += fuel;
        fuel = compute_fuel(fuel);
    }
    total
}

#[test]
fn test_compute_fuel_fuel() {
    assert_eq!(compute_fuel_fuel(14), 2);
    assert_eq!(compute_fuel_fuel(1969), 966);
    assert_eq!(compute_fuel_fuel(100756), 50346);
}
