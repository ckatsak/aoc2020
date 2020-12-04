use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

/// Read and parse all `u64` integers in the given `Path`.
pub fn read_u64s<P>(path: P) -> Result<Vec<u64>>
where
    P: AsRef<Path>,
{
    BufReader::with_capacity(1 << 14, File::open(path)?)
        .lines()
        .map(|line| {
            line.and_then(|x| {
                x.parse::<u64>()
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
            })
            .map_err(|err| anyhow::Error::from(err))
            //.map_err(|err| err.into())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn read() -> Result<()> {
        read_u64s("day01/part1.txt")?;
        Ok(())
    }
}
