use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Result};
use regex::Regex;

#[derive(Debug)]
struct BagVertex<K> {
    containable: Vec<K>,
}

impl<K> BagVertex<K> {
    fn new(containable: Vec<K>) -> Self {
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
        II: IntoIterator<Item = (K, Vec<K>)>,
    {
        let mut graph = HashMap::new();
        iterable.into_iter().for_each(|(subject_bag, object_bags)| {
            graph.insert(subject_bag, BagVertex::new(object_bags));
        });
        Self { bags: graph }
    }

    /// Traversing the `BagGraph` in a DFS manner, return `true` if `target` vertex is reachable
    /// from `start` vertex or `false` otherwise.
    fn dfs_search(&self, start: &K, target: &K) -> bool {
        let mut stack = Vec::with_capacity(self.bags.len());
        stack.push(start);
        let mut visited = HashSet::with_capacity(self.bags.len());
        visited.insert(start);
        while let Some(curr_key) = stack.pop() {
            if curr_key.eq(target) {
                // TODO: Memoize this here and add a check for it right above for O(V + E).
                return true;
            }
            let curr_vertex = self.bags.get(curr_key).unwrap();
            curr_vertex.containable.iter().for_each(|adj_key| {
                if !visited.contains(adj_key) {
                    stack.push(adj_key);
                    visited.insert(adj_key);
                }
            })
        }
        false
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
                    object_bags.push(caps["color"].to_owned());
                }
                (subject_bag.to_owned(), object_bags)
            }),
    );
    Ok(graph
        .bags
        .keys()
        .filter(|bag| graph.dfs_search(bag, &"shiny gold".to_owned()))
        .count()
        - 1) // Subtract the "shiny gold"-->"shiny gold" case that will have been included
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

    println!("bag colors = {}", solve(filepath)?);
    Ok(())
}

#[cfg(test)]
mod tests {}
