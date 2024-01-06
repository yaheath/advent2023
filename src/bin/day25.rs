use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use ya_advent_lib::read::read_input;

#[derive(Clone)]
struct Input {
    comp: String,
    connections: HashSet<String>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (comp, l) = s.split_once(": ").unwrap();
        let connections = HashSet::from_iter(l.split(' ').map(|ss| ss.into()));
        Ok(Input{comp: comp.into(), connections})
    }
}

#[allow(dead_code)]
fn dot(input: &[Input]) -> std::io::Result<()> {
    let mut file = File::create("day25.dot")?;
    write!(file, "graph G {{")?;
    for i in input {
        for j in &i.connections {
            write!(file, " {} -- {};", i.comp, j)?;
        }
    }
    write!(file, "}}")?;
    file.sync_all()?;
    Ok(())
}

#[derive(Clone)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }
    fn insert_edge(&mut self, a: &str, b: &str) {
        self.nodes.insert(a.to_owned());
        self.nodes.insert(b.to_owned());
        self.edges.entry(a.to_owned())
            .and_modify(|e| {(*e).insert(b.to_owned());})
            .or_insert(HashSet::from_iter([b.to_owned()]));
        self.edges.entry(b.to_owned())
            .and_modify(|e| {(*e).insert(a.to_owned());})
            .or_insert(HashSet::from_iter([a.to_owned()]));
    }

    fn remove_edge(&mut self, a: &str, b: &str) {
        self.edges.get_mut(a).unwrap().remove(b);
        self.edges.get_mut(b).unwrap().remove(a);
    }

    fn count_nodes(&self, anchor: &str) -> usize {
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: Vec<&String> = Vec::new();
        let anchor = anchor.to_owned();
        queue.push(&anchor);
        while let Some(node) = queue.pop() {
            visited.insert(node.clone());
            self.edges[node].iter().for_each(|n| {
                if !visited.contains(n) {
                    queue.push(n);
                }
            });
        }
        visited.len()
    }

    fn path_between(&self, a: &str, b: &str) -> Option<Vec<String>> {
        let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();
        let mut traversed: HashSet<String> = HashSet::new();
        queue.push_back((a.to_owned(), Vec::new()));
        while let Some((node, path)) = queue.pop_front() {
            traversed.insert(node.clone());
            let mut path = path.clone();
            path.push(node.clone());
            if node == b {
                return Some(path);
            }
            self.edges[&node].iter()
                .filter(|n| !traversed.contains(*n))
                .for_each(|n| queue.push_back((n.clone(), path.clone())));
        }
        None
    }
}

fn part1(input: &[Input]) -> usize {
    // let _ = dot(input);
    let mut graph = Graph::new();
    for i in input.iter() {
        for j in &i.connections {
            graph.insert_edge(&i.comp, j);
        }
    }
    let mut seen_edges: HashMap<(String,String),usize> = HashMap::new();
    loop {
        for _ in 0..20 {
            let nodes = graph.nodes.iter().choose_multiple(&mut rand::thread_rng(), 2);
            if let Some(path) = graph.path_between(nodes[0], nodes[1]) {
                path
                .iter()
                .tuple_windows()
                .for_each(|(a, b)| {
                    let key = (a.min(b).clone(), a.max(b).clone());
                    seen_edges.entry(key).and_modify(|n| *n += 1).or_insert(1);
                });
            }
        }
        let rem = seen_edges.iter()
            .sorted_by_key(|(_, n)| Reverse(*n))
            .map(|(k, _)| k)
            .take(3)
            .collect::<Vec<_>>();

        rem.iter().for_each(|(a, b)| graph.remove_edge(a, b));
        let n1 = graph.count_nodes(&rem[0].0);
        let n2 = graph.count_nodes(&rem[0].1);
        if n1 + n2 == graph.nodes.len() {
            return n1 * n2;
        }
        rem.iter().for_each(|(a, b)| graph.insert_edge(a, b));
    }
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day25_test() {
        let input: Vec<Input> = test_input(include_str!("day25.testinput"));
        assert_eq!(part1(&input), 54);
    }
}
