use anyhow::{anyhow, bail, Result};

use aoc2020::read_u64s;

/// Given a vector of `u64` integers and a `u64` target integer, this function calculates the 2SUM
/// and returns the indices of the resulting integers in the vector (in their final position; i.e.,
/// after sorting it).
///
/// An Error is returned if a solution for the 2SUM does not exist.
fn two_sum(expenses: &mut Vec<u64>, target: u64) -> Result<(usize, usize)> {
    expenses.sort_unstable();
    let mut last = expenses.binary_search(&target).unwrap_or_else(|i| i);
    for (i, curr) in expenses.iter().enumerate() {
        if last <= i {
            break; // 2SUM solution does not exist
        }
        last = match &expenses[i..last].binary_search(&(target - curr)) {
            Ok(j) => {
                return Ok((i, i + j)); // translate j due to subslicing
            }
            Err(j) => *j,
        }
    }
    Err(anyhow!("No solution for 2SUM exists"))
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let (target, file) = match argv.len() {
        1 => (2020, "day01/part1.txt"),
        3 => (
            argv[1].parse::<u64>().expect("<target> must be a u64"),
            argv[2].as_ref(),
        ),
        _ => {
            bail!(format!("Usage: {} [<target> <file>]", argv[0]));
        }
    };

    let mut expenses = read_u64s(file)?;
    let (i, j) = two_sum(&mut expenses, target)?;
    print_result(expenses[i], expenses[j]);
    Ok(())
}

#[inline(always)]
fn print_result(x: u64, y: u64) {
    println!("{} * {} = {}", x, y, x * y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn t1() -> Result<()> {
        let mut v = vec![10, 10, 10, 10, 10, 10, 10];
        let (i, j) = two_sum(&mut v, 20)?;
        print_result(v[i], v[j]);
        assert_eq!(v[i] + v[j], 20);
        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let mut v = (1..10).collect();
        match two_sum(&mut v, 20) {
            Ok((i, j)) => {
                print_result(v[i], v[j]);
                bail!("This should return an Err!");
            }
            Err(e) => {
                eprintln!("error = {:#?}", e);
                Ok(())
            }
        }
    }
}
