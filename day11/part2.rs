use std::convert::TryInto;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, PartialEq)]
enum Seat {
    Empty(usize, usize),
    Occupied(usize, usize),
    Floor,
}

impl std::convert::TryFrom<((usize, usize), char)> for Seat {
    type Error = String;

    fn try_from(((r, c), availability): ((usize, usize), char)) -> Result<Self, Self::Error> {
        match availability {
            'L' => Ok(Seat::Empty(r, c)),
            '#' => Ok(Seat::Occupied(r, c)),
            '.' => Ok(Seat::Floor),
            s => Err(format!("invalid seat: {:?} --> {:#?}", (r, c), s)),
        }
    }
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seat::Empty(_, _) => write!(f, "L"),
            Seat::Occupied(_, _) => write!(f, "#"),
            Seat::Floor => write!(f, "."),
        }
    }
}

impl Seat {
    /// Return 8 Iterators, one per direction, that yield the position (`(usize, usize)`)
    /// of all other seats in the `Layout<dims>` that are "visible" from position `pos`.
    #[inline(always)]
    fn visible_seats_from(
        pos: (usize, usize),
        dims: (usize, usize),
    ) -> Vec<Box<dyn Iterator<Item = (usize, usize)>>> {
        vec![
            Box::new(Seat::north(pos, dims)),
            Box::new(Seat::south(pos, dims)),
            Box::new(Seat::east(pos, dims)),
            Box::new(Seat::west(pos, dims)),
            Box::new(Seat::north_east(pos, dims)),
            Box::new(Seat::north_west(pos, dims)),
            Box::new(Seat::south_east(pos, dims)),
            Box::new(Seat::south_west(pos, dims)),
        ]
    }

    #[inline(always)]
    fn north((r, c): (usize, usize), _: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        (0..r).rev().zip((c..=c).cycle())
    }

    #[inline(always)]
    fn south(
        (r, c): (usize, usize),
        (nr, _): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (r + 1..nr).zip((c..=c).cycle())
    }

    #[inline(always)]
    fn east(
        (r, c): (usize, usize),
        (_, nc): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (r..=r).cycle().zip(c + 1..nc)
    }

    #[inline(always)]
    fn west((r, c): (usize, usize), _: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        (r..=r).cycle().zip((0..c).rev())
    }

    #[inline(always)]
    fn north_east(
        (r, c): (usize, usize),
        (_, nc): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (0..r).rev().zip(c + 1..nc)
    }

    #[inline(always)]
    fn north_west(
        (r, c): (usize, usize),
        _: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (0..r).rev().zip((0..c).rev())
    }

    #[inline(always)]
    fn south_east(
        (r, c): (usize, usize),
        (nr, nc): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (r + 1..nr).zip(c + 1..nc)
    }

    #[inline(always)]
    fn south_west(
        (r, c): (usize, usize),
        (nr, _): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (r + 1..nr).zip((0..c).rev())
    }
}

#[derive(Clone, PartialEq)]
struct Row {
    id: usize,
    seats: Vec<Seat>,
}

impl std::convert::TryFrom<(usize, &str)> for Row {
    type Error = String;

