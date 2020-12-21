use std::convert::TryInto;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, PartialEq)]
enum Seat {
    Empty(usize, usize, Vec<(usize, usize)>),
    Occupied(usize, usize, Vec<(usize, usize)>),
    Floor,
}

impl std::convert::TryFrom<((usize, usize), char, (usize, usize))> for Seat {
    type Error = String;

    fn try_from(
        (pos, availability, dims): ((usize, usize), char, (usize, usize)),
    ) -> Result<Self, Self::Error> {
        match availability {
            'L' => Ok(Seat::Empty(pos.0, pos.1, Seat::adjacent(pos, dims))),
            '#' => Ok(Seat::Occupied(pos.0, pos.1, Seat::adjacent(pos, dims))),
            '.' => Ok(Seat::Floor),
            c => Err(format!("invalid seat: {:?} --> {:#?}", pos, c)),
        }
    }
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seat::Empty(_, _, _) => write!(f, "L"),
            Seat::Occupied(_, _, _) => write!(f, "#"),
            Seat::Floor => write!(f, "."),
        }
    }
}

impl Seat {
    fn adjacent((r, c): (usize, usize), (nr, nc): (usize, usize)) -> Vec<(usize, usize)> {
        let mut ret = Vec::with_capacity(8);
        //eprint!("{:?} --> ", (r, c));
        if r == 0 {
            ret.push((r + 1, c)); // S
            if c > 0 {
                ret.extend_from_slice(&[(r, c - 1), (r + 1, c - 1)]); // W, SW
            }
            if c < nc - 1 {
                ret.extend_from_slice(&[(r, c + 1), (r + 1, c + 1)]); // E, SE
            }
        } else if r == nr - 1 {
            ret.push((r - 1, c)); // N
            if c > 0 {
                ret.extend_from_slice(&[(r, c - 1), (r - 1, c - 1)]); // W, NW
            }
            if c < nc - 1 {
                ret.extend_from_slice(&[(r, c + 1), (r - 1, c + 1)]); // E, NE
            }
        } else if c == 0 {
            ret.push((r, c + 1)); // E
            if r > 0 {
                ret.extend_from_slice(&[(r - 1, c), (r - 1, c + 1)]); // N, NE
            }
            if r < nr - 1 {
                ret.extend_from_slice(&[(r + 1, c), (r + 1, c + 1)]); // S, SE
            }
        } else if c == nc - 1 {
            ret.push((r, c - 1)); // W
            if r > 0 {
                ret.extend_from_slice(&[(r - 1, c), (r - 1, c - 1)]); // N, NW
            }
            if r < nr - 1 {
                ret.extend_from_slice(&[(r + 1, c), (r + 1, c - 1)]); // S, SW
            }
        } else {
            ret.extend_from_slice(&[
                (r - 1, c),     // N
                (r + 1, c),     // S
                (r, c + 1),     // E
                (r, c - 1),     // W
                (r - 1, c + 1), // NE
                (r - 1, c - 1), // NW
                (r + 1, c + 1), // SE
                (r + 1, c - 1), // SW
            ]);
        }
        //eprintln!("ret = {:?}", ret);
        ret
    }
}

#[derive(Clone, PartialEq)]
struct Row {
    id: usize,
    seats: Vec<Seat>,
}

impl std::convert::TryFrom<((usize, &str), (usize, usize))> for Row {
    type Error = String;

    fn try_from(((id, s), dims): ((usize, &str), (usize, usize))) -> Result<Self, Self::Error> {
        Ok(Row {
            id,
            seats: s
                .chars()
                .enumerate()
                .map(|(col, c)| ((id, col), c, dims).try_into())
                .collect::<Result<Vec<Seat>, String>>()?,
        })
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3}: ", self.id)?;
        for seat in &self.seats {
            write!(f, "{}", seat)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct Layout(Vec<Row>);

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl Layout {
    #[inline(always)]
    fn new<II: IntoIterator<Item = String>>(lines: II, dims: (usize, usize)) -> Result<Self> {
        Ok(Layout(
            lines
                .into_iter()
                .enumerate()
                .map(|(i, line)| ((i, line.as_ref()), dims).try_into())
                .collect::<Result<Vec<Row>, String>>()
                .map_err(|err| anyhow!("parsing input: {}", err))?,
        ))
    }

    #[inline]
    fn step(&self) -> Self {
        const EMPTY_ENOUGH: usize = 0;
        const CROWDY_ENOUGH: usize = 4;

        let order = |adj: &[(usize, usize)]| {
            adj.iter()
                .filter(|(ar, ac)| {
                    matches!(
                        self.0.get(*ar).unwrap().seats.get(*ac).unwrap(),
                        Seat::Occupied(_, _, _)
                    )
                })
                .count()
        };

        // FIXME: O(A*N) && too many allocations happenning here!
        let mut next = self.clone();
        next.0.iter_mut().for_each(|row| {
            row.seats.iter_mut().for_each(|seat| match seat {
                Seat::Empty(r, c, adj) => {
                    if order(&adj) == EMPTY_ENOUGH {
                        *seat = Seat::Occupied(*r, *c, adj.clone())
                    }
                }
                Seat::Occupied(r, c, adj) => {
                    if order(&adj) >= CROWDY_ENOUGH {
                        *seat = Seat::Empty(*r, *c, adj.clone())
                    }
                }
                _floor => (), // skip
            })
        });
        next
    }

    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.seats
                    .iter()
                    .filter(|&seat| matches!(seat, Seat::Occupied(_, _, _)))
                    .count()
            })
            .sum()
    }
}

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let input: Vec<_> = BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let (nr, nc) = (input.len(), input[0].len());
    let mut layout = Layout::new(input, (nr, nc))?;
    //eprintln!("Initial Layout:\n{}", layout);

    loop {
        let next = layout.step();
        //eprintln!("\n\nNext Layout:\n{}", next);
        if next == layout {
            return Ok(layout.count_occupied());
        }
        layout = next;
    }
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day11/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("# occupied = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
