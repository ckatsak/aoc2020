use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use aoc2020::read_u64s;

/// Given a vector of `u64` integers and a `u64` target integer, this function calculates a
/// solution to the 3SUM problem and returns the indices of the resulting integers in the vector.
///
/// An Error is returned if a solution for the 3SUM does not exist.
fn three_sum(expenses: &[u64], target: u64) -> Result<(usize, usize, usize)> {
    let mut h: HashMap<u64, usize> = HashMap::with_capacity(expenses.len() * expenses.len());
    expenses.iter().enumerate().for_each(|(i, e)| {
        h.insert(*e, i);
    });
    for (i, ei) in expenses.iter().enumerate() {
        for (j, ej) in expenses.iter().enumerate() {
            let t_ei_ej = if let Some(t_ei) = target.checked_sub(*ei) {
                if let Some(t_ei_ej) = t_ei.checked_sub(*ej) {
                    t_ei_ej
                } else {
                    continue;
                }
            } else {
                continue;
            };
            if let Some(&k) = h.get(&t_ei_ej) {
                return Ok((i, j, k));
            }
        }
    }
    Err(anyhow!("No solution for 3SUM exists"))
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

    let expenses = read_u64s(file)?;
    let (i, j, k) = three_sum(&expenses, target)?;
    print_result(expenses[i], expenses[j], expenses[k]);
    Ok(())
}

#[inline(always)]
fn print_result(x: u64, y: u64, z: u64) {
    println!("{} * {} * {} = {}", x, y, z, x * y * z);
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn t1() -> Result<()> {
        let v = vec![10, 10, 10, 10, 10, 10, 10];
        let (i, j, k) = three_sum(&v, 30)?;
        print_result(v[i], v[j], v[k]);
        assert_eq!(v[i] + v[j] + v[k], 30);
        Ok(())
    }
}