    fn try_from((id, s): (usize, &str)) -> Result<Self, Self::Error> {
        Ok(Row {
            id,
            seats: s
                .chars()
                .enumerate()
                .map(|(col, c)| ((id, col), c).try_into())
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
    fn new<II: IntoIterator<Item = String>>(lines: II) -> Result<Self> {
        Ok(Layout(
            lines
                .into_iter()
                .enumerate()
                .map(|(i, line)| (i, line.as_ref()).try_into())
                .collect::<Result<Vec<Row>, String>>()
                .map_err(|err| anyhow!("parsing input: {}", err))?,
        ))
    }

    fn step(&self) -> Self {
        const EMPTY: usize = 0;
        const CROWDY: usize = 5;

        let order = |pos| {
            Seat::visible_seats_from(pos, (self.0.len(), self.0.get(0).unwrap().seats.len()))
                .into_iter()
                .filter_map(|iter| {
                    for (vr, vc) in iter.into_iter() {
                        match self.0.get(vr).unwrap().seats.get(vc).unwrap() {
                            Seat::Empty(_, _) => {
                                return None;
                            }
                            Seat::Occupied(_, _) => {
                                return Some(());
                            }
                            Seat::Floor => (),
                        }
                    }
                    None
                })
                .count()
        };

        // TODO: Non-optimal: the order of each Seat will be calculated multiple times.
        let mut next = self.clone();
        next.0.iter_mut().for_each(|row| {
            row.seats.iter_mut().for_each(|seat| match seat {
                Seat::Empty(r, c) => {
                    if order((*r, *c)) == EMPTY {
                        *seat = Seat::Occupied(*r, *c)
                    }
                }
                Seat::Occupied(r, c) => {
                    if order((*r, *c)) >= CROWDY {
                        *seat = Seat::Empty(*r, *c)
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
                    .filter(|&seat| matches!(seat, Seat::Occupied(_, _)))
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
    let mut layout = Layout::new(input)?;
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
mod tests {
    use super::*;

    #[test]
    fn north() {
        assert_eq!(Seat::north((1, 1), (3, 3)).collect::<Vec<_>>(), &[(0, 1)]);
        assert_eq!(
            Seat::north((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(1, 2), (0, 2)]
        );
        assert_eq!(Seat::north((0, 1), (3, 3)).collect::<Vec<_>>(), &[]);
    }

    #[test]
    fn south() {
        assert_eq!(Seat::south((1, 1), (3, 3)).collect::<Vec<_>>(), &[(2, 1)]);
        assert_eq!(
            Seat::south((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(3, 2), (4, 2)]
        );
        assert_eq!(Seat::south((2, 2), (3, 3)).collect::<Vec<_>>(), &[]);
    }

    #[test]
    fn east() {
        assert_eq!(Seat::east((1, 1), (3, 3)).collect::<Vec<_>>(), &[(1, 2)]);
        assert_eq!(
            Seat::east((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(2, 3), (2, 4)]
        );
        assert_eq!(Seat::east((0, 2), (3, 3)).collect::<Vec<_>>(), &[]);
    }

    #[test]
    fn west() {
        assert_eq!(Seat::west((1, 1), (3, 3)).collect::<Vec<_>>(), &[(1, 0)]);
        assert_eq!(
            Seat::west((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(2, 1), (2, 0)]
        );
        assert_eq!(Seat::west((2, 0), (3, 3)).collect::<Vec<_>>(), &[]);
    }

    #[test]
    fn north_east() {
        assert_eq!(
            Seat::north_east((1, 1), (3, 3)).collect::<Vec<_>>(),
            &[(0, 2)]
        );
        assert_eq!(
            Seat::north_east((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(1, 3), (0, 4)]
        );
        assert_eq!(
            Seat::north_east((2, 1), (3, 3)).collect::<Vec<_>>(),
            &[(1, 2)]
        );
    }

    #[test]
    fn north_west() {
        assert_eq!(
            Seat::north_west((1, 1), (3, 3)).collect::<Vec<_>>(),
            &[(0, 0)]
        );
        assert_eq!(
            Seat::north_west((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(1, 1), (0, 0)]
        );
        assert_eq!(
            Seat::north_west((1, 2), (3, 3)).collect::<Vec<_>>(),
            &[(0, 1)]
        );
    }

    #[test]
    fn south_east() {
        assert_eq!(
            Seat::south_east((1, 1), (3, 3)).collect::<Vec<_>>(),
            &[(2, 2)]
        );
        assert_eq!(
            Seat::south_east((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(3, 3), (4, 4)]
        );
        assert_eq!(
            Seat::south_east((0, 1), (3, 3)).collect::<Vec<_>>(),
            &[(1, 2)]
        );
    }

    #[test]
    fn south_west() {
        assert_eq!(
            Seat::south_west((1, 1), (3, 3)).collect::<Vec<_>>(),
            &[(2, 0)]
        );
        assert_eq!(
            Seat::south_west((2, 2), (5, 5)).collect::<Vec<_>>(),
            &[(3, 1), (4, 0)]
        );
        assert_eq!(
            Seat::south_west((1, 2), (3, 3)).collect::<Vec<_>>(),
            &[(2, 1)]
        );
    }
}
