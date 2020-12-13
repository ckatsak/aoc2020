use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{collections::HashSet, fmt::Debug};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(i32),
    Jmp(isize),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..3] {
            "nop" => Ok(Instruction::Nop(s[3..].trim().parse()?)),
            "acc" => Ok(Instruction::Acc(s[3..].trim().parse()?)),
            "jmp" => Ok(Instruction::Jmp(s[3..].trim().parse()?)),
            token => Err(anyhow!("Unknown instruction {:#?}", token)),
        }
    }
}

#[derive(Clone, Debug)]
struct ProgramState {
    ip: isize,
    executed: HashSet<isize>,
    acc: i32,
}

impl ProgramState {
    fn new() -> Self {
        ProgramState {
            ip: 0,
            executed: HashSet::new(),
            acc: 0,
        }
    }
}

fn run_program(code: &[Instruction], mut state: ProgramState) -> Result<i32> {
    loop {
        state.executed.insert(state.ip);
        match code.get(state.ip as usize) {
            Some(&Instruction::Nop(_)) => {
                state.ip += 1;
            }
            Some(&Instruction::Acc(a)) => {
                state.acc += a;
                state.ip += 1;
            }
            Some(&Instruction::Jmp(offset)) => {
                state.ip += offset;
            }
            None => {
                return Err(anyhow!("ip out of bounds ({} > {})", state.ip, code.len()));
            }
        }
        if state.ip == code.len() as isize {
            return Ok(state.acc);
        }
        if state.executed.contains(&state.ip) {
            return Err(anyhow!("BUGGY"));
        }
    }
}

fn solve<P: AsRef<Path>>(path: P) -> Result<i32> {
    let mut code: Vec<_> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .collect();
    let mut state = ProgramState::new();
    let mut alt_execs = Vec::new();
    loop {
        state.executed.insert(state.ip);
        match code.get(state.ip as usize) {
            Some(&Instruction::Nop(offset)) => {
                alt_execs.push(((state.ip, Instruction::Jmp(offset)), state.clone()));
                state.ip += 1;
            }
            Some(&Instruction::Acc(a)) => {
                state.acc += a;
                state.ip += 1;
            }
            Some(&Instruction::Jmp(offset)) => {
                alt_execs.push(((state.ip, Instruction::Nop(offset)), state.clone()));
                state.ip += offset;
            }
            None => {
                return Err(anyhow!("ip out of bounds ({} > {})", state.ip, code.len()));
            }
        }
        if state.ip == code.len() as isize {
            return Ok(state.acc);
        }
        if state.executed.contains(&state.ip) {
            break;
        }
    }

    for ((pos, alt_instr), state) in alt_execs {
        code.push(alt_instr);
        let orig_instr = code.swap_remove(pos as usize);
        if let Ok(acc) = run_program(&code, state) {
            return Ok(acc);
        }
        code.push(orig_instr);
        code.swap_remove(pos as usize);
    }
    Err(anyhow!("No solution found at all!"))
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
