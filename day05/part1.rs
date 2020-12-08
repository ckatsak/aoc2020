use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<u64> {
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .map(|line| {
                u64::from_str_radix(
                    &line
                        .unwrap()
                        .replace(|c| c == 'F' || c == 'L', "0")
                        .replace(|c| c == 'B' || c == 'R', "1"),
                    2,
                )
                .unwrap()
            })
            .max()
            .unwrap(),
    )
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day05/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("highest seat ID = {:#?}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
