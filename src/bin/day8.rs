#[macro_use]
extern crate quick_error;

use rayon::prelude::*;

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

    let data = input.trim().chars()
        .map(|c| c.to_digit(10).ok_or_else(||
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid digit {}", c))))
        .collect::<io::Result<Vec<u32>>>()?;

    let rows = 6;
    let cols = 25;
    let pixels = rows * cols;
    let layers = data.len() / pixels;

    let (_, result) = data.par_chunks(pixels)
        .map(|layer| {
            let mut count = [0; 3];
            layer.iter().for_each(|&pixel| count[pixel as usize] += 1);
            (count[0], count[1] * count[2])
        })
        .min_by(|(x, _), (y, _)| x.cmp(&y)).unwrap();

    println!("Part 1: {}", result);

    let image = (0..rows).into_par_iter().map(|row| {
        (0..cols).into_par_iter().map(|col| {
            let i = row * cols + col;
            for j in 0..layers {
                match data[j * pixels + i] {
                    0 => return Ok('🀫'),
                    1 => return Ok('🀆'),
                    _ => continue,
                }
            }
            Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("Invalid pixel ({}, {})", col, row)))
        }).collect::<io::Result<String>>()
    }).collect::<io::Result<Vec<String>>>()?.join("\n");

    println!("Part 2:\n{}", image);

    Ok(())
}
