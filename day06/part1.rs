use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let mut group_answered: HashSet<_> = HashSet::with_capacity(26);
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .map(|line| {
                let line = line.as_ref().unwrap();
                if line.is_empty() {
                    let ret = group_answered.len();
                    group_answered.clear();
                    return ret;
                }
                line.chars().for_each(|q| {
                    group_answered.insert(q);
                });
                0usize // do not pollute the sum of empty lines
            })
            .sum::<usize>()
            + group_answered.len(),
        // Last group's questions are never counted unless the input file ends with double '\n'
        // (in which case the HashSet will have been cleared anyway), hence the final addition.
    )
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day06/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("sum = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
