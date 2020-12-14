use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, bail, Result};

fn two_sum(numbers: &[i64], target: i64, h: &mut HashSet<i64>) -> Result<(), i64> {
    h.clear();
    for num in numbers {
        h.insert(*num);
    }
    for num in numbers {
        if h.contains(&(target - num)) {
            return Ok(());
        }
    }
    Err(target)
}

fn solve<P: AsRef<Path>>(preamble: usize, path: P) -> Result<i64> {
    let numbers: Vec<_> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut h = HashSet::with_capacity(preamble);
    for (i, num) in numbers[preamble..].iter().enumerate() {
        if let Err(result) = two_sum(&numbers[i..i + preamble], *num, &mut h) {
            return Ok(result);
        }
    }
    Err(anyhow!("No such number exists!"))
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let (preamble, filepath) = match argv.len() {
        1 => (25, "day09/input.txt"),
        3 => (argv[1].parse()?, argv[2].as_ref()),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<preamble> <file>]", argv[0]));
        }
    };

    println!("first 'wrong' number = {}", solve(preamble, filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
