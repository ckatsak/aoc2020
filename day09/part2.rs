use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, bail, Result};

/// Find a solution for 2SUM in the given `numbers` slice for the given `target`.
/// If no such solution exists, `target` is returned wrapped in an `Err`.
///
/// O(n) amortized
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

/// Find the first number in the given `numbers` slice (after the given `preamble`) which is not a
/// 2SUM solution of its previous `preamble` numbers.
///
/// Returns an error if such a number does not exist.
fn find_invalid(numbers: &[i64], preamble: usize) -> Result<i64> {
    let mut h = HashSet::with_capacity(preamble);
    for (i, num) in numbers[preamble..].iter().enumerate() {
        if let Err(result) = two_sum(&numbers[i..i + preamble], *num, &mut h) {
            return Ok(result);
        }
    }
    Err(anyhow!("No such invalid number exists!"))
}

/// Find the first subslice within the given `numbers` slice which numbers sum to the given
/// `target`.
///
/// Returns an error if such a subslice does not exist.
///
/// O(n)
fn subslice_sum(numbers: &[i64], target: i64) -> Result<&[i64]> {
    assert!(numbers.len() > 1);
    let (mut l, mut r) = (0, 0);
    let mut running_sum = numbers[l];
    while r < numbers.len() || (l + 1 < r && running_sum < target) {
        if r == numbers.len() {
            running_sum -= numbers[l];
            l += 1;
        } else if running_sum < target {
            r += 1;
            running_sum += numbers[r];
        } else {
            running_sum -= numbers[l];
            l += 1;
        }
        if running_sum == target {
            return Ok(&numbers[l..r + 1]);
        }
    }
    Err(anyhow!("No subslice summing to {} exists!", target))
}

fn solve<P: AsRef<Path>>(preamble: usize, path: P) -> Result<i64> {
    let numbers: Vec<_> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let subset = subslice_sum(&numbers, find_invalid(&numbers, preamble)?)?;
    Ok(subset.iter().min().unwrap() + subset.iter().max().unwrap())
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

    println!("weakness = {}", solve(preamble, filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
