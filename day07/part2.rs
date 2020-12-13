use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};
use regex::Regex;

#[derive(Debug)]
struct BagVertex<K> {
    containable: Vec<(usize, K)>,
}

impl<K> BagVertex<K> {
    fn new(containable: Vec<(usize, K)>) -> Self {
        BagVertex { containable }
    }
}

#[derive(Debug)]
struct BagGraph<K> {
    bags: HashMap<K, BagVertex<K>>,
}

impl<K: Eq + Hash + Clone + Debug> BagGraph<K> {
    #[inline]
    fn new<II>(iterable: II) -> Self
    where
        II: IntoIterator<Item = (K, Vec<(usize, K)>)>,
    {
        let mut graph = HashMap::new();
        iterable.into_iter().for_each(|(subject_bag, object_bags)| {
            graph.insert(subject_bag, BagVertex::new(object_bags));
        });
        Self { bags: graph }
    }

    /// Traversing the `BagGraph` in a DFS manner, return the total number of bags that are
    /// recursively contained in bag `curr_key`.
    fn dfs_count_bags(&self, curr_key: &K, content_sum: &mut HashMap<K, usize>) -> usize {
        self.bags
            .get(curr_key)
            .unwrap()
            .containable
            .iter()
            .map(|(quant, child)| {
                quant
                    + quant
                        * if content_sum.contains_key(child) {
                            *content_sum.get(child).unwrap()
                        } else {
                            let count = self.dfs_count_bags(child, content_sum);
                            content_sum.insert(child.clone(), count);
                            count
                        }
            })
            .sum()
    }
}

fn solve<P: AsRef<Path>>(path: P) -> Result<usize> {
    let subject_re = Regex::new(r#"^(?P<color>\w+\s\w+)"#).unwrap();
    let objects_re = Regex::new(r#"\s(?P<quantity>\d+)\s(?P<color>\w+\s\w+)[\s\w]+[,.]"#).unwrap();
    let graph = BagGraph::new(
        BufReader::with_capacity(1 << 14, std::fs::File::open(path)?)
            .lines()
            .map(|line| {
                let line = line.as_ref().unwrap();
                let subject_bag = subject_re
                    .captures(line)
                    .unwrap()
                    .name("color")
                    .unwrap()
                    .as_str();
                let mut object_bags = vec![];
                for caps in objects_re.captures_iter(line) {
                    object_bags.push((caps["quantity"].parse().unwrap(), caps["color"].to_owned()));
                }
                (subject_bag.to_owned(), object_bags)
            }),
    );
    Ok(graph.dfs_count_bags(
        &"shiny gold".to_owned(),
        &mut HashMap::with_capacity(graph.bags.len()),
    ))
}

fn main() -> Result<()> {
    let argv: Vec<_> = std::env::args().collect();
    let filepath = match argv.len() {
        1 => "day07/input.txt",
        2 => argv[1].as_ref(),
        _ => {
            bail!(format!("Usage:\n\t$ {} [<file>]", argv[0]));
        }
    };

    println!("# bags = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
