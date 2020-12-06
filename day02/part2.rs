#![feature(str_split_once)]
#![feature(unchecked_math)]

use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .filter(|line| {
                line.as_ref()
                    .unwrap()
                    .split_once(": ")
                    .map(|(policy, password)| (policy.split_once(' ').unwrap(), password))
                    .map(|((range, letter), password)| {
                        (range.split_once('-').unwrap(), letter, password)
                    })
                    .map(|((lower, upper), letter, password)| {
                        (
                            // SAFETY:
                            // The given bounds are always >1, as explicitly stated in the problem.
                            unsafe { lower.parse::<usize>().unwrap().unchecked_sub(1) },
                            unsafe { upper.parse::<usize>().unwrap().unchecked_sub(1) },
                            letter,
                            password,
                        )
                    })
                    .map(|(lower, upper, letter, password)| {
                        let letter = letter.chars().next().unwrap();
                        password.chars().nth(lower).unwrap().eq(&letter)
                            ^ password.chars().nth(upper).unwrap().eq(&letter)
                    })
                    .unwrap()
            })
            .count(),
    )
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day02/part1.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("#invalid = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
