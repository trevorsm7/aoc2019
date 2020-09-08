#[macro_use]
extern crate quick_error;

use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Sub};

quick_error! {
    #[derive(Debug)]
    pub enum SuperError {
        IoError(err: io::Error) { from() }
        ParseIntError(err: ParseIntError) { from() }
    }
}

type SuperResult<T> = Result<T, SuperError>;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cardinal {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Cardinal {
    fn is_horizontal(self) -> bool {
        match self {
            Cardinal::Up(_) => false,
            Cardinal::Down(_) => false,
            Cardinal::Left(_) => true,
            Cardinal::Right(_) => true,
        }
    }

    fn distance(self) -> u32 {
        match self {
            Cardinal::Up(value) => value,
            Cardinal::Down(value) => value,
            Cardinal::Left(value) => value,
            Cardinal::Right(value) => value,
        }
    }
}

type Wire = Vec<Cardinal>;
type WireSlice<'a> = &'a [Cardinal];

// Parse str "U2" into Cardinal::Up(2)
fn parse_cardinal(input: &str) -> SuperResult<Cardinal> {
    let mut chars = input.trim().chars();
    let cardinal = chars.next();
    let value = chars.as_str().parse()?;
    match cardinal {
        Some('U') => Ok(Cardinal::Up(value)),
        Some('D') => Ok(Cardinal::Down(value)),
        Some('L') => Ok(Cardinal::Left(value)),
        Some('R') => Ok(Cardinal::Right(value)),
        _ => Err(io::Error::new(io::ErrorKind::InvalidData,
            format!("Invalid cardinal {}", input)).into()),
    }
}

#[test]
fn test_parse_cardinal() {
    assert_eq!(parse_cardinal(" R8 ").ok(), Some(Cardinal::Right(8)));
    assert_eq!(parse_cardinal("U5").ok(), Some(Cardinal::Up(5)));
    assert_eq!(parse_cardinal("L5").ok(), Some(Cardinal::Left(5)));
    assert_eq!(parse_cardinal("D3").ok(), Some(Cardinal::Down(3)));
    assert!(parse_cardinal("A1").is_err());
    assert!(parse_cardinal("").is_err());
    assert!(parse_cardinal("U").is_err());
}

// Parse str "R2,D2" into vec![Cardinal::Right(2), Cardinal::Down(2)]
fn parse_wire(input: &str) -> SuperResult<Wire> {
    input.trim().split(',')
        .map(parse_cardinal)
        .collect::<SuperResult<Wire>>()
}

#[test]
fn test_parse_wire() {
    assert_eq!(parse_wire(" U7 , R6 , D4 , L4 ").ok(),
        Some(vec![Cardinal::Up(7), Cardinal::Right(6), Cardinal::Down(4), Cardinal::Left(4)]));
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Coordinate (i32, i32);

impl Coordinate {
    fn distance(self) -> u32 {
        self.0.abs() as u32 + self.1.abs() as u32
    }
}

#[test]
fn test_distance() {
    assert_eq!(Coordinate(2, 3).distance(), 5);
    assert_eq!(Coordinate(-1, -2).distance(), 3);
}

impl Add<Cardinal> for Coordinate {
    type Output = Self;
    fn add(self, rhs: Cardinal) -> Self {
        match rhs {
            Cardinal::Up(value) => Coordinate(self.0, self.1 + value as i32),
            Cardinal::Down(value) => Coordinate(self.0, self.1 - value as i32),
            Cardinal::Left(value) => Coordinate(self.0 - value as i32, self.1),
            Cardinal::Right(value) => Coordinate(self.0 + value as i32, self.1),
        }
    }
}

impl AddAssign<Cardinal> for Coordinate {
    fn add_assign(&mut self, rhs: Cardinal) {
        *self = *self + rhs;
    }
}

impl Sub for Coordinate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Coordinate(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[test]
fn test_coordinates() {
    assert_eq!(Coordinate(1, 2) + Cardinal::Up(2), Coordinate(1, 4));
    assert_eq!(Coordinate(-1, 2) + Cardinal::Down(3), Coordinate(-1, -1));
    assert_eq!(Coordinate(1, 2) + Cardinal::Left(2), Coordinate(-1, 2));
    assert_eq!(Coordinate(-1, 2) + Cardinal::Right(3), Coordinate(2, 2));
    assert_eq!(Coordinate(4, 6) - Coordinate(3, 4), Coordinate(1, 2));
}

fn is_contained(p: i32, a: i32, b: i32) -> bool {
    if a < b {
        p >= a && p <= b
    } else {
        p >= b && p <= a
    }
}

fn intersect_lines(origin1: Coordinate, cardinal1: Cardinal, origin2: Coordinate, cardinal2: Cardinal) -> Option<Coordinate> {
    let end1 = origin1 + cardinal1;
    let end2 = origin2 + cardinal2;
    if cardinal1.is_horizontal() == cardinal2.is_horizontal() {
        // Both lines horizontal or both vertical; possibly match at endpoints, but ignore overlap
        if origin1 == origin2 {
            Some(origin1)
        } else if origin1 + cardinal1 == origin2 {
            Some(origin2)
        } else if origin1 == origin2 + cardinal2 {
            Some(origin1)
        } else if origin1 + cardinal1 == origin2 + cardinal2 {
            Some(origin1 + cardinal1)
        } else {
            None
        }
    } else if cardinal1.is_horizontal() {
        // First line is horizontal, second line is vertical
        if is_contained(origin2.0, origin1.0, end1.0) && is_contained(origin1.1, origin2.1, end2.1) {
            Some(Coordinate(origin2.0, origin1.1))
        } else {
            None
        }
    } else {
        // First line is vertical, second line is horizontal
        if is_contained(origin1.0, origin2.0, end2.0) && is_contained(origin2.1, origin1.1, end1.1) {
            Some(Coordinate(origin1.0, origin2.1))
        } else {
            None
        }
    }
}

#[test]
fn test_intersect_lines() {
    assert_eq!(intersect_lines(Coordinate(2, 0), Cardinal::Up(4), Coordinate(0, 2), Cardinal::Right(4)), Some(Coordinate(2, 2)));
    assert_eq!(intersect_lines(Coordinate(4, 2), Cardinal::Left(4), Coordinate(2, 4), Cardinal::Down(4)), Some(Coordinate(2, 2)));
    assert_eq!(intersect_lines(Coordinate(0, 2), Cardinal::Right(2), Coordinate(4, 2), Cardinal::Left(2)), Some(Coordinate(2, 2)));
    assert_eq!(intersect_lines(Coordinate(0, 2), Cardinal::Right(3), Coordinate(4, 0), Cardinal::Up(4)), None);
}

fn find_intersection(first: WireSlice, second: WireSlice) -> Option<(u32, u32)> {
    let mut closest: Option<Coordinate> = None;
    let mut shortest: Option<u32> = None;

    let mut origin1 = Coordinate(0, 0);
    let mut distance1 = 0;
    for &cardinal1 in first {
        let mut origin2 = Coordinate(0, 0);
        let mut distance2 = 0;
        for &cardinal2 in second {
            if let Some(point) = intersect_lines(origin1, cardinal1, origin2, cardinal2) {
                // Ignore intersections at the origin
                if point != Coordinate(0, 0) {
                    closest = Some(match closest {
                        Some(old) => if point.distance() < old.distance() { point } else { old },
                        None => point,
                    });
                    // Distance to start of each segment PLUS distance from each segment to intersection
                    let distance = distance1 + distance2 + (point - origin1).distance() + (point - origin2).distance();
                    shortest = Some(match shortest {
                        Some(old) => if distance < old { distance } else { old },
                        None => distance,
                    });
                }
            }
            origin2 += cardinal2;
            distance2 += cardinal2.distance();
        }
        origin1 += cardinal1;
        distance1 += cardinal1.distance();
    }

    closest.map(|c| (Coordinate::distance(c), shortest.unwrap()))
}

#[cfg(test)]
fn find_intersection_helper(first: &str, second: &str, result: (u32, u32)) {
    let first = parse_wire(first).unwrap();
    let second = parse_wire(second).unwrap();
    assert_eq!(find_intersection(&first, &second), Some(result));
}

#[test]
fn test_find_intersection() {
    find_intersection_helper(
        "R8,U5,L5,D3",
        "U7,R6,D4,L4",
        (6, 30));

    find_intersection_helper(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83",
        (159, 610));

    find_intersection_helper(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        (135, 410));
}

fn main() -> SuperResult<()> {
    let reader = {
        let mut args = env::args();
        let name: Cow<'static, str> = args.nth(1).map(|s| s.into()).unwrap_or_else(|| "input".into());
        let file = File::open(name.as_ref())?;
        BufReader::new(file)
    };

    // Collect lines of input into Vec<Vec<Cardinal>>
    let wires = reader.lines()
        .map(|s| parse_wire(&s?))
        .collect::<SuperResult<Vec<Wire>>>()?;

    println!("{:?}", find_intersection(&wires[0], &wires[1]));

    Ok(())
}
