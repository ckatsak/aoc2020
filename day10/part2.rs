use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

const JUMP_CONSTRAINT: u8 = 3;

fn solve<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut ratings: Vec<u64> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    ratings.sort_unstable();

    let mut memo = std::collections::HashMap::with_capacity(1 + ratings.len());
    memo.insert(0, 1);
    ratings.iter().for_each(|&r| {
        memo.insert(
            r,
            // NOTE: If r < JUMP_CONSTRAINT then ∀i∈[1, r], otherwise ∀i∈[1, JUMP_CONSTRAINT]
            (1..=((JUMP_CONSTRAINT as u64).min(r)))
                .map(|i| memo.get(&(r - i)).unwrap_or(&0))
                .sum(),
        );
    });

    Ok(*memo.get(ratings.last().unwrap()).unwrap())
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

    println!("# arrangements = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
