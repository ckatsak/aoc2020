use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut ratings: Vec<u64> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    ratings.sort_unstable();

    let mut differences = Vec::with_capacity(ratings.len() + 1);
    differences.push(ratings[0]);
    for (i, _) in ratings.iter().enumerate().skip(1) {
        differences.push(ratings[i] - ratings[i - 1]);
    }
    differences.push(3);

    let (mut j1, mut j3) = (0, 0);
    for diff in differences {
        if 1 == diff {
            j1 += 1;
        }
        if 3 == diff {
            j3 += 1;
        }
    }

    Ok(j1 * j3)
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day10/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("prod = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
