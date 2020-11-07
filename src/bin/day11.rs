#[macro_use]
extern crate quick_error;

use rayon::prelude::*;

use intcode::Intcode;

use std::borrow::Cow;
use std::collections::HashSet;
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

    println!("Part 1: {:?}", paint_hull(&memory)?);

    Ok(())
}

fn paint_hull(memory: &[isize]) -> SuperResult<usize> {
    let mut program = Intcode::new(&memory);
    program.run()?;
    
    let mut position = (0, 0);
    let mut direction = 0; // The robot starts facing up
    let mut hull = HashSet::new();
    while program.resume(hull.contains(&position) as isize)? {
        let turn = program.output.pop().unwrap(); 
        let color = program.output.pop().unwrap();

        // 0 means black, and 1 means white
        if color == 1 {
            hull.insert(position);
        } else {
            hull.remove(&position);
        }

        // 0 means turn left 90 degrees, and 1 means turn right 90 degrees
        direction = (direction + turn * 2 + 3) % 4;

        // After the robot turns, it should always move forward exactly one panel
        position = match direction {
            0 => (position.0, position.1 - 1), // up
            1 => (position.0 + 1, position.1), // right
            2 => (position.0, position.1 + 1), // down
            3 => (position.0 - 1, position.1), // left
            _ => panic!("Corrupted direction"),
        }
    }

    display_hull(&hull);

    Ok(0)
}

fn display_hull(hull: &HashSet<(isize, isize)>) {
    let (left, top, right, bottom) = compute_bounds(hull);
    let rows = bottom - top + 1;
    let cols = right - left + 1;

    let image = (0..rows).into_par_iter().map(|row| {
        (0..cols).into_par_iter().map(|col| {
            let x = left + col;
            let y = top + row;
            match hull.contains(&(x, y)) {
                true => '🀫',
                false => '🀆',
            }
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    println!("{}", image);
}

fn compute_bounds(hull: &HashSet<(isize, isize)>) -> (isize, isize, isize, isize) {
    let mut left = isize::MAX;
    let mut top = isize::MAX;
    let mut right = isize::MIN;
    let mut bottom = isize::MIN;
    for &(x, y) in hull.iter() {
        if x < left {
            left = x;
        }
        if x > right {
            right = x;
        }
        if y < top {
            top = y;
        }
        if y > bottom {
            bottom = y;
        }
    }
    (left, top, right, bottom)
}
