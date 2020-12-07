#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn fine_validate(passport: &HashMap<String, String>) -> bool {
    let num_in_range = |value: &str, lower, upper| match value.parse::<u32>() {
        Ok(num) if num >= lower && num <= upper => true,
        _ => false,
    };
    const ECLS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let hgt = passport.get("hgt").unwrap();
    let hcl = passport.get("hcl").unwrap();
    let pid = passport.get("pid").unwrap();
    num_in_range(passport.get("byr").unwrap(), 1920, 2002)
        && num_in_range(passport.get("iyr").unwrap(), 2010, 2020)
        && num_in_range(passport.get("eyr").unwrap(), 2020, 2030)
        && ((hgt.ends_with("cm") && num_in_range(&hgt[..hgt.len() - 2], 150, 193))
            || (hgt.ends_with("in") && num_in_range(&hgt[..hgt.len() - 2], 59, 76)))
        && hcl.len() == 7
        && hcl.starts_with('#')
        && hcl[1..]
            .chars()
            .all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
        && ECLS.contains(&passport.get("ecl").unwrap().as_ref())
        && pid.len() == 9
        && num_in_range(pid, 0, 999_999_999)
}

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let mut passport: HashMap<_, _> = HashMap::with_capacity(9);
    let validate = |passport: &HashMap<_, _>| {
        for &field in FIELDS {
            if !passport.contains_key(field) {
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
                let ret = validate(&passport) && fine_validate(&passport);
                passport.clear();
                return ret;
            }
            line.split(' ').for_each(|token| {
                let (k, v) = token.split_once(':').unwrap();
                passport.insert(k.to_owned(), v.to_owned());
            });
            false
        })
        .count();
    // Last passport is never validated, unless the input file ends with double
    // '\n' (in which case the passport will have been cleared anyway), so:
    if validate(&passport) && fine_validate(&passport) {
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
