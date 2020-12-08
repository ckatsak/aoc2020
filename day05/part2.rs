use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<u64> {
    let seats: BTreeSet<_> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
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
        .collect();
    let (i, prev_seat) = seats
        .iter()
        .enumerate()
        .skip(1) // skip "very front"
        .find(|&(_, e)| !(seats.contains(&(e - 1)) && seats.contains(&(e + 1))))
        .unwrap();
    assert_ne!(i, seats.len()); // should not be "very back"
    Ok(prev_seat + 1)
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

    println!("my own seat ID = {:#?}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
