use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let mut group_answered = [0; 26];
    let mut group_cardinality = 0;
    let count_common_and_clear = |group_answered: &mut [usize], group_cardinality: &mut usize| {
        let mut ret = 0;
        for letter in group_answered.iter_mut() {
            if *letter == *group_cardinality {
                ret += 1;
            }
            *letter = 0;
        }
        *group_cardinality = 0;
        ret
    };
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .map(|line| {
                let line = line.as_ref().unwrap();
                if line.is_empty() {
                    count_common_and_clear(&mut group_answered, &mut group_cardinality)
                } else {
                    for c in line.as_bytes().iter() {
                        group_answered[(*c - 97) as usize] += 1;
                    }
                    group_cardinality += 1;
                    0usize // do not pollute the sum of empty lines
                }
            })
            .sum::<usize>()
            + count_common_and_clear(&mut group_answered, &mut group_cardinality),
        // Last group's questions are never counted (unless the input file ends with double
        // '\n', in which case a BUG is introduced right here), hence the final addition.
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
