#[macro_use]
extern crate quick_error;

use std::borrow::Cow;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
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

    let com = u16::from_str_radix("COM", 36).unwrap();
    let you = u16::from_str_radix("YOU", 36).unwrap();
    let san = u16::from_str_radix("SAN", 36).unwrap();

    // Insert COM into map as root
    orbits.insert(com, Orbit::root());

    let orbit_count: u32 = orbits.keys()
        .map(|&k| count_orbits(&orbits, k))
        .sum();

    println!("{}", orbit_count);
    println!("{}", count_transfers(&orbits, you, san).unwrap());

    Ok(())
}

struct Orbit {
    id: u16,
    count: Cell<Option<u32>>,
}

impl Orbit {
    fn new(id: u16) -> Self {
        Orbit {id, count: Cell::new(None)}
    }

    fn root() -> Self {
        Orbit {id: 0, count: Cell::new(Some(0))}
    }
}

fn count_transfers(orbits: &HashMap<u16, Orbit>, from: u16, to: u16) -> Option<u32> {
    // Visit nodes from 'from' to root
    let mut from_path = vec![];
    let mut tmp = orbits[&from].id;
    while tmp != 0 {
        from_path.push(tmp);
        tmp = orbits[&tmp].id;
    }

    // Visit nodes from 'to' to root, stopping at the first common node
    tmp = orbits[&to].id;
    let mut depth = 0;
    while tmp != 0 {
        if let Some(idx) = from_path.iter().position(|&x| x == tmp) {
            return Some(depth + idx as u32);
        }
        tmp = orbits[&tmp].id;
        depth += 1;
    }

    None
}

fn count_orbits(orbits: &HashMap<u16, Orbit>, id: u16) -> u32 {
    let orbit = &orbits[&id];
    if let None = orbit.count.get() {
        orbit.count.set(Some(count_orbits(&orbits, orbit.id) + 1));
    }
    orbit.count.get().unwrap()
}

fn parse_orbit(input: &str) -> SuperResult<(u16, Orbit)> {
    let result = input.trim().split(')')
        .map(|s| u16::from_str_radix(s, 36))
        .collect::<Result<Vec<u16>, ParseIntError>>()?;
    Ok((result[1], Orbit::new(result[0])))
}
