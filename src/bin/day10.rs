#[macro_use]
extern crate quick_error;

use std::{env, fs, io};
use std::borrow::Cow;
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
        fs::read_to_string(name.as_ref())?
    };

    let coords = parse_asteroid_map(&input);
    let visible = compute_visibility(&coords);
    println!("Part 1: {}", visible.iter().max().unwrap());

    Ok(())
}

fn compute_visibility(coords: &[(usize, usize)]) -> Vec<usize> {
    let mut visible = vec![0; coords.len()];

    // Iterate over each pair of asteroids
    for (i, &a) in coords.iter().enumerate() {
        'next: for (j, &b) in coords[i+1..].iter().enumerate() {
            // Pre-compute reciprocal of vector length
            let bax = (b.0 as f32 - a.0 as f32).recip();
            let bay = (b.1 as f32 - a.1 as f32).recip();

            // Check all other asteroids for blocking visibility
            for &c in coords[..i].iter().chain(coords[i+1..i+j+1].iter()).chain(coords[i+j+2..].iter()) {
                if is_blocking(c, a, bax, bay) {
                    continue 'next;
                }
            }

            visible[i] += 1;
            visible[i+j+1] += 1;
        }
    }

    visible
}

fn is_blocking(c: (usize, usize), a: (usize, usize), bax: f32, bay: f32) -> bool {
    const EPSILON: f32 = 10. * f32::EPSILON;
    match (bax.is_finite(), bay.is_finite()) {
        (true, true) => { // Diagonal line
            let tx = (c.0 as f32 - a.0 as f32) * bax;
            let ty = (c.1 as f32 - a.1 as f32) * bay;
            (tx - ty).abs() < EPSILON && tx >= 0. && tx <= 1.// && ty >= 0. && ty <= 1.
        }
        (false, true) => { // Vertical line
            let ty = (c.1 as f32 - a.1 as f32) * bay;
            (c.0 as f32 - a.0 as f32).abs() < EPSILON && ty >= 0. && ty <= 1.
        }
        (true, false) => { // Horizontal line
            let tx = (c.0 as f32 - a.0 as f32) * bax;
            (c.1 as f32 - a.1 as f32).abs() < EPSILON && tx >= 0. && tx <= 1.
        }
        (false, false) => false, // Degenerate line
    }
}

#[cfg(test)]
fn test_compute_visibility_helper(map: &str, expected: usize) {
    let coords = parse_asteroid_map(map);
    let visibility = compute_visibility(&coords);
    assert_eq!(visibility.iter().max(), Some(&expected));
}

#[test]
fn test_compute_visibility() {
    test_compute_visibility_helper(".#..#
                                    .....
                                    #####
                                    ....#
                                    ...##", 8);

    test_compute_visibility_helper("......#.#.
                                    #..#.#....
                                    ..#######.
                                    .#.#.###..
                                    .#..#.....
                                    ..#....#.#
                                    #..#....#.
                                    .##.#..###
                                    ##...#..#.
                                    .#....####", 33);

    test_compute_visibility_helper("#.#...#.#.
                                    .###....#.
                                    .#....#...
                                    ##.#.#.#.#
                                    ....#.#.#.
                                    .##..###.#
                                    ..#...##..
                                    ..##....##
                                    ......#...
                                    .####.###.", 35);
}

fn parse_asteroid_map(input: &str) -> Vec<(usize, usize)> {
    input.lines().enumerate() // Iterate over rows
        .flat_map(|(row, text)| text.trim().chars().enumerate() // Iterate over cols
            .filter_map(move |(col, token)| if token == '#' { Some((col, row)) } else { None }))
        .collect()
}

#[test]
fn test_parse_asteroid_map() {
    let map = ".#..#
               .....
               #####
               ....#
               ...##";
    let coords = [(1, 0), (4, 0), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (4, 3), (3, 4), (4, 4)];
    assert_eq!(parse_asteroid_map(&map), &coords);
}
