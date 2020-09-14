#[macro_use]
extern crate quick_error;

use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
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
    let reader = {
        let name: Cow<'static, str> = env::args().nth(1)
            .map(|s| s.into()).unwrap_or_else(|| "input".into());
        BufReader::new(File::open(name.as_ref())?)
    };

    let mut orbits = reader.lines()
        .map(|s| parse_orbit(&s?))
        .collect::<SuperResult<HashMap<u16, Orbit>>>()?;

    // Insert COM into map as root
    let com = u16::from_str_radix("COM", 36).unwrap();
    orbits.insert(com, (0, Cell::new(Some(0))));

    let orbit_count: u32 = orbits.keys()
        .map(|&k| count_orbits(&orbits, k))
        .sum();

    println!("{}", orbit_count);

    Ok(())
}

type Orbit = (u16, Cell<Option<u32>>);

fn count_orbits(orbits: &HashMap<u16, Orbit>, id: u16) -> u32 {
    let orbit = &orbits[&id];
    if let None = orbit.1.get() {
        orbit.1.set(Some(count_orbits(&orbits, orbit.0) + 1));
    }
    orbit.1.get().unwrap()
}

fn parse_orbit(input: &str) -> SuperResult<(u16, Orbit)> {
    let result = input.trim().split(')')
        .map(|s| u16::from_str_radix(s, 36))
        .collect::<Result<Vec<u16>, ParseIntError>>()?;
    Ok((result[1], (result[0], Cell::new(None))))
}
