use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let mut j = 0;
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .skip(1)
            .filter(|line| {
                let line = line.as_ref().unwrap();
                j += 3;
                line.bytes().nth(j % line.len()).unwrap().eq(&b'#')
            })
            .count(),
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

    println!("#trees = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
