#![feature(str_split_once)]

use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let mut passport: HashSet<_> = HashSet::with_capacity(9);
    let validate = |passport: &HashSet<_>| -> bool {
        const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for &field in FIELDS {
            if !passport.contains(field) {
                return false;
            }
        }
        true
    };
    let mut ret = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .filter(|line| {
            let line = line.as_ref().unwrap();
            if line.is_empty() {
                let ret = validate(&passport);
                passport.clear();
                return ret;
            }
            line.split(' ').for_each(|token| {
                passport.insert(token.split_once(':').unwrap().0.to_owned());
            });
            false
        })
        .count();
    // Last passport is never validated, unless the input file ends with double
    // '\n' (in which case case passport has been cleared anyway), so:
    if validate(&passport) {
        ret += 1
    }
    Ok(ret)
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day04/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("# valid passports = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
