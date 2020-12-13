use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{collections::HashSet, fmt::Debug};

use anyhow::{anyhow, bail, Result};

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(isize),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..3] {
            "nop" => Ok(Instruction::Nop),
            "acc" => Ok(Instruction::Acc(s[3..].trim().parse()?)),
            "jmp" => Ok(Instruction::Jmp(s[3..].trim().parse()?)),
            token => Err(anyhow!("Unknown instruction {:#?}", token)),
        }
    }
}

fn solve<P: AsRef<Path>>(path: P) -> Result<i32> {
    let code: Vec<_> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .collect();
    let mut acc = 0;
    let mut ip = 0isize;
    let mut executed = HashSet::with_capacity(code.len());
    loop {
        executed.insert(ip);
        match code.get(ip as usize) {
            Some(&Instruction::Nop) => {
                ip += 1;
            }
            Some(&Instruction::Acc(a)) => {
                acc += a;
                ip += 1;
            }
            Some(&Instruction::Jmp(offset)) => {
                ip += offset;
            }
            None => {
                return Err(anyhow!("ip out of bounds ({} >= {})", ip, code.len()));
            }
        }
        if executed.contains(&ip) {
            return Ok(acc);
        }
    }
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day08/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("acc = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
