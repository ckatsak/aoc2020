use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn steer_left(&mut self, degrees: isize) {
        for _ in 0..degrees / 90 {
            *self = match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            };
        }
    }
    fn steer_right(&mut self, degrees: isize) {
        for _ in 0..degrees / 90 {
            *self = match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        }
    }
}

#[derive(Debug)]
struct Ferry {
    pos: (isize, isize),
    direction: Direction,
}

impl Ferry {
    fn new(direction: Direction) -> Self {
        Ferry {
            pos: (0, 0),
            direction,
        }
    }

    fn follow(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(y) | Instruction::South(y) => {
                self.pos = (self.pos.0 + y, self.pos.1)
            }
            Instruction::East(x) | Instruction::West(x) => self.pos = (self.pos.0, self.pos.1 + x),
            Instruction::Left(degrees) => {
                self.direction.steer_left(degrees);
            }
            Instruction::Right(degrees) => {
                self.direction.steer_right(degrees);
            }
            Instruction::Forward(by) => match self.direction {
                Direction::North => {
                    self.pos = (self.pos.0 - by, self.pos.1);
                }
                Direction::South => {
                    self.pos = (self.pos.0 + by, self.pos.1);
                }
                Direction::East => {
                    self.pos = (self.pos.0, self.pos.1 + by);
                }
                Direction::West => {
                    self.pos = (self.pos.0, self.pos.1 - by);
                }
            },
        };
    }
}

#[derive(Debug)]
enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let by = s[1..].parse::<isize>()?;
        match &s[..1] {
            "N" => Ok(Instruction::North(-by)),
            "S" => Ok(Instruction::South(by)),
            "E" => Ok(Instruction::East(by)),
            "W" => Ok(Instruction::West(-by)),
            "L" => Ok(Instruction::Left(by)),
            "R" => Ok(Instruction::Right(by)),
            "F" => Ok(Instruction::Forward(by)),
            _ => Err(anyhow!("Cannot parse unknown instruction {:#?}", s)),
        }
    }
}

fn solve<P: AsRef<Path>>(path: P) -> Result<isize> {
    let mut ferry = Ferry::new(Direction::East);
    BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .for_each(|instruction| {
            ferry.follow(instruction);
        });
    Ok(ferry.pos.0.abs() + ferry.pos.1.abs())
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day12/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("Manhattan distance = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
