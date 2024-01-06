use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Plot,
    Rock,
    Start
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Plot,
            '#' => Cell::Rock,
            'S' => Cell::Start,
            _ => panic!(),
        }
    }
}
impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Plot => '.',
            Cell::Rock => '#',
            Cell::Start => 'S',
        }
    }
}

pub fn mkgrid(input: &[String]) -> Grid<Cell> {
    Grid::from_input(input, Cell::Plot, 0)
}

pub fn fill<C>(grid: &Grid<Cell>, start: C, max_steps: usize, expand: bool) -> HashSet<Coord2D>
where C: Into<Coord2D> {
    let start: Coord2D = start.into();
    let x_size = grid.x_bounds().end - grid.x_bounds().start;
    let y_size = grid.y_bounds().end - grid.y_bounds().start;
    let parity = max_steps % 2;
    let mut out = HashSet::new();
    let mut queue:VecDeque<(Coord2D, usize)> = VecDeque::new();
    let mut stepped: HashSet<Coord2D> = HashSet::new();
    queue.push_back((start, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if steps != 0 && (steps == max_steps || steps % 2 == parity) {
            out.insert(pos);
            if steps == max_steps { continue; }
        }
        pos.neighbors4()
            .iter()
            .filter(|p| (expand || grid.contains_coord(**p)) && grid.get(p.x.rem_euclid(x_size), p.y.rem_euclid(y_size)) != Cell::Rock)
            .for_each(|p| {
                if !stepped.contains(p) {
                    queue.push_back((*p, steps + 1));
                    stepped.insert(*p);
                }
            });
    }
    out
}

fn part1(input: &[String]) -> usize {
    let grid = mkgrid(input);
    let (x, y) = grid.find(|c,_,_| c == Cell::Start).unwrap();
    fill(&grid, (x, y), 64, false).len()
}

fn aitken_neville(v0: usize, v1: usize, v2: usize, x: usize) -> usize {
    let mut p = [v0, v1, v2];
    for i in 1..3 {
        for j in 0..3 - i {
            p[j] = p[j] + (x - j) / ((i + j) - j) * (p[j + 1] - p[j]);
        }
    }
    p[0]
}

#[allow(dead_code)]
fn show(grid: &Grid<Cell>, steps: &HashSet<Coord2D>) {
    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            if steps.contains(&Coord2D::new(x, y)) {
                print!("O");
            }
            else {
                let c:char = grid.get(x, y).into();
                print!("{}", c);
            }
        }
        println!();
    }
}

fn bigfill<C>(grid: &Grid<Cell>, start: C, maxsteps: usize) -> usize
where C: Into<Coord2D> + Copy {
    // This works because of properties of the input:
    // size is 131x131, start point is at center (thus the 65),
    // and there's a clear path from the center to each edge,
    // and there are no rocks on the edge.
    let v0 = fill(grid, start, 65, true).len();
    let v1 = fill(grid, start, 65 + 131, true).len();
    let v2 = fill(grid, start, 65 + 131 * 2, true).len();
    println!("{v0} {v1} {v2}");
    aitken_neville(v0, v1, v2, (maxsteps - 65) / 131)
}

fn part2(input: &[String]) -> usize {
    let grid = mkgrid(input);
    let (x, y) = grid.find(|c,_,_| c == Cell::Start).unwrap();
    bigfill(&grid, (x, y), 26501365)
}

#[allow(dead_code)]
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
    fn day21_test() {
        let input: Vec<String> = test_input(include_str!("day21.testinput"));
        let grid = mkgrid(&input);
        let (x, y) = grid.find(|c,_,_| c == Cell::Start).unwrap();
        assert_eq!(fill(&grid, (x, y), 6, false).len(), 16);
        assert_eq!(fill(&grid, (x, y), 10, true).len(), 50);
        assert_eq!(fill(&grid, (x, y), 50, true).len(), 1594);
        assert_eq!(fill(&grid, (x, y), 1000, true).len(), 668697);
        //assert_eq!(bigfill(&grid, Coord2D::new(x, y), 5000), 16733044);
    }
}
