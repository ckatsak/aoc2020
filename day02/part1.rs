#![feature(str_split_once)]

use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    Ok(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .filter(|line| {
                line.as_ref()
                    .unwrap()
                    .split_once(": ")
                    .map(|(policy, password)| (policy.split_once(' ').unwrap(), password))
                    .map(|((range, letter), password)| {
                        (range.split_once('-').unwrap(), letter, password)
                    })
                    .map(|((lower, upper), letter, password)| {
                        (
                            lower.parse::<usize>().unwrap(),
                            upper.parse::<usize>().unwrap(),
                            letter,
                            password,
                        )
                    })
                    .map(|(lower, upper, letter, password)| {
                        let cnt = password.matches(letter).count();
                        cnt >= lower && cnt <= upper
                    })
                    .unwrap()
            })
            .count(),
    )
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day02/part1.txt",
        2 => argv[2].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("#invalid = {}", solve(filepath)?);
    Ok(())
}

//fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
//    Ok(
//        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
//            .lines()
//            .filter_map(|line| -> Option<()> {
//                let line = line.unwrap();
//                if let Some((policy, password)) = line.split_once(": ") {
//                    if let Some((allowed_range, letter)) = policy.split_once(' ') {
//                        if let Some((lower, upper)) = allowed_range.split_once('-').map(|(l, u)| {
//                            (
//                                l.parse::<usize>()
//                                    .expect(&format!("could not parse {:?} as `usize`", l)),
//                                u.parse::<usize>()
//                                    .expect(&format!("could not parse {:?} as `usize`", u)),
//                            )
//                        }) {
//                            let cnt = password.matches(letter).count();
//                            if cnt >= lower && cnt <= upper {
//                                return Some(());
//                            }
//                        }
//                    }
//                }
//                None
//            })
//            .count(),
//    )
//}

#[cfg(test)]
mod tests {}
