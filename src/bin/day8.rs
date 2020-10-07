#[macro_use]
extern crate quick_error;

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

    let mut result = 0;
    let mut fewest_zeros = u32::MAX;
    for layer in data.chunks(pixels) {
        let mut count = [0; 3];
        layer.iter().for_each(|&pixel| count[pixel as usize] += 1);
        if count[0] < fewest_zeros {
            fewest_zeros = count[0];
            result = count[1] * count[2];
        }
    }

    println!("Part 1: {}", result);

    let image = (0..rows).map(|row| {
        (0..cols).map(|col| {
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
    }).collect::<io::Result<Vec<String>>>()?;

    println!("Part 2:");
    for row in image {
        println!("{}", row);
    }

    Ok(())
}
