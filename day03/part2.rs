use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<(usize, usize, usize, usize, usize)> {
    let (mut j1, mut j2, mut j3, mut j4, mut j5) = (0, 0, 0, 0, 0);
    let mut i = 1;
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .skip(1)
            .map(|line| {
                let line = line.unwrap();
                j1 += 1;
                j2 += 3;
                j3 += 5;
                j4 += 7;
                if i % 2 == 0 {
                    j5 += 1;
                }
                i += 1;
                (
                    if line.bytes().nth(j1 % line.len()).unwrap().eq(&b'#') {
                        1
                    } else {
                        0
                    },
                    if line.bytes().nth(j2 % line.len()).unwrap().eq(&b'#') {
                        1
                    } else {
                        0
                    },
                    if line.bytes().nth(j3 % line.len()).unwrap().eq(&b'#') {
                        1
                    } else {
                        0
                    },
                    if line.bytes().nth(j4 % line.len()).unwrap().eq(&b'#') {
                        1
                    } else {
                        0
                    },
                    if (i - 1) % 2 == 0 && line.bytes().nth(j5 % line.len()).unwrap().eq(&b'#') {
                        1
                    } else {
                        0
                    },
                )
            })
            .fold(
                (0, 0, 0, 0, 0),
                |(slope1, slope2, slope3, slope4, slope5), (step1, step2, step3, step4, step5)| {
                    (
                        slope1 + step1,
                        slope2 + step2,
                        slope3 + step3,
                        slope4 + step4,
                        slope5 + step5,
                    )
                },
            ),
    )
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day03/part1.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    let slope_results = solve(filepath)?;
    println!("slope results = {:#?}", slope_results);
    println!(
        "final product = {}",
        slope_results.0 * slope_results.1 * slope_results.2 * slope_results.3 * slope_results.4
    );
    Ok(())
}

#[cfg(test)]
mod tests {}
