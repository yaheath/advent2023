use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::infinite_grid::InfiniteGrid;
use ya_advent_lib::read::read_input;

struct Input {
    dir: CDir,
    steps: usize,
    color: u32,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\w) (\d+) .#(\w+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let dir = match caps.get(1).unwrap().as_str().chars().next().unwrap() {
                'U' => CDir::N,
                'D' => CDir::S,
                'L' => CDir::W,
                'R' => CDir::E,
                _ => panic!(),
            };
            let steps:usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let color:u32 = u32::from_str_radix(caps.get(3).unwrap().as_str(), 16).unwrap();
            Ok(Input {dir, steps, color})
        }
        else {
            Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Edge(u32),
    Interior,
    Undug,
}

impl From<Cell> for char {
    fn from(v: Cell) -> char {
        match v {
            Cell::Edge(_) => '#',
            Cell::Interior => '*',
            Cell::Undug => '.',
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum EdgeType {
    Vert,
    Horiz,
    SCorner,
    NCorner,
    Other,
}

fn edge_type(grid: &InfiniteGrid<Cell>, c:Coord2D) -> EdgeType {
    let n = matches!(grid.get_c(c + CDir::N), Cell::Edge(_));
    let s = matches!(grid.get_c(c + CDir::S), Cell::Edge(_));
    let e = matches!(grid.get_c(c + CDir::E), Cell::Edge(_));
    let w = matches!(grid.get_c(c + CDir::W), Cell::Edge(_));
    match (n, s, e, w) {
        (true, true, false, false) => EdgeType::Vert,
        (false, false, true, true) => EdgeType::Horiz,
        (true, false, _, _) if e != w => EdgeType::NCorner,
        (false, true, _, _) if e != w => EdgeType::SCorner,
        _ => EdgeType::Other,
    }
}

fn dig(input: &Vec<Input>) -> InfiniteGrid<Cell> {
    let mut grid = InfiniteGrid::new(Cell::Undug);
    let mut current = Coord2D::new(0, 0);
    grid.set_c(current, Cell::Edge(input[0].color));
    for i in input {
        for _ in 0..i.steps {
            current += i.dir;
            grid.set_c(current, Cell::Edge(i.color));
        }
    }
    for y in grid.y_bounds() {
        let mut int = false;
        let mut edge_start: Option<EdgeType> = None;
        for x in grid.x_bounds() {
            let c = grid.get(x, y);
            match (c, int, edge_start) {
                (Cell::Edge(_), false, None) => {
                    match edge_type(&grid, Coord2D::new(x,y)) {
                        EdgeType::Vert => { int = true; },
                        EdgeType::NCorner => { edge_start = Some(EdgeType::NCorner); },
                        EdgeType::SCorner => { edge_start = Some(EdgeType::SCorner); },
                        _ => panic!(),
                    }
                },
                (Cell::Edge(_), true, None) => {
                    match edge_type(&grid, Coord2D::new(x,y)) {
                        EdgeType::Vert => { int = false; },
                        EdgeType::NCorner => { edge_start = Some(EdgeType::NCorner); },
                        EdgeType::SCorner => { edge_start = Some(EdgeType::SCorner); },
                        _ => panic!(),
                    }
                }
                (Cell::Edge(_), _, Some(et)) => {
                    match edge_type(&grid, Coord2D::new(x,y)) {
                        EdgeType::Horiz => {},
                        EdgeType::NCorner => {
                            if et == EdgeType::SCorner {
                                int = !int;
                            }
                            edge_start = None;
                        },
                        EdgeType::SCorner => {
                            if et == EdgeType::NCorner {
                                int = !int;
                            }
                            edge_start = None;
                        },
                        _ => panic!(),
                    }
                },
                (Cell::Undug, false, _) => {},
                (Cell::Undug, true, _) => {
                    grid.set(x, y, Cell::Interior);
                },
                (Cell::Interior, _, _) => {},
            }
        }
    }
    grid
}

fn part1(input: &Vec<Input>) -> usize {
    let grid = dig(input);
    grid.iter().filter(|(_, c)| !matches!(**c, Cell::Undug)).count()
}

fn polygon_from_input(input: &Vec<Input>) -> Vec<Coord2D> {
    let mut poly = Vec::new();
    let mut pos = Coord2D::new(0, 0);
    for i in input {
        let dx = (i.color >> 4) as i64;
        let dir = match i.color & 0xf {
            0 => CDir::E,
            1 => CDir::S,
            2 => CDir::W,
            3 => CDir::N,
            _ => panic!(),
        };
        pos += Into::<Coord2D>::into(dir) * dx;
        poly.push(pos.clone());
    }
    assert_eq!(poly[poly.len() - 1], Coord2D::new(0,0));
    poly
}

fn poly_area(poly: &Vec<Coord2D>) -> i64 {
    let (sum1, sum2) = poly.iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.x * b.y, a.y * b.x))
        .fold((0, 0), |(sum1, sum2), (a, b)| (sum1 + a, sum2 + b));
    let intarea = (sum1 - sum2).abs() / 2;
    let edgearea = poly.iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs())
        .sum::<i64>() / 2 + 1;
    intarea + edgearea
}

fn part2(input: &Vec<Input>) -> i64 {
    let polygon = polygon_from_input(&input);
    poly_area(&polygon)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Input> = test_input(include_str!("day18.testinput"));
        assert_eq!(part1(&input), 62);
        assert_eq!(part2(&input), 952408144115);
    }
}
