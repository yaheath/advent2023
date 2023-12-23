use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::coords::{Coord2D, CDir};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Wall,
    Open,
    Slope(CDir),
}
impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Open,
            '>' => Cell::Slope(CDir::E),
            '<' => Cell::Slope(CDir::W),
            '^' => Cell::Slope(CDir::N),
            'v' => Cell::Slope(CDir::S),
            _ => panic!(),
        }
    }
}

fn dir_from(a: Coord2D, b: Coord2D) -> CDir {
    for d in [CDir::N, CDir::E, CDir::S, CDir::W] {
        if a + d == b { return d; }
    }
    panic!();
}

struct Graph {
    nodes: HashSet<Coord2D>,
    edges: HashMap<Coord2D, HashSet<Coord2D>>,
    weights: HashMap<(Coord2D, Coord2D), usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
            weights: HashMap::new(),
        }
    }
    fn insert_edge(&mut self, a: Coord2D, b: Coord2D, dx: usize) {
        // println!("edge {a} -> {b} {dx}");
        self.nodes.insert(a);
        self.nodes.insert(b);
        let key = (a, b);
        assert!(!self.weights.contains_key(&key) || self.weights[&key] == dx);
        self.weights.insert(key, dx);
        self.edges.entry(a)
            .and_modify(|v| {(*v).insert(b);})
            .or_insert(HashSet::from_iter([b]));
    }
    fn contains_node(&self, node: Coord2D) -> bool {
        self.nodes.contains(&node)
    }
    fn contains_edge(&self, from: Coord2D, to: Coord2D) -> bool {
        self.edges.get(&from).map_or(false, |h| h.contains(&to))
    }
    fn traverse(&self, from: Coord2D, to: Coord2D) -> usize {
        let traversed = HashSet::from_iter([from]);
        self.traverse_r(from, to, traversed)
    }
    fn traverse_r(&self, from: Coord2D, to: Coord2D, traversed: HashSet<Coord2D>) -> usize {
        assert!(self.nodes.contains(&from));
        assert!(self.nodes.contains(&to));

        let mut ret = 0;
        for next in &self.edges[&from] {
            let key = (from, *next);
            let dx = self.weights[&key];
            if *next == to {
                return dx;
            }
            if !traversed.contains(next) {
                let mut t = traversed.clone();
                t.insert(*next);
                ret = ret.max(dx + self.traverse_r(*next, to, t));
            }
        }
        ret
    }
}

fn build_graph(grid: &Grid<Cell>, start: Coord2D, end: Coord2D, part2: bool) -> Graph {
    let mut queue: VecDeque<(Coord2D, Coord2D, Coord2D, usize)> = VecDeque::new();
    let mut graph = Graph::new();
    queue.push_back((start, start, start, 0));
    while let Some((anchor, from, pos, steps)) = queue.pop_front() {
        if pos == end {
            graph.insert_edge(anchor, pos, steps);
            continue;
        }
        if graph.contains_node(pos) {
            if graph.contains_edge(anchor, pos) {
                continue;
            }
            graph.insert_edge(anchor, pos, steps);
        }
        let nei = pos.neighbors4().into_iter()
            .filter(|c| grid.contains_coord(*c) && *c != anchor && *c != from)
            .filter(|c| match grid.get_c(*c) {
                Cell::Open => true,
                Cell::Slope(_) => true,
                _ => false,
            })
            .collect::<Vec<_>>();
        if nei.len() == 1 {
            queue.push_back((anchor, pos, nei[0], steps + 1));
        }
        else if nei.len() > 1 {
            graph.insert_edge(anchor, pos, steps);
            for n in nei {
                if !part2 {
                    if match grid.get_c(n) {
                        Cell::Open => false,
                        Cell::Slope(d) if dir_from(pos, n) == d => false,
                        _ => true,
                    } { continue; }
                }
                queue.push_back((pos, pos, n, 1));
            }
        }
    }
    graph
}

fn run(input: &Vec<String>, part2: bool) -> usize {
    let grid = Grid::from_input(input, Cell::Wall, 0);
    let start = Coord2D::new(1, 0);
    let end = Coord2D::new(grid.x_bounds().end - 2, grid.y_bounds().end - 1);
    let graph = build_graph(&grid, start, end, part2);
    graph.traverse(start, end)
}

fn part1(input: &Vec<String>) -> usize {
    run(input, false)
}

fn part2(input: &Vec<String>) -> usize {
    run(input, true)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day23_test() {
        let input: Vec<String> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 94);
        assert_eq!(part2(&input), 154);
    }
}
